use anyhow;

use chrono::Utc;
use dotenv;
use entities::model::{
    tbl_building, tbl_department, tbl_door, tbl_door_to_request, tbl_keycard, tbl_keycard_history,
    tbl_request, tbl_request_comment, tbl_request_department, tbl_request_entrance,
    tbl_room, tbl_room_department, tbl_user,
};
use fake::faker::address::raw::{BuildingNumber, StreetName, StreetSuffix};

use fake::faker::chrono::en::DateTimeAfter;

use fake::faker::chrono::zh_tw::DateTimeBefore;
use fake::faker::company::en::BsNoun;
use fake::faker::internet::raw::FreeEmail;
use fake::faker::lorem::zh_tw::Sentence;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::{self, Fake};
use rand::rngs::ThreadRng;
use rand::Rng;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait, IntoActiveModel,
};
use tracing::log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let database_url = dotenv::var("DATABASE_URL")?;
    let (database_url, db_name) = migration_helper::split_connection_string(&database_url);
    info!("{}/{}", database_url, db_name);

    let err = migration_helper::drop_database(database_url, db_name).await;
    if let Err(err) = err {
        info!("{}", err.to_string());
    }
    let err = migration_helper::create_database(database_url, db_name).await;
    if let Err(err) = err {
        info!("{}", err.to_string());
    }
    let err = migration_helper::run_migration(database_url, db_name).await;
    if let Err(err) = err {
        info!("{}", err.to_string());
    }
    let reasons = vec![
        "Ich brauche Zutritt, um meine tägliche Arbeit erledigen zu können.",
"                    Ich benötige Zutritt, um an Meetings teilnehmen zu können.",
"Ich brauche Zutritt, um Zugang zu wichtigen Dokumenten und Materialien zu haben.",
"Ich benötige Zutritt, um mich mit Kollegen treffen und Austausch pflegen zu können.",
"Ich brauche Zutritt, um an Fortbildungen teilnehmen zu können.",
"Ich benötige Zutritt, um an Vorlesungen teilnehmen zu können, um mich fortzubilden.",
"Ich brauche Zutritt, um an Veranstaltungen teilnehmen zu können, die für meine Arbeit relevant sind.",
"Ich benötige Zutritt, um an Konferenzen teilnehmen zu können, um meine Kenntnisse zu erweitern.",
"Ich brauche Zutritt, um Zugang zu verschiedenen Einrichtungen und Services zu haben, die für meine Arbeit wichtig sind.",
"Ich benötige Zutritt, um an Diskussionen und Debatten teilnehmen zu können, die für meine Arbeit von Bedeutung sind.",
"Ich brauche Zutritt, um an Projektmeetings teilnehmen zu können.",
"Ich benötige Zutritt, um meine Arbeitsmaterialien und -ausrüstung zugänglich zu haben.",
"Ich brauche Zutritt, um an Schulungen und Workshops teilnehmen zu können.",
"Ich benötige Zutritt, um an Vorstandssitzungen teilnehmen zu können.",
"Ich brauche Zutritt, um an Research-Meetings teilnehmen zu können.",
"Ich benötige Zutritt, um an Kontrollbesuchen teilnehmen zu können.",
"Ich benötige Zutritt, um an Supervisionen teilnehmen zu können"
    ];
    let db = Database::connect(format!("{}/{}", database_url, db_name)).await?;
    let mut rng = rand::thread_rng();

    let password = orion::pwhash::Password::from_slice(b"1234").unwrap();
    let hash = orion::pwhash::hash_password(&password, 3, 1 << 16).unwrap();
    //user generation
    for idx in 1..400 {
        

        let name: String = Name(EN).fake_with_rng(&mut rng);
        let email: String = FreeEmail(EN).fake_with_rng(&mut rng);

        tbl_user::ActiveModel {
            email: ActiveValue::Set(email),
            name: ActiveValue::Set(name),
            password: ActiveValue::Set(hash.unprotected_as_encoded().to_string()),
            role_id: ActiveValue::Set(Some(idx)),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }

    let users = tbl_user::Entity::find().all(&db).await?;
    for _ in 0..4 {
        let suffix: String = StreetSuffix(EN).fake_with_rng(&mut rng);
        let name: String = StreetName(EN).fake_with_rng(&mut rng);
        let number: String = BuildingNumber(EN).fake_with_rng(&mut rng);
        let name: String = format!("{} {}. {}", name, suffix, number);
        tbl_building::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let buildings = tbl_building::Entity::find().all(&db).await?;
    for _ in 0..100 {
        let floor = rng.gen_range(0..5);
        let room_number = rng.gen_range(1..20);
        let mut room_prefix = "S";
        if rng.gen_bool(0.2) {
            room_prefix = "HS";
        }
        let name = format!("{}{}{:02}", room_prefix, &floor, &room_number);
        tbl_room::ActiveModel {
            building_id: ActiveValue::Set(buildings[rng.gen_range(0..buildings.len())].building_id),
            floor: ActiveValue::Set(floor),
            is_sensitive: ActiveValue::Set(Some(rng.gen_ratio(1, 8))),
            name: ActiveValue::Set(name),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let rooms = tbl_room::Entity::find().all(&db).await?;
    for room in &rooms {
        tbl_door::ActiveModel {
            name: ActiveValue::Set(BsNoun().fake_with_rng(&mut rng)),
            room_id: ActiveValue::Set(room.room_id),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    for _ in 0..200 {
        tbl_door::ActiveModel {
            name: ActiveValue::Set(BsNoun().fake_with_rng(&mut rng)),
            room_id: ActiveValue::Set(rooms[rng.gen_range(0..rooms.len())].room_id.to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let doors = tbl_door::Entity::find().all(&db).await?;
    let department_names = vec![
        "Hausmeister",
        "Winfo",
        "ITSec",
        "BWL",
        "KI",
        "Verwaltung",
        "Küche",
    ];
    for name in department_names {
        let res = tbl_department::ActiveModel {
            description: ActiveValue::Set(Some(name.to_owned())),
            name: ActiveValue::Set(name.to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await?;
        for _ in 0..rng.gen_range(5..100) {
            let _ = tbl_room_department::ActiveModel {
                department_id: ActiveValue::Set(res.department_id),
                room_id: ActiveValue::Set(rooms[rng.gen_range(0..rooms.len())].room_id.to_owned()),
            }
            .insert(&db)
            .await;
        }
    }
    let departments = tbl_department::Entity::find().all(&db).await?;
    //request that are in step 1

    for _ in 0..100 {
        let active_until: DateTimeUtc =
            DateTimeAfter(chrono::offset::Utc::now()).fake_with_rng(&mut rng);
        let changed_at: DateTimeUtc =
            DateTimeBefore(chrono::offset::Utc::now()).fake_with_rng(&mut rng);
        let created_at: DateTimeUtc = DateTimeBefore(changed_at).fake_with_rng(&mut rng);
        let request = tbl_request::ActiveModel {
            active_until: ActiveValue::Set(Some(active_until.naive_utc())),
            changed_at: ActiveValue::Set(changed_at.naive_utc()),
            created_at: ActiveValue::Set(created_at.naive_utc()),
            is_proposal: ActiveValue::Set(true),
            requester_id: ActiveValue::Set(users[rng.gen_range(0..users.len())].user_id.to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await?;
        // decide which type a request is
        let type_of_request = rng.gen_range(1..=3);

        async fn gen_keycard_proposal(
            request: &tbl_request::Model,
            db: &DatabaseConnection,
        ) -> anyhow::Result<()> {
            // keycard
            let keycard = tbl_keycard::ActiveModel {
                request_id: ActiveValue::Set(Some(request.request_id.to_owned())),
                ..Default::default()
            }
            .insert(db)
            .await?;
            let mut request = request.clone().into_active_model();
            request.keycard_id = ActiveValue::Set(Some(keycard.keycard_id.to_owned()));
            request.update(db).await?;
            Ok(())
        }
        async fn gen_access_proposal(
            request: &tbl_request::Model,
            db: &DatabaseConnection,
            rng: &mut ThreadRng,
            departments: &Vec<tbl_department::Model>,
            buildings: &Vec<tbl_building::Model>,
            reasons: &Vec<&str>,
        ) -> anyhow::Result<()> {
            for _ in 0..rng.gen_range(1..3) {
                let _ = tbl_request_department::ActiveModel {
                    department_id: ActiveValue::Set(
                        departments[rng.gen_range(0..departments.len())]
                            .department_id
                            .to_owned(),
                    ),
                    request_id: ActiveValue::Set(request.request_id.to_owned()),
                }
                .insert(db)
                .await;
            }

            if rng.gen_bool(0.3) {
                tbl_request_entrance::ActiveModel {
                    building_id: ActiveValue::Set(
                        buildings[rng.gen_range(0..buildings.len())]
                            .building_id
                            .to_owned(),
                    ),
                    request_id: ActiveValue::Set(request.request_id.to_owned()),
                    proposed_rooms: ActiveValue::Set("H230".to_string()),
                    ..Default::default()
                }
                .insert(db)
                .await?;
            }

            let mut request = request.clone().into_active_model();

            request.description =
                ActiveValue::Set(Some(reasons[rng.gen_range(0..reasons.len())].to_owned()));
            request.update(db).await?;
            Ok(())
        }
        match type_of_request {
            1 => {
                gen_keycard_proposal(&request, &db).await?;
            }
            2 => {
                gen_access_proposal(&request, &db, &mut rng, &departments, &buildings, &reasons)
                    .await?;
            }
            3 => {
                gen_keycard_proposal(&request, &db).await?;
                gen_access_proposal(&request, &db, &mut rng, &departments, &buildings, &reasons)
                    .await?;
            }
            _ => {
                panic!()
            }
        };
    }
    // 80 of 100 get to the next stage
    let requests = tbl_request::Entity::find().all(&db).await?;
    for idx in 0..80 {
        let request = &requests[idx];
        let mut request_active_model = request.clone().into_active_model();
        request_active_model.is_proposal = ActiveValue::Set(false);
        request_active_model.update(&db).await?;
        for _ in 0..rng.gen_range(1..10) {
            let _ = tbl_door_to_request::ActiveModel {
                door_id: ActiveValue::Set(doors[rng.gen_range(0..doors.len())].door_id.to_owned()),
                request_id: ActiveValue::Set(request.request_id.to_owned()),
            }
            .insert(&db)
            .await;
        }
    }
    let requests = tbl_request::Entity::find().all(&db).await?;
    // requests can now be finalised

    //comments between workers and user
    for _ in 0..500 {
        let request = &requests[rng.gen_range(0..requests.len())];
        let user_who_writes_comment;
        //get the user or a random worker
        if rng.gen_bool(0.5) {
            let user = request.requester_id.to_owned();
            user_who_writes_comment = Some(user);
        } else {
            let workers: Vec<_> = users
                .iter()
                .filter(|f| {
                    f.role_id < Some(4)
                })
                .collect();
            let worker = workers[rng.gen_range(0..workers.len())].user_id.to_owned();
            user_who_writes_comment = Some(worker)
        }
        let written_at: DateTimeUtc =
            DateTimeAfter(DateTimeUtc::from_utc(request.created_at, Utc)).fake_with_rng(&mut rng);
        if let Some(commenter) = user_who_writes_comment {
            tbl_request_comment::ActiveModel {
                request_id: ActiveValue::Set(request.request_id.to_owned()),
                user_id: ActiveValue::Set(commenter),
                comment: ActiveValue::Set(Sentence(1..20).fake_with_rng(&mut rng)),
                written_at: ActiveValue::Set(written_at.naive_utc()),
                ..Default::default()
            }
            .insert(&db)
            .await?;
            //update changed_at in request
            let mut request = request.clone().into_active_model();
            request.changed_at = ActiveValue::Set(written_at.naive_utc());
            request.update(&db).await?;
        }
    }
    for x in requests.iter().filter(|f| !f.is_proposal) {
        let accept = rng.gen_bool(0.2);
        let reject = rng.gen_bool(0.2);
        if accept && !reject || reject && !accept {
            let mut request = x.clone().into_active_model();
            request.accept = ActiveValue::Set(accept);
            request.reject = ActiveValue::Set(reject);
            request.pending = ActiveValue::Set(false);
            request.update(&db).await?;
        }
    }
    let requests = tbl_request::Entity::find().all(&db).await?;
    let keycards = tbl_keycard::Entity::find().all(&db).await?;
    for _ in 0..1000 {
        let requests: Vec<_> = requests.iter().filter(|f| f.accept).collect();
        let keycards: Vec<_> = keycards
            .iter()
            .filter(|f| requests.iter().any(|req| Some(req.request_id )== f.request_id))
            .collect();
        let keycard = &keycards[rng.gen_range(0..keycards.len())];
        let keycard_request = requests
            .iter()
            .find(|f| Some(f.request_id) == keycard.request_id)
            .unwrap();
        let door = &doors[rng.gen_range(0..doors.len())];
        let mut used_at: chrono::DateTime<chrono::Utc> =
            DateTimeBefore(chrono::Utc::now()).fake_with_rng(&mut rng);
        // active_until smaller than now time
        // to prevent assigning historie values into the future
        if keycard_request.changed_at < used_at.naive_utc() {
            used_at = DateTimeBefore(DateTimeUtc::from_utc(keycard_request.changed_at, Utc))
                .fake_with_rng(&mut rng);
        }
        tbl_keycard_history::ActiveModel {
            keycard_id: ActiveValue::Set(keycard.keycard_id.to_owned()),
            door_id: ActiveValue::Set(door.door_id.to_owned()),
            used_at: ActiveValue::Set(used_at.naive_utc()),
            success: ActiveValue::Set(rng.gen_bool(0.6)),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    Ok(())
}
