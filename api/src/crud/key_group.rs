use crate::util::{self, deserialize_some, error::CrudError};
use entities::model::{tbl_door, tbl_door_group, tbl_door_to_group_door, tbl_user};
use sea_orm::ActiveModelTrait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use util::convert_active::Convert;
use utoipa::ToSchema;
use uuid::Uuid;

use super::door::GetDoor;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateKeyGroup {
    name: String,
    description: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangeKeyGroup {
    name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    description: Option<Option<String>>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetKeyGroup {
    pub door_group_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
impl From<&tbl_door_group::Model> for GetKeyGroup {
    fn from(model: &tbl_door_group::Model) -> Self {
        let model = model.clone();
        Self {
            door_group_id: model.door_group_id,
            name: model.name,
            description: model.description,
        }
    }
}
pub async fn create_door_group(
    key_goup: &CreateKeyGroup,
    db: &DatabaseConnection,
) -> Result<GetKeyGroup, CrudError> {
    let model = tbl_door_group::ActiveModel {
        name: Set(key_goup.name.to_string()),
        description: Set(key_goup.description.clone()),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok((&model).into())
}
pub async fn update_door_group(
    key_goup: &ChangeKeyGroup,
    key_group_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetKeyGroup, CrudError> {
    let model = tbl_door_group::Entity::find_by_id(key_group_id.clone())
        .one(db)
        .await?;
    match &model {
        Some(model) => {
            let active_model: tbl_door_group::ActiveModel = model.clone().into();

            let active_model = tbl_door_group::ActiveModel {
                description: key_goup.description.convert(model.description.clone()),
                name: key_goup.name.convert(model.name.clone()),
                ..active_model
            };
            let active_model = active_model.update(db).await?;
            Ok((&active_model).into())
        }
        None => Err(CrudError::NotFound),
    }
}
pub async fn add_door_to_door_group(
    door_id: &Uuid,
    door_group_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<(), CrudError> {
    tbl_door_to_group_door::ActiveModel {
        door_group_id: Set(door_group_id.clone()),
        door_id: Set(door_id.clone()),
    }
    .insert(db)
    .await?;
    Ok(())
}
pub async fn remove_key_from_key_group(
    door_id: &Uuid,
    door_group_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<(), CrudError> {
    let model = tbl_door_to_group_door::Entity::find()
        .filter(tbl_door_to_group_door::Column::DoorGroupId.eq(door_group_id.clone()))
        .filter(tbl_door_to_group_door::Column::DoorId.eq(door_id.clone()))
        .one(db)
        .await?;
    match model {
        Some(model) => {
            model.delete(db).await?;
            Ok(())
        }
        None => Err(CrudError::NotFound),
    }
}
pub async fn get_all_door_group(db: &DatabaseConnection) -> Result<Vec<GetKeyGroup>, CrudError> {
    let model = tbl_door_group::Entity::find().all(db).await?;
    Ok(model.iter().map(|f| f.into()).collect())
}
pub async fn get_doors_of_door_group(
    db: &DatabaseConnection,
    key_group_id: &Uuid,
) -> Result<Vec<GetDoor>, CrudError> {
    let model = tbl_door_to_group_door::Entity::find()
        .filter(tbl_door_to_group_door::Column::DoorGroupId.eq(key_group_id.clone()))
        .find_also_related(tbl_door::Entity)
        .all(db)
        .await?;
    Ok(model
        .iter()
        .map(|f| (f.1.as_ref().expect("can not be null")).into())
        .collect())
}
pub async fn get_door_group_of_user(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<Vec<GetKeyGroup>, CrudError> {
    let model = tbl_user::Entity::find_by_id(user_id.clone())
        .find_also_related(tbl_door_group::Entity)
        .all(db)
        .await?;
    Ok(model
        .iter()
        .map(|f| (f.1.as_ref().expect("can not be null")).into())
        .collect())
}
