use anyhow;

use chrono::{NaiveDateTime, Utc};
use dotenv;
use entities::model::{
    tbl_building, tbl_door, tbl_key, tbl_key_group, tbl_key_group_key, tbl_key_user_history,
    tbl_keycard, tbl_keycard_history, tbl_leader, tbl_request, tbl_request_comment, tbl_role,
    tbl_room, tbl_user, tbl_worker,
};
use fake::faker::address::raw::BuildingNumber;
use fake::faker::barcode::zh_tw::Isbn13;
use fake::faker::chrono::en::DateTimeAfter;
use fake::faker::chrono::raw::Time;
use fake::faker::chrono::zh_tw::DateTimeBefore;
use fake::faker::company::en::{BsNoun, CatchPhase};
use fake::faker::company::raw::CompanyName;
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
    println!("{}/{}", database_url, db_name);

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
        name: ActiveValue::Set("Angestellter".to_string()),
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
    }
    let keycards = tbl_keycard::Entity::find().all(&db).await?;
    let users_filtered:Vec<tbl_user::Model>= users.iter().filter(|f| {
        f.role_id.unwrap()
            == roles
                .iter()
                .find(|f| f.name == "Angestellter")
                .unwrap()
                .role_id
    }).map(|f|f.to_owned()).collect::<>();
    //insert workers
    for user in users_filtered {
        println!("{:#?}",user);
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
        let name: String = BuildingNumber(EN).fake_with_rng(&mut rng);
        tbl_building::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let buildings = tbl_building::Entity::find().all(&db).await?;
    for _ in 0..100 {
        tbl_room::ActiveModel {
            building_id: ActiveValue::Set(buildings[rng.gen_range(0..buildings.len())].building_id),
            floor: ActiveValue::Set(rng.gen_range(0..5)),
            is_sensitive: ActiveValue::Set(Some(rng.gen_ratio(1, 8))),
            name: ActiveValue::Set(CompanyName(EN).fake_with_rng(&mut rng)),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let rooms = tbl_room::Entity::find().all(&db).await?;
    for room in &rooms {
        tbl_door::ActiveModel {
            name: ActiveValue::Set(BsNoun().fake_with_rng(&mut rng)),
            room_id: ActiveValue::Set(Some(room.room_id)),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    for _ in 0..200 {
        tbl_door::ActiveModel {
            name: ActiveValue::Set(BsNoun().fake_with_rng(&mut rng)),
            room_id: ActiveValue::Set(Some(
                rooms[rng.gen_range(0..rooms.len())].room_id.to_owned(),
            )),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let doors = tbl_door::Entity::find().all(&db).await?;
    for _ in 0..400 {
        tbl_key::ActiveModel {
            description: ActiveValue::Set(CatchPhase().fake_with_rng(&mut rng)),
            door_id: ActiveValue::Set(doors[rng.gen_range(0..doors.len())].door_id),
            name: ActiveValue::Set(Isbn13().fake_with_rng(&mut rng)),
            value: ActiveValue::Set(Isbn13().fake_with_rng(&mut rng)),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let keys = tbl_key::Entity::find().all(&db).await?;
    for _ in 0..20 {
        let sentence: String = Sentence(0..2).fake_with_rng(&mut rng);
        tbl_key_group::ActiveModel {
            name: ActiveValue::Set(Industry(EN).fake_with_rng(&mut rng)),
            description: ActiveValue::Set(Some(sentence)),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let key_groups = tbl_key_group::Entity::find().all(&db).await?;

    for group in &key_groups {
        for _ in 0..rng.gen_range(0..20) {
            let res = tbl_key_group_key::ActiveModel {
                key_group_id: ActiveValue::Set(group.key_group_id),
                key_id: ActiveValue::Set(keys[rng.gen_range(0..keys.len())].key_id),
            }
            .insert(&db)
            .await;
            if let Err(res) = res {
                info!("{}", res.to_string());
            }
        }
    }
    for _ in 0..200 {
        let has_active_time = rng.gen_ratio(1, 5);
        let mut active_at = None;
        let mut active_duration = None;
        if has_active_time {
            active_at = Some(Time(EN).fake_with_rng(&mut rng));
            active_duration = Some(rng.gen_range(60..(60 * 60 * 20)));
        }
        let lent_date = DateTimeBefore(chrono::offset::Utc::now()).fake_with_rng(&mut rng);
        let due_date = DateTimeAfter(lent_date).fake_with_rng(&mut rng);
        let is_active = chrono::offset::Utc::now() < due_date || rng.gen_ratio(1, 20);
        let sentence: String = Sentence(0..3).fake_with_rng(&mut rng);
        tbl_key_user_history::ActiveModel {
            active_at: ActiveValue::Set(active_at),
            active_duration: ActiveValue::Set(active_duration),
            comment: ActiveValue::Set(Some(sentence)),
            due_at: ActiveValue::Set(Some(due_date.naive_utc())),
            lent_at: ActiveValue::Set(Some(lent_date.naive_utc())),
            has_problem: ActiveValue::Set(Some(rng.gen_ratio(1, 20))),
            is_active: ActiveValue::Set(Some(is_active)),
            lent: ActiveValue::Set(Some(is_active)),
            key_id: ActiveValue::Set(keys[rng.gen_range(0..keys.len())].key_id),
            user_id: ActiveValue::Set(users[rng.gen_range(0..users.len())].user_id),
        }
        .insert(&db)
        .await?;
    }
    for _ in 0..100 {
        let accept = rng.gen_ratio(1, 10);
        let reject = rng.gen_ratio(1, 10);
        tbl_request::ActiveModel {
            accept: ActiveValue::Set(Some(accept && !reject)),
            pending: ActiveValue::Set(Some(accept == reject)),
            reject: ActiveValue::Set(Some(!accept && reject)),
            description: ActiveValue::Set(Sentence(0..10).fake_with_rng(&mut rng)),
            key_group_id: ActiveValue::Set(
                key_groups[rng.gen_range(0..key_groups.len())]
                    .key_group_id
                    .to_owned(),
            ),
            requester_id: ActiveValue::Set(users[rng.gen_range(0..users.len())].user_id),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let requests = tbl_request::Entity::find().all(&db).await?;
    for _ in 0..500 {
        let request = &requests[rng.gen_range(0..requests.len())];
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
