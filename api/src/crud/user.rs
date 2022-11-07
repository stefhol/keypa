use entities::model::{tbl_admin, tbl_role, tbl_user};
use itertools::Itertools;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::{convert_active::Convert, deserialize_some, error::CrudError};

use super::role::GetRole;
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct GetUser {
    pub user_id: Uuid,
    pub name: String,
    pub role: Option<GetRole>,
    pub email: String,
}
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ChangeUser {
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    pub role_id: Option<Option<Uuid>>,
    pub email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    pub picture_url: Option<Option<String>>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub role_id: Option<Uuid>,
    pub email: String,
    pub password: String,
    pub picture_url: Option<String>,
}
impl From<&(tbl_user::Model, Option<tbl_role::Model>)> for GetUser {
    fn from((user, role): &(tbl_user::Model, Option<tbl_role::Model>)) -> Self {
        Self {
            user_id: user.user_id.clone(),
            name: user.name.to_string(),
            role: role.to_owned().map(|f| GetRole::from(&f)),
            email: user.email.to_owned(),
        }
    }
}
pub async fn get_all_user(db: &DatabaseConnection) -> Result<Vec<GetUser>, CrudError> {
    let model = tbl_user::Entity::find()
        .find_also_related(tbl_role::Entity)
        .filter(tbl_user::Column::IsActive.eq(true))
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
        .filter(tbl_user::Column::IsActive.eq(true))
        .one(db)
        .await?;
    Ok(model)
}
pub async fn get_single_user(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<GetUser, CrudError> {
    let user_id = user_id.clone();
    let model = tbl_user::Entity::find_by_id(user_id)
        .find_also_related(tbl_role::Entity)
        .filter(tbl_user::Column::IsActive.eq(true))
        .one(db)
        .await?;
    match model {
        Some(model) => Ok(GetUser::from(&model)),
        None => Err(CrudError::NotFound),
    }
}
pub async fn delete_user(db: &DatabaseConnection, user_id: &Uuid) -> Result<(), CrudError> {
    let user_id = user_id.clone();
    let model = tbl_user::Entity::find_by_id(user_id).one(db).await?;
    match model {
        Some(model) => {
            let mut model: tbl_user::ActiveModel = model.into();
            model.is_active = Set(false);
            model.update(db).await?;
            return Ok(());
        }
        None => return Err(CrudError::NotFound),
    }
}
pub async fn update_user(
    db: &DatabaseConnection,
    change_user: ChangeUser,
    user_id: &Uuid,
) -> Result<GetUser, CrudError> {
    let model = tbl_user::Entity::find_by_id(user_id.clone())
        .one(db)
        .await?;
    if let Some(model) = &model {
        let active_model: tbl_user::ActiveModel = model.clone().into();
        let active_model = tbl_user::ActiveModel {
            email: change_user.email.convert(model.email.clone()),
            name: change_user.name.convert(model.name.clone()),
            role_id: change_user.role_id.convert(model.role_id.clone()),
            picture_url: change_user.picture_url.convert(model.picture_url.clone()),
            ..active_model
        };
        let user = active_model.update(db).await?;
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
    let model = tbl_user::ActiveModel {
        name: Set(user.name.clone()),
        role_id: Set(user.role_id.clone()),
        is_active: Set(true),
        email: Set(user.email.clone()),
        picture_url: Set(user.picture_url.clone()),
        password: Set(user.password.clone()),
        ..Default::default()
    };
    let user = model.insert(db).await?;
    let mut role = None;
    if let Some(role_id) = user.role_id {
        role = tbl_role::Entity::find_by_id(role_id).one(db).await?;
    }
    let user: GetUser = (&(user, role)).into();
    return Ok(user);
}
pub async fn is_admin_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let user_id = user_id.clone();
    let admin = tbl_admin::Entity::find_by_id(user_id).one(db).await?;
    return Ok(admin.is_some());
}
