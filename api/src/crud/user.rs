use crate::util::error::CrudError;
use entities::model::{tbl_user};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct GetUser {
    pub user_id: Uuid,
    pub name: String,
pub role_id: Option<i64>,
    pub email: String,
    pub tel: Option<String>,
    pub address: Option<String>,
    pub picture_url: Option<String>,
}

impl From<&tbl_user::Model> for GetUser {
    fn from(user: &tbl_user::Model) -> Self {
        Self {
            user_id: user.user_id.clone(),
            name: user.name.to_string(),
            role_id: user.role_id,
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
    return match user.role_id {
        Some(role) => Ok(role == 1),
        None => Ok(false),
    };
}
pub async fn is_worker_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let user = get_single_user(user_id, db).await?;
    return match user.role_id {
        Some(role) => Ok(role == 3),
        None => Ok(false),
    };
}
pub async fn is_leader_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let user = get_single_user(user_id, db).await?;
    return match user.role_id {
        Some(role) => Ok(role == 4),
        None => Ok(false),
    };
}
