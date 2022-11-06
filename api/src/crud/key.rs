use chrono::{DateTime, Utc};
use entities::model::{tbl_key, tbl_key_user_history};
use sea_orm::{
    prelude::DateTimeUtc, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, QueryFilter,
    Statement,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::{door::GetDoor, user::GetUser};
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetKey {
    pub key_id: Uuid,
    pub name: String,
    pub value: String,
    pub description: Option<String>,
    pub door_id: Uuid,
    pub door: Option<GetDoor>,
}
impl From<&tbl_key::Model> for GetKey {
    fn from(key: &tbl_key::Model) -> Self {
        let key = key.clone();
        Self {
            key_id: key.key_id,
            name: key.name,
            value: key.value,
            description: key.description,
            door_id: key.door_id,
            door: None,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetKeyHistory {
    pub key_id: Uuid,
    pub key: Option<GetKey>,
    pub user_id: Uuid,
    pub user: Option<GetUser>,
    pub due_at: Option<DateTimeUtc>,
    pub lent_at: Option<DateTimeUtc>,
    pub lent: Option<bool>,
    pub is_active: Option<bool>,
    pub has_problem: Option<bool>,
    pub comment: Option<String>,
}
impl From<&tbl_key_user_history::Model> for GetKeyHistory {
    fn from(key_history: &tbl_key_user_history::Model) -> Self {
        Self {
            key_id: key_history.key_id,
            key: None,
            user_id: key_history.user_id,
            user: None,
            due_at: key_history.lent_at.map(|f| DateTime::from_local(f, Utc)),
            lent_at: key_history.lent_at.map(|f| DateTime::from_local(f, Utc)),
            lent: key_history.lent,
            is_active: key_history.is_active,
            has_problem: key_history.has_problem,
            comment: key_history.comment.clone(),
        }
    }
}
pub async fn get_key_history_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetKeyHistory>, CrudError> {
    let key = tbl_key_user_history::Entity::find()
        .filter(tbl_key_user_history::Column::UserId.eq(user_id.clone()))
        .all(db)
        .await?;

    Ok(key.iter().map(|f| f.into()).collect())
}
pub async fn get_key_by_key_id(
    key_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetKey, CrudError> {
    let key = tbl_key::Entity::find_by_id(key_id.clone()).one(db).await?;

    match key {
        Some(key) => Ok((&key).into()),
        None => Err(CrudError::NotFound),
    }
}
pub async fn get_keys_of_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetKey>, CrudError> {
    let values = tbl_key::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select (tbl_key.*) from tbl_user
            join tbl_key_group on tbl_user.user_id = tbl_key_group.owner_id
            join tbl_key_group_key on tbl_key_group.key_group_id = tbl_key_group_key.key_group_id
            join tbl_key on tbl_key_group_key.key_id = tbl_key.key_id
            where user_id = $1"#,
            vec![user_id.clone().into()],
        ))
        .all(db)
        .await?;
    Ok(values.iter().map(|f| f.into()).collect())
}
