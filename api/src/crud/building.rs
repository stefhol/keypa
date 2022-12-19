use itertools::Itertools;
use sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;
#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
struct QueryResult {
    building_id: Uuid,
    building_name: String,
    door_id: Uuid,
    floor: i32,
    is_sensitive: Option<bool>,
    room_id: Uuid,
    room_name: String,
    door_name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GetBuilding {
    pub building_id: Uuid,
    pub name: String,
    pub rooms: Vec<GetRoom>,
}
impl From<(&QueryResult, &Vec<QueryResult>)> for GetBuilding {
    fn from((item, all_items): (&QueryResult, &Vec<QueryResult>)) -> Self {
        let item = item.clone();
        Self {
            building_id: item.building_id,
            name: item.building_name.to_owned(),
            rooms: all_items
                .iter()
                .unique_by(|room| room.room_id)
                .filter(|room| room.building_id == item.building_id)
                .map(|room| GetRoom {
                    building_id: item.building_id,
                    floor: item.floor,
                    is_sensitive: item.is_sensitive,
                    name: item.room_name.to_owned(),
                    room_id: item.room_id,
                    doors: all_items
                        .iter()
                        .unique_by(|door| door.door_id)
                        .filter(|door| {
                            room.building_id == item.building_id && room.door_id == door.door_id
                        })
                        .map(|door| GetDoor {
                            door_id: door.door_id,
                            name: door.door_name.to_owned(),
                            room_id: door.room_id,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GetRoom {
    pub building_id: Uuid,
    pub floor: i32,
    pub is_sensitive: Option<bool>,
    pub name: String,
    pub room_id: Uuid,
    pub doors: Vec<GetDoor>,
}
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GetDoor {
    pub door_id: Uuid,
    pub name: String,
    pub room_id: Uuid,
}
///Get out of raw query results a tree of departments with buildings rooms and doors
async fn query(db: &DatabaseConnection) -> Result<Vec<GetBuilding>, CrudError> {
    let query_result: Vec<QueryResult> = QueryResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        select tbl_room.room_id, tbl_room.name as room_name, floor, is_sensitive, tbl_building.building_id,tbl_building.name as building_name, tbl_door.name as door_name,door_id  from tbl_building
        join tbl_room on tbl_building.building_id = tbl_room.building_id
        join tbl_door on tbl_room.room_id = tbl_door.room_id
        "#,
        vec![],
    ))
    .all(db)
    .await?;
    Ok(query_result
        .iter()
        .unique_by(|building| building.building_id)
        .map(|query| GetBuilding::from((query, &query_result)))
        .collect())
}
pub async fn get_building(db: &DatabaseConnection) -> Result<Vec<GetBuilding>, CrudError> {
    query(db).await
}
pub async fn get_building_without_rooms(
    db: &DatabaseConnection,
) -> Result<Vec<GetBuilding>, CrudError> {
    let buildings = query(db).await?;
    Ok(buildings
        .iter()
        .map(|buildings| {
            let mut buildings = buildings.to_owned();
            buildings.rooms = vec![];
            buildings
        })
        .collect())
}
