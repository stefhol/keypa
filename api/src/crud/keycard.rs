use chrono::{DateTime, Utc};
use entities::model::tbl_keycard;
use sea_orm::{DatabaseConnection, DbBackend, EntityTrait, Statement};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetKeycard {
    pub keycard_id: Uuid,
    pub user_id: Uuid,
    pub is_lost: bool,
    pub is_locked: bool,
    pub is_deactivated: bool,
    pub is_given_back: bool,
    pub request_id: Option<Uuid>,
    pub given_out: Option<DateTime<Utc>>,
}
impl From<&tbl_keycard::Model> for GetKeycard {
    fn from(model: &tbl_keycard::Model) -> Self {
        let keycard = model.clone();
        Self {
            keycard_id: keycard.keycard_id,
            is_lost: keycard.is_lost,
            is_locked: keycard.is_locked,
            is_deactivated: keycard.is_deactivated,
            is_given_back: keycard.is_given_back,
            request_id: keycard.request_id,
            user_id: keycard.user_id,
            given_out: keycard.given_out.map(|f| DateTime::from_utc(f, Utc)),
        }
    }
}
async fn get_keycard_query(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<Vec<GetKeycard>, CrudError> {
    let values = tbl_keycard::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select (tk.*) from tbl_user
            join tbl_request tr on tbl_user.user_id = tr.requester_id
            join tbl_keycard tk on tr.keycard_id = tk.keycard_id
            where tbl_user.user_id = $1
            and tr.accept = true
            and tr.active = true
            "#,
            vec![user_id.clone().into()],
        ))
        .all(db)
        .await?;
    Ok(values.iter().map(|f| f.into()).collect())
}

pub async fn get_keycards_from_user(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<Vec<GetKeycard>, CrudError> {
    Ok(get_keycard_query(db, user_id).await?)
}
