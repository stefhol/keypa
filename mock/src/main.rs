use anyhow;

use dotenv;
use entities::model::{
    tbl_building, tbl_department, tbl_door, tbl_room,
    tbl_room_department, tbl_user,
};




use rand::Rng;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, EntityTrait};
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

    let db = Database::connect(format!("{}/{}", database_url, db_name)).await?;
    let mut rng = rand::thread_rng();

    let password = orion::pwhash::Password::from_slice(b"1234").unwrap();
    let hash = orion::pwhash::hash_password(&password, 3, 1 << 16).unwrap();
    
    let _ = tbl_user::ActiveModel {
        email: ActiveValue::Set(String::from("admin@demo.de")),
        name: ActiveValue::Set(String::from("Demo Admin")),
        password: ActiveValue::Set(hash.unprotected_as_encoded().to_string()),
        role_id: ActiveValue::Set(Some(1)),
        ..Default::default()
    }
    .insert(&db)
    .await;
    let _ = tbl_user::ActiveModel {
        email: ActiveValue::Set(String::from("vl@demo.de")),
        name: ActiveValue::Set(String::from("Demo Verwatlungsvorgesetzter")),
        password: ActiveValue::Set(hash.unprotected_as_encoded().to_string()),
        role_id: ActiveValue::Set(Some(2)),
        ..Default::default()
    }
    .insert(&db)
    .await;
    let _ = tbl_user::ActiveModel {
        email: ActiveValue::Set(String::from("vw@demo.de")),
        name: ActiveValue::Set(String::from("Demo Verwatlungsmitarbeiter")),
        password: ActiveValue::Set(hash.unprotected_as_encoded().to_string()),
        role_id: ActiveValue::Set(Some(3)),
        ..Default::default()
    }
    .insert(&db)
    .await;
    let _ = tbl_user::ActiveModel {
        email: ActiveValue::Set(String::from("mi@demo.de")),
        name: ActiveValue::Set(String::from("Demo Mitarbeiter")),
        password: ActiveValue::Set(hash.unprotected_as_encoded().to_string()),
        role_id: ActiveValue::Set(Some(4)),
        ..Default::default()
    }
    .insert(&db)
    .await;

    let _users = tbl_user::Entity::find().all(&db).await?;

    for name in vec!["WIWI", "FIM", "AM", "JURA"] {
        let _ = tbl_building::ActiveModel {
            name: ActiveValue::Set(name.to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await;
    }
    let buildings = tbl_building::Entity::find().all(&db).await?;
    for _ in 0..100 {
        let floor = rng.gen_range(0..5);
        let room_number = rng.gen_range(1..20);
        let name = format!("{}{:02}", &floor, &room_number);
        let _ = tbl_room::ActiveModel {
            building_id: ActiveValue::Set(buildings[rng.gen_range(0..buildings.len())].building_id),
            floor: ActiveValue::Set(floor),
            is_sensitive: ActiveValue::Set(Some(rng.gen_ratio(1, 10))),
            name: ActiveValue::Set(name),
            ..Default::default()
        }
        .insert(&db)
        .await;
    }
    let rooms = tbl_room::Entity::find().all(&db).await?;
    for room in &rooms {
        tbl_door::ActiveModel {
            room_id: ActiveValue::Set(room.room_id),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    for _ in 0..200 {
        tbl_door::ActiveModel {
            room_id: ActiveValue::Set(rooms[rng.gen_range(0..rooms.len())].room_id.to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await?;
    }
    let _doors = tbl_door::Entity::find().all(&db).await?;
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
    let _departments = tbl_department::Entity::find().all(&db).await?;
    
    Ok(())
}
