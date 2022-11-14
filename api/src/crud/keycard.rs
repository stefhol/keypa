use chrono::{DateTime, Utc};
use entities::model::tbl_keycard;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::crud;
use crate::util::error::CrudError;

use super::user::GetUser;
#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct GetKeycard {
    pub keycard_id: Uuid,
    pub active: bool,
    pub user: GetUser,
    pub active_until: Option<DateTimeUtc>,
}
impl From<(&tbl_keycard::Model, &GetUser)> for GetKeycard {
    fn from((keycard, user): (&tbl_keycard::Model, &GetUser)) -> Self {
        let keycard = keycard.clone();
        Self {
            keycard_id: keycard.keycard_id,
            user: user.clone(),
            active: keycard.active,
            active_until: keycard.active_until.map(|f| DateTime::from_local(f, Utc)),
        }
    }
}
pub async fn get_keycards_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetKeycard>, CrudError> {
    let model = tbl_keycard::Entity::find()
        .filter(tbl_keycard::Column::UserId.eq(user_id.clone()))
        .all(db)
        .await?;
    let user = crud::user::get_single_user(db, user_id).await?;
    Ok(model.iter().map(|f| GetKeycard::from((f, &user))).collect())
}
pub async fn get_single_keycard(
    key_card_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetKeycard, CrudError> {
    let model = tbl_keycard::Entity::find_by_id(key_card_id.clone())
        .one(db)
        .await?;
    match &model {
        Some(model) => {
            let user = crud::user::get_single_user(db, &model.user_id).await?;
            Ok(GetKeycard::from((model, &user)))
        }
        None => Err(CrudError::NotFound),
    }
}
