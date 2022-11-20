use entities::model::tbl_door;
use itertools::Itertools;
use sea_orm::{DatabaseConnection, DbBackend, EntityTrait, Statement};

use uuid::Uuid;

use super::building::{GetCompleteBuilding, GetCompleteDoor, GetCompleteRoom};
use super::door::GetDoor;
use crate::crud;
use crate::util::error::CrudError;

pub async fn get_doors_of_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetDoor>, CrudError> {
    let values = tbl_door::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select (tbl_door.*) from tbl_user
            join tbl_door_group on tbl_user.user_id = tbl_door_group.owner_id
            join tbl_door_to_group_door on tbl_door_group.door_group_id = tbl_door_to_group_door.door_group_id
            join tbl_door on tbl_door_to_group_door.door_id = tbl_door.door_id
            where tbl_door_group.is_active = true and
            tbl_user.user_id = $1"#,
            vec![user_id.clone().into()],
        ))
        .all(db)
        .await?;

    Ok(values.iter().map(|f| f.into()).collect())
}
pub async fn get_doors_of_door_group_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetDoor>, CrudError> {
    let values = tbl_door::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select (tbl_door.*) from tbl_door_group
            join tbl_door_to_group_door on tbl_door_group.door_group_id = tbl_door_to_group_door.door_group_id
            join tbl_door on tbl_door_to_group_door.door_id = tbl_door.door_id
            where tbl_door_group.door_group_id = $1"#,
            vec![user_id.clone().into()],
        ))
        .all(db)
        .await?;

    Ok(values.iter().map(|f| f.into()).collect())
}
pub async fn get_building_by_user_id_with_only_authorized_doors(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetCompleteBuilding>, CrudError> {
    let buildings = crud::building::get_building_complex(db).await?;
    let authorized_doors = get_doors_of_user_id(user_id, db).await?;
    let filtered_buildings = get_complex_building_authorized(buildings, authorized_doors);
    Ok(filtered_buildings)
    // Ok(values.iter().map(|f| f.into()).collect())
}
pub async fn get_building_by_door_group_with_only_authorized_doors(
    door_group_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetCompleteBuilding>, CrudError> {
    let buildings = crud::building::get_building_complex(db).await?;
    let authorized_doors = get_doors_of_door_group_id(door_group_id, db).await?;
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
