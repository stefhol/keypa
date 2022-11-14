use crate::crud;
use crate::util::{convert_active::Convert, deserialize_some, error::CrudError};
use async_recursion::async_recursion;
use entities::model::{tbl_admin, tbl_role, tbl_user};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

<<<<<<< HEAD
use super::{role::GetRole, worker::GetSmallWorker};
=======
use crate::util::{convert_active::Convert, deserialize_some, error::CrudError};

use super::role::GetRole;
>>>>>>> 9560445 (minor fixes)
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct GetUser {
    pub user_id: Uuid,
    pub name: String,
    pub role: Option<GetRole>,
    pub email: String,
    pub worker: Option<GetSmallWorker>,
    pub is_leader: Option<bool>,
    pub is_admin: Option<bool>,
}
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct GetUserSmall {
    pub user_id: Uuid,
    pub name: String,
    pub role: Option<GetRole>,
    pub email: String,
    pub is_worker: Option<bool>,
    pub is_leader: Option<bool>,
    pub is_admin: Option<bool>,
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
            is_admin: None,
            is_leader: None,
            worker: None,
        }
    }
}
impl From<&(tbl_user::Model, Option<tbl_role::Model>)> for GetUserSmall {
    fn from((user, role): &(tbl_user::Model, Option<tbl_role::Model>)) -> Self {
        Self {
            user_id: user.user_id.clone(),
            name: user.name.to_string(),
            role: role.to_owned().map(|f| GetRole::from(&f)),
            email: user.email.to_owned(),
            is_admin: None,
            is_leader: None,
            is_worker: None,
        }
    }
}
impl From<&GetUser> for GetUserSmall {
    fn from(user: &GetUser) -> Self {
        let user = user.clone();
        Self {
            user_id: user.user_id,
            name: user.name,
            role: user.role,
            email: user.email,
            is_worker: Some(user.worker.is_some()),
            is_leader: user.is_leader,
            is_admin: user.is_admin,
        }
    }
}

pub async fn get_all_user(db: &DatabaseConnection) -> Result<Vec<GetUser>, CrudError> {
<<<<<<< HEAD
    let users = get_raw_all_user(db).await?;
    let mut user_vec = vec![];
    for user in users {
        let user = fill_single_user(&user, db).await?;
        user_vec.push(user);
    }
    Ok(user_vec)
=======
    let model = tbl_user::Entity::find()
        .find_also_related(tbl_role::Entity)
        .filter(tbl_user::Column::IsActive.eq(true))
        .all(db)
        .await?;
    Ok(model.iter().map(|f| GetUser::from(f)).collect_vec())
>>>>>>> 9560445 (minor fixes)
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
#[async_recursion]
pub async fn get_single_user(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<GetUser, CrudError> {
    let user = get_raw_single_user(user_id, db).await?;
    let user = fill_single_user(&user, db).await?;

    Ok(user)
}
pub async fn fill_single_user(
    user: &GetUser,
    db: &DatabaseConnection,
) -> Result<GetUser, CrudError> {
    let mut user = user.clone();
    user.is_admin = Some(is_admin_by_user_id(&user.user_id, db).await?);
    user.is_leader = Some(crud::worker::is_leader_by_user_id(&user.user_id, db).await?);
    match crud::worker::get_worker_by_user_id(db, &user.user_id).await {
        Ok(worker) => {
            user.worker = Some(worker);
            Ok(user)
        }
        Err(_) => Ok(user),
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
async fn get_raw_single_user(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetUser, CrudError> {
    let user = tbl_user::Entity::find_by_id(user_id.clone())
        .find_also_related(tbl_role::Entity)
        .filter(tbl_user::Column::IsActive.eq(true))
        .one(db)
        .await?;
    match &user {
        Some(user) => Ok(user.into()),
        None => Err(CrudError::NotFound),
    }
}
async fn get_raw_all_user(db: &DatabaseConnection) -> Result<Vec<GetUser>, CrudError> {
    Ok(tbl_user::Entity::find()
        .find_also_related(tbl_role::Entity)
        .filter(tbl_user::Column::IsActive.eq(true))
        .all(db)
        .await?
        .iter()
        .map(|f| f.into())
        .collect())
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
