use itertools::Itertools;
use sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::building::{GetBuilding, GetDoor, GetRoom};

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct GetDepartment {
    pub department_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub buildings: Vec<GetBuilding>,
}

impl From<(&QueryResult, &Vec<QueryResult>)> for GetDepartment {
    fn from((item, all_items): (&QueryResult, &Vec<QueryResult>)) -> Self {
        let item = item.clone();
        //Get out of raw query results a tree of departments with buildings rooms and doors
        Self {
            department_id: item.department_id,
            name: item.department_name.to_owned(),
            description: item.department_description.to_owned(),
            buildings: all_items
                .iter()
                .unique_by(|building| building.building_id)
                .filter(|f| f.department_id == item.department_id)
                .map(|building| {
                    let building = building.clone();
                    GetBuilding {
                        building_id: building.building_id,
                        name: building.building_name.to_owned(),
                        rooms: all_items
                            .iter()
                            .unique_by(|room| room.room_id)
                            .filter(|room| {
                                building.department_id == room.department_id
                                    && room.building_id == building.building_id
                            })
                            .map(|room| {
                                let room = room.clone();
                                GetRoom {
                                    building_id: room.building_id,
                                    floor: room.floor,
                                    is_sensitive: room.is_sensitive,
                                    name: room.room_name.to_owned(),
                                    room_id: room.room_id,
                                    doors: all_items
                                        .iter()
                                        .unique_by(|door| door.door_id)
                                        .filter(|door| {
                                            building.department_id == room.department_id
                                                && room.building_id == building.building_id
                                                && door.door_id == room.door_id
                                        })
                                        .map(|door| {
                                            let door = door.clone();
                                            GetDoor {
                                                door_id: door.door_id,
                                                name: door.door_name.to_owned(),
                                                room_id: door.room_id,
                                            }
                                        })
                                        .collect(),
                                }
                            })
                            .collect(),
                    }
                })
                .collect(),
        }
    }
}
#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
struct QueryResult {
    building_id: Uuid,
    building_name: String,
    department_description: Option<String>,
    department_id: Uuid,
    department_name: String,
    door_id: Uuid,
    floor: i32,
    is_sensitive: Option<bool>,
    room_id: Uuid,
    room_name: String,
    door_name: String,
}
///Get out of raw query results a tree of departments with buildings rooms and doors
async fn query(db: &DatabaseConnection) -> Result<Vec<GetDepartment>, CrudError> {
    let query_result: Vec<QueryResult> = QueryResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        select tbl_department.department_id, tbl_department.name as department_name, tbl_department.description as department_description, tbl_room.room_id, tbl_room.name as room_name, floor, is_sensitive, tbl_building.building_id,tbl_building.name as building_name, tbl_door.name as door_name,door_id  from tbl_department
        join tbl_room_department on tbl_department.department_id = tbl_room_department.department_id
        join tbl_room on tbl_room_department.room_id = tbl_room.room_id
        join tbl_building on tbl_building.building_id = tbl_room.building_id
        join tbl_door on tbl_room.room_id = tbl_door.room_id
        ;
        "#,
        vec![],
    ))
    .all(db)
    .await?;
    Ok(query_result
        .iter()
        .unique_by(|department| department.department_id)
        .map(|query| GetDepartment::from((query, &query_result)))
        .collect())
}
pub async fn get_department(db: &DatabaseConnection) -> Result<Vec<GetDepartment>, CrudError> {
    // let model_department = tbl_department::Entity::find().all(db).await?;
    // let model_room_department = tbl_room_department::Entity::find().all(db).await?;
    // let model = tbl_bu
    Ok(query(db).await?)
}
