use anyhow;

use chrono::{NaiveDateTime, Utc};
use dotenv;
use entities::model::{
    tbl_building, tbl_door, tbl_door_group, tbl_door_request, tbl_door_to_group_door,
    tbl_door_user_access, tbl_keycard, tbl_keycard_history, tbl_leader, tbl_request_base,
    tbl_request_comment, tbl_role, tbl_room, tbl_user, tbl_worker,
};
use fake::faker::address::raw::{BuildingNumber, StreetName, StreetSuffix};

use fake::faker::chrono::en::DateTimeAfter;

use fake::faker::chrono::zh_tw::DateTimeBefore;
use fake::faker::company::en::BsNoun;
use fake::faker::company::raw::{Industry, Profession};
use fake::faker::internet::raw::FreeEmail;
use fake::faker::lorem::zh_tw::Sentence;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::{self, Fake};
use rand::Rng;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, EntityTrait, IntoActiveModel};
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

    let db = Database::connect(format!("{}/{}", database_url, db_name)).await?;
    let mut rng = rand::thread_rng();
    let profession: String = Profession(EN).fake_with_rng(&mut rng);
    tbl_role::ActiveModel {
        name: ActiveValue::Set(profession),
        ..Default::default()
    }
    .insert(&db)
    .await?;
    tbl_role::ActiveModel {
        name: ActiveValue::Set("administrative staff".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await?;
    let profession: String = Profession(EN).fake_with_rng(&mut rng);
    tbl_role::ActiveModel {
        name: ActiveValue::Set(profession),
        ..Default::default()
    }
    .insert(&db)
    .await?;
    let profession: String = Profession(EN).fake_with_rng(&mut rng);
    tbl_role::ActiveModel {
        name: ActiveValue::Set(profession),
        ..Default::default()
    }
    .insert(&db)
    .await?;
    let roles = tbl_role::Entity::find().all(&db).await?;
    let password = orion::pwhash::Password::from_slice(b"1234").unwrap();
    let hash = orion::pwhash::hash_password(&password, 3, 1 << 16).unwrap();
    //user generation
    for _ in 0..400 {
        let role = &roles[rng.gen_range(0..roles.len())];

        let name: String = Name(EN).fake_with_rng(&mut rng);
        let email: String = FreeEmail(EN).fake_with_rng(&mut rng);

        tbl_user::ActiveModel {
            email: ActiveValue::Set(email),
            name: ActiveValue::Set(name),
            password: ActiveValue::Set(hash.unprotected_as_encoded().to_string()),
            role_id: ActiveValue::Set(Some(role.role_id.to_owned())),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }

    let users = tbl_user::Entity::find().all(&db).await?;

    for user in &users {
        let mut active_until: Option<NaiveDateTime> = None;
        let mut active = false;
        if rng.gen_ratio(2, 5) {
            active = rng.gen_bool(0.5);
            let due_date: DateTimeUtc =
                DateTimeAfter(chrono::offset::Utc::now()).fake_with_rng(&mut rng);
            active_until = Some(due_date.naive_utc());
        };
        tbl_keycard::ActiveModel {
            user_id: ActiveValue::Set(user.user_id.to_owned()),
            active_until: ActiveValue::Set(active_until),
            active: ActiveValue::Set(active),
            ..Default::default()
        }
        .insert(&db)
        .await?;

        for _ in 0..rng.gen_range(1..4) {
            let sentence: String = Sentence(0..5).fake_with_rng(&mut rng);
            tbl_door_group::ActiveModel {
                name: ActiveValue::Set(Industry(EN).fake_with_rng(&mut rng)),
                description: ActiveValue::Set(Some(sentence)),
                owner_id: ActiveValue::Set(user.user_id.clone()),
                ..Default::default()
            }
            .insert(&db)
            .await?;
        }
    }
    let door_groups = tbl_door_group::Entity::find().all(&db).await?;
    let keycards = tbl_keycard::Entity::find().all(&db).await?;
    let users_filtered: Vec<tbl_user::Model> = users
        .iter()
        .filter(|f| {
            f.role_id.unwrap()
                == roles
                    .iter()
                    .find(|f| f.name == "administrative staff")
                    .unwrap()
                    .role_id
        })
        .map(|f| f.to_owned())
        .collect();
    //insert workers
    for user in users_filtered {
        tbl_worker::ActiveModel {
            user_id: ActiveValue::Set(user.user_id.to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let workers = tbl_worker::Entity::find().all(&db).await?;
    //select leaders out of workers
    for idx in 0..3 {
        tbl_leader::ActiveModel {
            user_id: ActiveValue::Set(workers.get(idx).unwrap().user_id),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let leaders = tbl_leader::Entity::find().all(&db).await?;
    for worker in &workers {
        if leaders.iter().any(|f| f.user_id == worker.user_id) {
            //worker is leader skip
            continue;
        }
        let mut worker: tbl_worker::ActiveModel = worker.clone().into();
        worker.boss_user_id =
            ActiveValue::Set(Some(leaders[rng.gen_range(0..leaders.len())].user_id));
        worker.update(&db).await?;
    }
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

    for group in &door_groups {
        for _ in 0..rng.gen_range(0..20) {
            let res = tbl_door_to_group_door::ActiveModel {
                door_group_id: ActiveValue::Set(group.door_group_id),
                door_id: ActiveValue::Set(doors[rng.gen_range(0..doors.len())].door_id),
            }
            .insert(&db)
            .await;
            if let Err(res) = res {
                info!("{}", res.to_string());
            }
        }
    }
    for _ in 0..200 {
        let lent_date = DateTimeBefore(chrono::offset::Utc::now()).fake_with_rng(&mut rng);
        let due_date = DateTimeAfter(lent_date).fake_with_rng(&mut rng);
        let is_active = chrono::offset::Utc::now() < due_date || rng.gen_ratio(1, 20);
        let sentence: String = Sentence(0..3).fake_with_rng(&mut rng);
        tbl_door_user_access::ActiveModel {
            comment: ActiveValue::Set(Some(sentence)),
            due_at: ActiveValue::Set(Some(due_date.naive_utc())),
            lent_at: ActiveValue::Set(Some(lent_date.naive_utc())),
            has_problem: ActiveValue::Set(Some(rng.gen_ratio(1, 20))),
            is_active: ActiveValue::Set(Some(is_active)),
            lent: ActiveValue::Set(Some(is_active)),
            door_id: ActiveValue::Set(doors[rng.gen_range(0..doors.len())].door_id),
            user_id: ActiveValue::Set(users[rng.gen_range(0..users.len())].user_id),
        }
        .insert(&db)
        .await?;
    }
    for _ in 0..100 {
        let accept = rng.gen_ratio(1, 10);
        let reject = rng.gen_ratio(1, 10);
        let request_base = tbl_request_base::ActiveModel {
            accept: ActiveValue::Set(accept && !reject),
            pending: ActiveValue::Set(accept == reject),
            reject: ActiveValue::Set(!accept && reject),
            description: ActiveValue::Set(Sentence(0..10).fake_with_rng(&mut rng)),
            requester_id: ActiveValue::Set(users[rng.gen_range(0..users.len())].user_id),
            ..Default::default()
        }
        .insert(&db)
        .await?;
        tbl_door_request::ActiveModel {
            door_group_id: ActiveValue::Set(
                door_groups[rng.gen_range(0..door_groups.len())]
                    .door_group_id
                    .to_owned(),
            ),
            request_id: ActiveValue::Set(request_base.request_id.clone()),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let requests = tbl_request_base::Entity::find().all(&db).await?;
    let door_requests = tbl_door_request::Entity::find().all(&db).await?;
    for _ in 0..500 {
        let door_request = &door_requests[rng.gen_range(0..door_requests.len())];
        let request = requests
            .iter()
            .find(|f| &f.request_id == &door_request.request_id)
            .unwrap();
        let user_who_writes_comment;
        //get the user or a random worker
        if rng.gen_bool(0.5) {
            let user = request.requester_id.to_owned();
            user_who_writes_comment = Some(user);
        } else {
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
    for _ in 0..1000 {
        let keycard = &keycards[rng.gen_range(0..keycards.len())];
        let door = &doors[rng.gen_range(0..doors.len())];
        let mut used_at: chrono::DateTime<chrono::Utc> =
            DateTimeBefore(chrono::Utc::now()).fake_with_rng(&mut rng);
        if let Some(val) = keycard.active_until {
            // active_until smaller than now time
            // to prevent assigning historie values into the future
            if val < used_at.naive_utc() {
                used_at = DateTimeBefore(DateTimeUtc::from_utc(val, Utc)).fake_with_rng(&mut rng);
            }
        }
        tbl_keycard_history::ActiveModel {
            keycard_id: ActiveValue::Set(keycard.keycard_id.to_owned()),
            door_id: ActiveValue::Set(door.door_id.to_owned()),
            used_at: ActiveValue::Set(used_at.naive_utc()),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    Ok(())
}
