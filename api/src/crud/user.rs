use crate::util::error::CrudError;
use entities::model::{tbl_role, tbl_user};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::role::GetRole;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct GetUser {
    pub user_id: Uuid,
    pub name: String,
    pub role: Option<GetRole>,
    pub email: String,
    pub tel: Option<String>,
    pub address: Option<String>,
    pub picture_url: Option<String>,
}

impl From<&(tbl_user::Model, Option<tbl_role::Model>)> for GetUser {
    fn from((user, role): &(tbl_user::Model, Option<tbl_role::Model>)) -> Self {
        Self {
            user_id: user.user_id.clone(),
            name: user.name.to_string(),
            role: role.to_owned().map(|f| GetRole::from(&f)),
            email: user.email.to_owned(),
            tel: user.tel.to_owned(),
            address: user.address.to_owned(),
            picture_url: user.address.to_owned(),
        }
    }
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
pub async fn get_all_user(db: &DatabaseConnection) -> Result<Vec<GetUser>, CrudError> {
    Ok(tbl_user::Entity::find()
        .find_also_related(tbl_role::Entity)
        .filter(tbl_user::Column::IsActive.eq(true))
        .all(db)
        .await?
        .iter()
        .map(|f| f.into())
        .collect())
}

pub async fn is_admin_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let user = get_single_user(user_id, db).await?;
    return match user.role {
        Some(role) => Ok(role.name == "administrative admin"),
        None => Ok(false),
    };
}
pub async fn is_worker_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let user = get_single_user(user_id, db).await?;
    return match user.role {
        Some(role) => Ok(role.name == "administrative staff"),
        None => Ok(false),
    };
}
pub async fn is_leader_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let user = get_single_user(user_id, db).await?;
    return match user.role {
        Some(role) => Ok(role.name == "administrative leader"),
        None => Ok(false),
    };
}
