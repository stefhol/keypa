use entities::model::tbl_room;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::building::GetBuilding;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetRoom {
    pub room_id: Uuid,
    pub name: String,
    pub floor: i32,
    pub is_sensitive: Option<bool>,
    pub building: Option<GetBuilding>,
    pub building_id: Uuid,
}
impl From<&tbl_room::Model> for GetRoom {
    fn from(room: &tbl_room::Model) -> Self {
        Self {
            name: room.name.clone(),
            room_id: room.room_id,
            floor: room.floor,
            is_sensitive: room.is_sensitive,
            building_id: room.building_id,
            building: None,
        }
    }
}
pub async fn get_door_from_room_id(
    room_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetRoom, CrudError> {
    let model = tbl_room::Entity::find_by_id(room_id.clone())
        .one(db)
        .await?;
    match model {
        Some(model) => Ok((&model).into()),
        None => Err(CrudError::NotFound),
    }
}
