use entities::model::{tbl_admin, tbl_role, tbl_user};
use itertools::Itertools;
use paperclip::actix::Apiv2Schema;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, ModelTrait,
    QueryFilter,
};
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
pub async fn get_user_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<tbl_user::Model>, CrudError> {
    let model = tbl_user::Entity::find()
        .filter(tbl_user::Column::Email.eq(email))
        .one(db)
        .await?;
    Ok(model)
}
pub async fn get_single_user(
    db: &DatabaseConnection,
    user_id: &str,
) -> Result<Option<GetUser>, CrudError> {
    let user_id = Uuid::parse_str(user_id)?;
    let model = tbl_user::Entity::find_by_id(user_id)
        .find_also_related(tbl_role::Entity)
        .filter(tbl_user::Column::IsActive.eq(true))
        .one(db)
        .await?;
    Ok(model.map(|f| GetUser::from(&f)))
}
pub async fn delete_user(
    db: &DatabaseConnection,
    user_id: &str,
) -> Result<DeleteResult, CrudError> {
    let user_id = Uuid::parse_str(user_id)?;
    let model = tbl_user::Entity::find_by_id(user_id).one(db).await?;
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
    id: &str,
) -> Result<GetUser, CrudError> {
    let model = tbl_user::Entity::find_by_id(Uuid::parse_str(id)?)
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
pub async fn is_admin_by_user_id(
    user_id: &str,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let user_id = Uuid::parse_str(user_id)?;
    let admin = tbl_admin::Entity::find_by_id(user_id).one(db).await?;
    return Ok(admin.is_some());
}
