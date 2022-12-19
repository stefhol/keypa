use entities::model::{tbl_building, tbl_door, tbl_room};
use itertools::Itertools;
use sea_orm::{DatabaseConnection, DbBackend, EntityTrait, Statement};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::door::GetDoor;
use crate::util::error::CrudError;
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GetCompleteBuilding {
    pub building_id: Uuid,
    pub name: String,
    pub rooms: Vec<GetCompleteRoom>,
}
impl From<(&tbl_building::Model, &Vec<GetCompleteRoom>)> for GetCompleteBuilding {
    fn from((building, rooms): (&tbl_building::Model, &Vec<GetCompleteRoom>)) -> Self {
        let building = building.clone();
        Self {
            building_id: building.building_id,
            name: building.name,
            rooms: rooms
                .iter()
                .filter(|f| &f.building_id == &building.building_id)
                .cloned()
                .collect(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GetCompleteRoom {
    pub room_id: Uuid,
    pub name: String,
    pub floor: i32,
    pub is_sensitive: Option<bool>,
    pub building_id: Uuid,
    pub doors: Vec<GetCompleteDoor>,
}
impl From<(&tbl_room::Model, &Vec<GetCompleteDoor>)> for GetCompleteRoom {
    fn from((room, doors): (&tbl_room::Model, &Vec<GetCompleteDoor>)) -> Self {
        let room = room.clone();
        Self {
            doors: doors
                .iter()
                .filter(|f| &f.room_id == &room.room_id)
                .cloned()
                .collect(),
            building_id: room.building_id,
            floor: room.floor,
            is_sensitive: room.is_sensitive,
            name: room.name,
            room_id: room.room_id,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GetCompleteDoor {
    pub door_id: Uuid,
    pub name: String,
    pub owner: bool,
    pub room_id: Uuid,
}
impl From<&tbl_door::Model> for GetCompleteDoor {
    fn from(door: &tbl_door::Model) -> Self {
        let door = door.clone();
        Self {
            door_id: door.door_id,
            name: door.name,
            owner: false,
            room_id: door.room_id,
        }
    }
}
pub async fn get_doors_of_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetDoor>, CrudError> {
    let values = tbl_door::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select (tbl_door.*) from tbl_user
            join tbl_request on tbl_user.user_id = tbl_request.requester_id
            join tbl_door_to_request on tbl_request.request_id = tbl_door_to_request.request_id
            join tbl_door on tbl_door_to_request.door_id = tbl_door.door_id
            where tbl_user.user_id = $1"#,
            vec![user_id.clone().into()],
        ))
        .all(db)
        .await?;

    Ok(values.iter().map(|f| f.into()).collect())
}
pub async fn get_doors_of_department_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetDoor>, CrudError> {
    let values = tbl_door::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select (d.*) from tbl_user
            join tbl_request tr on tbl_user.user_id = tr.requester_id
            join tbl_request_department trd on tr.request_id = trd.request_id
            join tbl_department td on trd.department_id = td.department_id
            join tbl_room_department t on td.department_id = t.department_id
            join tbl_room r on t.room_id = r.room_id
            join tbl_door d on r.room_id = d.room_id
            where tbl_user.user_id = $1"#,
            vec![user_id.clone().into()],
        ))
        .all(db)
        .await?;

    Ok(values.iter().map(|f| f.into()).collect())
}
pub async fn get_building_complex(
    db: &DatabaseConnection,
) -> Result<Vec<GetCompleteBuilding>, CrudError> {
    let doors = tbl_door::Entity::find().all(db).await?;
    let rooms = tbl_room::Entity::find().all(db).await?;
    let buildings = tbl_building::Entity::find().all(db).await?;
    let doors: Vec<GetCompleteDoor> = doors.iter().map(|f| f.into()).collect();
    let rooms: Vec<GetCompleteRoom> = rooms.iter().map(|f| (f, &doors).into()).collect();
    let buildings: Vec<GetCompleteBuilding> =
        buildings.iter().map(|f| (f, &rooms).into()).collect();
    Ok(buildings)
}
pub async fn get_building_by_user_id_with_only_authorized_doors(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetCompleteBuilding>, CrudError> {
    let buildings = get_building_complex(db).await?;
    let mut department_authorized_doors = get_doors_of_department_user_id(user_id, db).await?;
    let mut authorized_doors = get_doors_of_user_id(user_id, db).await?;
    authorized_doors.append(&mut department_authorized_doors);
    let filtered_buildings = get_complex_building_authorized(buildings, authorized_doors);
    Ok(filtered_buildings)
}
/// get only buiding with authorized doors
#[allow(dead_code)]
fn get_only_authorized_complex_building(
    buildings: Vec<GetCompleteBuilding>,
    authorized_doors: Vec<GetDoor>,
) -> Vec<GetCompleteBuilding> {
    let mut filtered_buildings = vec![];
    for builing in buildings {
        let mut rooms = vec![];
        for room in builing.rooms {
            if authorized_doors
                .iter()
                .map(|f| f.room_id)
                .contains(&room.room_id)
            {
                let mut doors = vec![];
                for door in room.doors {
                    if authorized_doors
                        .iter()
                        .map(|f| f.door_id)
                        .contains(&door.door_id)
                    {
                        doors.push(GetCompleteDoor {
                            owner: true,
                            ..door
                        })
                    }
                }
                rooms.push(GetCompleteRoom { doors, ..room })
            }
        }
        if rooms.len() > 0 {
            filtered_buildings.push(GetCompleteBuilding { rooms, ..builing })
        }
    }
    filtered_buildings
}
/// Returns complete structure with value in door changed
fn get_complex_building_authorized(
    buildings: Vec<GetCompleteBuilding>,
    authorized_doors: Vec<GetDoor>,
) -> Vec<GetCompleteBuilding> {
    let buildings: Vec<GetCompleteBuilding> = buildings
        .iter()
        .map(|building| {
            let mut building = building.clone();
            building.rooms = building
                .rooms
                .iter()
                .map(|room| {
                    let mut room = room.clone();
                    room.doors = room
                        .doors
                        .iter()
                        .map(|door| {
                            let mut door = door.clone();
                            door.owner = authorized_doors
                                .iter()
                                .map(|f| f.door_id)
                                .contains(&door.door_id);
                            door
                        })
                        .collect();
                    room
                })
                .collect();
            building
        })
        .collect();
    buildings
}
