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
    room_name: String
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
        let all_items: Vec<_> = all_items
            .iter()
            .filter(|f| item.building_id == f.building_id)
            .collect();
        //Get out of raw query results a tree of departments with buildings rooms and doors
        Self {
            building_id: item.building_id,
            name: item.building_name.to_owned(),
            rooms: all_items
                .iter()
                .unique_by(|f|f.room_id)
                .map(|room| {
                    let all_items: Vec<_> = all_items
                        .iter()
                        .filter(|f| room.room_id == f.room_id)
                        .collect();
                    let room = room.clone();
                    GetRoom {
                        building_id: room.building_id,
                        floor: room.floor,
                        is_sensitive: room.is_sensitive,
                        name: room.room_name.to_owned(),
                        room_id: room.room_id,
                        doors: all_items
                            .iter()
                            .map(|door| {
                                let door = door.clone();
                                GetDoor {
                                    door_id: door.door_id,
                                    room_id: door.room_id,
                                }
                            })
                            .collect(),
                    }
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
    pub room_id: Uuid,
}
///Get out of raw query results a tree of departments with buildings rooms and doors
async fn query(db: &DatabaseConnection) -> Result<Vec<GetBuilding>, CrudError> {
    let mut query_result: Vec<QueryResult> = QueryResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        select tbl_room.room_id, tbl_room.name as room_name, floor, is_sensitive, tbl_building.building_id,tbl_building.name as building_name, door_id  from tbl_building
        join tbl_room on tbl_building.building_id = tbl_room.building_id
        join tbl_door on tbl_room.room_id = tbl_door.room_id
        "#,
        vec![],
    ))
    .all(db)
    .await?;
    query_result.sort_by(|a,b|a.room_name.cmp(&b.room_name));
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
