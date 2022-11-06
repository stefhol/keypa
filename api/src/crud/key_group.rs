use entities::model::{tbl_key, tbl_key_group, tbl_key_group_key, tbl_user};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::key::GetKey;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateKeyGroup {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetKeyGroup {
    pub key_group_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
impl From<&tbl_key_group::Model> for GetKeyGroup {
    fn from(model: &tbl_key_group::Model) -> Self {
        let model = model.clone();
        Self {
            key_group_id: model.key_group_id,
            name: model.name,
            description: model.description,
        }
    }
}
pub async fn create_key_group(
    key_goup: &CreateKeyGroup,
    db: &DatabaseConnection,
) -> Result<GetKeyGroup, CrudError> {
    let model = tbl_key_group::ActiveModel {
        name: Set(key_goup.name.to_string()),
        description: Set(key_goup.description.clone()),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok((&model).into())
}
pub async fn update_key_group(
    key_goup: &CreateKeyGroup,
    key_group_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetKeyGroup, CrudError> {
    let model = tbl_key_group::Entity::find_by_id(key_group_id.clone())
        .one(db)
        .await?;
    match model {
        Some(model) => {
            let mut model: tbl_key_group::ActiveModel = model.into();
            model.description = Set(key_goup.description.clone());
            model.name = Set(key_goup.name.clone());
            let model = model.update(db).await?;
            Ok((&model).into())
        }
        None => Err(CrudError::NotFound),
    }
}
pub async fn add_key_to_key_group(
    key_id: &Uuid,
    key_group_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<(), CrudError> {
    tbl_key_group_key::ActiveModel {
        key_group_id: Set(key_group_id.clone()),
        key_id: Set(key_id.clone()),
    }
    .insert(db)
    .await?;
    Ok(())
}
pub async fn remove_key_from_key_group(
    key_id: &Uuid,
    key_group_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<(), CrudError> {
    let model = tbl_key_group_key::Entity::find()
        .filter(tbl_key_group_key::Column::KeyGroupId.eq(key_group_id.clone()))
        .filter(tbl_key_group_key::Column::KeyId.eq(key_id.clone()))
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
pub async fn get_all_key_group(db: &DatabaseConnection) -> Result<Vec<GetKeyGroup>, CrudError> {
    let model = tbl_key_group::Entity::find().all(db).await?;
    Ok(model.iter().map(|f| f.into()).collect())
}
pub async fn get_keys_of_key_group(
    db: &DatabaseConnection,
    key_group_id: &Uuid,
) -> Result<Vec<GetKey>, CrudError> {
    let model = tbl_key_group_key::Entity::find()
        .filter(tbl_key_group_key::Column::KeyGroupId.eq(key_group_id.clone()))
        .find_also_related(tbl_key::Entity)
        .all(db)
        .await?;
    Ok(model
        .iter()
        .map(|f| (f.1.as_ref().expect("can not be null")).into())
        .collect())
}
pub async fn get_key_group_of_user(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<Vec<GetKeyGroup>, CrudError> {
    let model = tbl_user::Entity::find_by_id(user_id.clone())
        .find_also_related(tbl_key_group::Entity)
        .all(db)
        .await?;
    Ok(model
        .iter()
        .map(|f| (f.1.as_ref().expect("can not be null")).into())
        .collect())
}
