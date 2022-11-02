use entities::model::{tbl_role, tbl_user};
use itertools::Itertools;
use paperclip::actix::Apiv2Schema;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::util::{
    deserialize_some,
    error::CrudError,
    user::{CreateActiveModel, UpdateActiveModel},
};

use super::role::GetRole;
#[derive(Serialize, Deserialize, Clone, Debug, Apiv2Schema)]
pub struct GetUser {
    pub user_id: String,
    pub name: String,
    pub role: Option<GetRole>,
    pub email: String,
}
#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct ChangeUser {
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    pub role: Option<Option<String>>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct CreateUser {
    pub name: String,
    pub role: Option<String>,
    pub email: String,
    pub password: String,
}
impl From<&(tbl_user::Model, Option<tbl_role::Model>)> for GetUser {
    fn from((user, role): &(tbl_user::Model, Option<tbl_role::Model>)) -> Self {
        Self {
            user_id: user.user_id.to_string(),
            name: user.name.to_string(),
            role: role.to_owned().map(|f| GetRole::from(&f)),
            email: user.email.to_owned(),
        }
    }
}
pub async fn get_all_user(db: &DatabaseConnection) -> Result<Vec<GetUser>, CrudError> {
    let model = tbl_user::Entity::find()
        .find_also_related(tbl_role::Entity)
        .all(db)
        .await?;
    Ok(model.iter().map(|f| GetUser::from(f)).collect_vec())
}
pub async fn get_single_user(
    db: &DatabaseConnection,
    uuid: uuid::Uuid,
) -> Result<Option<GetUser>, CrudError> {
    let model = tbl_user::Entity::find_by_id(uuid)
        .find_also_related(tbl_role::Entity)
        .one(db)
        .await?;
    Ok(model.map(|f| GetUser::from(&f)))
}
pub async fn delete_user(db: &DatabaseConnection, uuid: String) -> Result<DeleteResult, CrudError> {
    let model = tbl_user::Entity::find_by_id(Uuid::parse_str(&uuid)?)
        .one(db)
        .await?;
    if let Some(model) = model {
        info!("{:#?}", model);
        return Ok(model.delete(db).await?);
    } else {
        return Err(CrudError::NotFound);
    }
}
pub async fn update_user(
    db: &DatabaseConnection,
    user: ChangeUser,
    id: String,
) -> Result<GetUser, CrudError> {
    let model = tbl_user::Entity::find_by_id(Uuid::parse_str(&id)?)
        .one(db)
        .await?;
    if let Some(model) = model {
        let model: tbl_user::ActiveModel = user.convert(model.into())?;
        let user = model.update(db).await?;
        let mut role = None;
        if let Some(role_id) = user.role_id {
            role = tbl_role::Entity::find_by_id(role_id).one(db).await?;
        }
        let user: GetUser = (&(user, role)).into();
        return Ok(user);
    }
    Err(CrudError::NotFound)
}
pub async fn create_user(db: &DatabaseConnection, user: CreateUser) -> Result<GetUser, CrudError> {
    let model: tbl_user::ActiveModel = user.convert()?;
    let user = model.insert(db).await?;
    let mut role = None;
    if let Some(role_id) = user.role_id {
        role = tbl_role::Entity::find_by_id(role_id).one(db).await?;
    }
    let user: GetUser = (&(user, role)).into();
    return Ok(user);
}
