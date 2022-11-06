use entities::model::tbl_building;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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
