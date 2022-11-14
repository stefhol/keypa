use entities::model::{tbl_building, tbl_door, tbl_room};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GetBuilding {
    pub building_id: Uuid,
    pub name: String,
}
impl From<&tbl_building::Model> for GetBuilding {
    fn from(building: &tbl_building::Model) -> Self {
        let building = building.clone();
        Self {
            building_id: building.building_id,
            name: building.name,
        }
    }
}
pub async fn get_door_from_room_id(
    building_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetBuilding, CrudError> {
    let model = tbl_building::Entity::find_by_id(building_id.clone())
        .one(db)
        .await?;
    match model {
        Some(model) => Ok((&model).into()),
        None => Err(CrudError::NotFound),
    }
}
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
