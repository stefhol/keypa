use entities::model::{tbl_door, tbl_room};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::{building::GetBuilding, room::GetRoom};
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetDoorWithRoom {
    pub door_id: Uuid,
    pub room_id: Uuid,
    pub room_name: String,
    pub floor: i32,
    pub is_sensitive: Option<bool>,
    pub building_id: Uuid,
    pub building: Option<GetBuilding>,
}
impl From<&(tbl_door::Model, Option<tbl_room::Model>)> for GetDoorWithRoom {
    fn from((door, room): &(tbl_door::Model, Option<tbl_room::Model>)) -> Self {
        let room = room.clone().expect("Can not be null");
        Self {
            door_id: door.door_id,
            room_id: room.room_id,
            room_name: room.name,
            floor: room.floor,
            building: None,
            is_sensitive: room.is_sensitive,
            building_id: room.building_id,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetDoor {
    pub door_id: Uuid,
    pub room_id: Uuid,
    pub room: Option<GetRoom>,
}
impl From<&tbl_door::Model> for GetDoor {
    fn from(door: &tbl_door::Model) -> Self {
        Self {
            door_id: door.door_id,
            room_id: door.room_id,
            room: None,
        }
    }
}
pub async fn get_door_and_room_from_door_id(
    door_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetDoorWithRoom, CrudError> {
    let model = tbl_door::Entity::find_by_id(door_id.clone())
        .find_also_related(tbl_room::Entity)
        .one(db)
        .await?;
    match model {
        Some(model) => Ok((&model).into()),
        None => Err(CrudError::NotFound),
    }
}
pub async fn get_door_from_door_id(
    door_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetDoor, CrudError> {
    let model = tbl_door::Entity::find_by_id(door_id.clone())
        .one(db)
        .await?;
    match model {
        Some(model) => Ok((&model).into()),
        None => Err(CrudError::NotFound),
    }
}
