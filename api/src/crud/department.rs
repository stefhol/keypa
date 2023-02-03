use itertools::Itertools;
use sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::building::{GetBuilding, GetDoor, GetRoom};

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GetDepartment {
    pub department_id: Uuid,
    pub is_sensitive: Option<bool>,
    pub name: String,
    pub description: Option<String>,
    pub buildings: Vec<GetBuilding>,
}

impl From<(&QueryResult, &Vec<QueryResult>, &Vec<Uuid>)> for GetDepartment {
    fn from(
        (item, all_items, sensitive_departments): (&QueryResult, &Vec<QueryResult>, &Vec<Uuid>),
    ) -> Self {
        let item = item.clone();
        let all_items: Vec<_> = all_items
            .iter()
            .filter(|f| item.department_id == f.department_id)
            .collect();
        //Get out of raw query results a tree of departments with buildings rooms and doors
        Self {
            is_sensitive: Some(
                sensitive_departments
                    .iter()
                    .any(|f| f.to_owned() == item.department_id),
            ),
            department_id: item.department_id,
            name: item.department_name.to_owned(),
            description: item.department_description.to_owned(),
            buildings: all_items
                .iter()
                .unique_by(|f| f.building_id)
                .map(|building| {
                    let all_items: Vec<_> = all_items
                        .iter()
                        .filter(|f| building.building_id == f.building_id)
                        .collect();
                    let building = building.clone();
                    GetBuilding {
                        building_id: building.building_id,
                        name: building.building_name.to_owned(),
                        rooms: all_items
                            .iter()
                            .unique_by(|f| f.room_id)
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
}
///Get out of raw query results a tree of departments with buildings rooms and doors
async fn query(db: &DatabaseConnection) -> Result<Vec<GetDepartment>, CrudError> {
    let mut query_result: Vec<QueryResult> = QueryResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        select tbl_department.department_id, tbl_department.name as department_name, tbl_department.description as department_description, tbl_room.room_id, tbl_room.name as room_name, floor, is_sensitive, tbl_building.building_id,tbl_building.name as building_name, door_id  from tbl_department
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
    query_result.sort_by(|a, b| a.room_name.cmp(&b.room_name));
    let sensitive_departments = query_sensitive_departments(db).await?;
    Ok(query_result
        .iter()
        .unique_by(|f| f.department_id)
        .map(|query| GetDepartment::from((query, &query_result, &sensitive_departments)))
        .collect())
}

pub async fn query_sensitive_departments(db: &DatabaseConnection) -> Result<Vec<Uuid>, CrudError> {
    #[derive(Serialize, Deserialize, FromQueryResult, Debug)]
    struct QuerySensitiveDepartmentsResult {
        department_id: Uuid,
    }
    let query_result: Vec<QuerySensitiveDepartmentsResult> =
        QuerySensitiveDepartmentsResult::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            select distinct is_sensitive, tbl_room_department.department_id from tbl_room_department
            join tbl_room on tbl_room.room_id = tbl_room_department.room_id where is_sensitive = true;
        "#,
            vec![],
        ))
        .all(db)
        .await?;
    Ok(query_result.iter().map(|f| f.department_id).collect())
}
///Get out of raw query results a tree of departments with buildings rooms and doors
async fn query_of_user_id_without_temp(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<Vec<GetDepartment>, CrudError> {
    let mut query_result: Vec<QueryResult> = QueryResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        select distinct tbl_department.department_id, tbl_department.name as department_name, tbl_department.description as department_description, tbl_room.room_id, tbl_room.name as room_name, floor, is_sensitive, tbl_building.building_id,tbl_building.name as building_name, door_id  from tbl_department
        join tbl_room_department on tbl_department.department_id = tbl_room_department.department_id
        join tbl_room on tbl_room_department.room_id = tbl_room.room_id
        join tbl_building on tbl_building.building_id = tbl_room.building_id
        join tbl_door on tbl_room.room_id = tbl_door.room_id
        join tbl_request_department trd on tbl_department.department_id = trd.department_id
        join tbl_request tr on trd.request_id = tr.request_id
        where tr.requester_id = $1
        and tr.active = true
        and tr.accept = true
        and tr.keycard_id is NULL
        ;
        "#,
        vec![user_id.clone().into()],
    ))
    .all(db)
    .await?;
    let sensitive_departments = query_sensitive_departments(db).await?;
    query_result.sort_by(|a, b| a.room_name.cmp(&b.room_name));
    Ok(query_result
        .iter()
        .unique_by(|f| f.department_id)
        .map(|query| GetDepartment::from((query, &query_result, &sensitive_departments)))
        .collect())
}
///Get out of raw query results a tree of departments with buildings rooms and doors
async fn query_of_user_id_with_keycard_id(
    db: &DatabaseConnection,
    user_id: &Uuid,
    keycard_id: &Uuid,
) -> Result<Vec<GetDepartment>, CrudError> {
    let mut query_result: Vec<QueryResult> = QueryResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        select distinct tbl_department.department_id, tbl_department.name as department_name, tbl_department.description as department_description, tbl_room.room_id, tbl_room.name as room_name, floor, is_sensitive, tbl_building.building_id,tbl_building.name as building_name, door_id  from tbl_department
        join tbl_room_department on tbl_department.department_id = tbl_room_department.department_id
        join tbl_room on tbl_room_department.room_id = tbl_room.room_id
        join tbl_building on tbl_building.building_id = tbl_room.building_id
        join tbl_door on tbl_room.room_id = tbl_door.room_id
        join tbl_request_department trd on tbl_department.department_id = trd.department_id
        join tbl_request tr on trd.request_id = tr.request_id
        where tr.requester_id = $1
        and tr.active = true
        and tr.accept = true
        and tr.keycard_id = $2
        ;
        "#,
        vec![
            user_id.clone().into(),
            keycard_id.clone().into()
        ],
    ))
    .all(db)
    .await?;
    let sensitive_departments = query_sensitive_departments(db).await?;
    query_result.sort_by(|a, b| a.room_name.cmp(&b.room_name));
    Ok(query_result
        .iter()
        .unique_by(|f| f.department_id)
        .map(|query| GetDepartment::from((query, &query_result, &sensitive_departments)))
        .collect())
}
/// get all departments in the db
pub async fn get_department(db: &DatabaseConnection) -> Result<Vec<GetDepartment>, CrudError> {
    Ok(query(db).await?)
}
/// get the departments in the possession of the user
/// 
/// Does not return the departments in a temp request
pub async fn get_department_of_user_id(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<Vec<GetDepartment>, CrudError> {
    Ok(query_of_user_id_without_temp(db, user_id).await?)
}
/// get the departments in the possession of the user and a specific keycard 
/// 
/// This is used for temp keycards
pub async fn get_department_of_user_id_and_keycard_id
(
    db: &DatabaseConnection,
    user_id: &Uuid,
    keycard_id: &Uuid,
) -> Result<Vec<GetDepartment>, CrudError> {
    Ok(query_of_user_id_with_keycard_id(db, user_id, keycard_id).await?)
}
