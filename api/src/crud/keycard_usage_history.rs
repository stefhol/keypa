use chrono::{DateTime, Utc};
use sea_orm::{FromQueryResult, DatabaseConnection, Statement, DbBackend};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub struct KeycardUsageHistory {
    pub keycard_history_id: i64,
    pub keycard_id: Uuid,
    pub door_id: Uuid,
    pub used_at: DateTime<Utc>,
    pub success: bool,
    pub room_id: Uuid,
    pub room_name: String,
    pub room_floor: i32,
    pub is_sensitive: Option<bool>,
    pub building_id: Uuid,
    pub building_name: String,
    pub username: Option<String>,
    pub role_id: Option<i64>,
    pub user_id: Option<Uuid>,
}
impl From<&Query> for KeycardUsageHistory {
    fn from(value: &Query) -> Self {
        let value = value.clone();
        Self {
            keycard_history_id: value.keycard_history_id,
            keycard_id: value.keycard_id,
            door_id: value.door_id,
            used_at: DateTime::from_local(value.used_at,Utc),
            success: value.success,
            room_id: value.room_id,
            room_name: value.room_name,
            room_floor: value.room_floor,
            is_sensitive: value.is_sensitive,
            building_id: value.building_id,
            building_name: value.building_name,
            username: value.username,
            role_id: value.role_id,
            user_id: value.user_id,
        }
    }
}
#[derive(Debug, Clone, FromQueryResult, Serialize, Deserialize)]
pub struct Query {
    pub keycard_history_id: i64,
    pub keycard_id: Uuid,
    pub door_id: Uuid,
    pub used_at: sea_orm::prelude::DateTime,
    pub success: bool,
    pub room_id: Uuid,
    pub room_name: String,
    pub room_floor: i32,
    pub is_sensitive: Option<bool>,
    pub building_id: Uuid,
    pub building_name: String,
    pub username: Option<String>,
    pub role_id: Option<i64>,
    pub user_id: Option<Uuid>,
}
pub(crate) async fn query_keycard_usage(
    db: &DatabaseConnection,
) -> Result<Vec<KeycardUsageHistory>, CrudError> {
    let query_result = Query::find_by_statement(
        Statement::from_sql_and_values(DbBackend::Postgres,
        r#"
        select tbl_keycard_usage_history.*, td.room_id, tr.name as room_name, floor as room_floor, is_sensitive, tr.building_id as building_id, tb.name as building_name, tu.name as username, role_id, tu.user_id as user_id  
        from tbl_keycard_usage_history
        join tbl_door td on td.door_id = tbl_keycard_usage_history.door_id
        join tbl_room tr on td.room_id = tr.room_id
        join tbl_building tb on tr.building_id = tb.building_id
        left join tbl_keycard tk on tbl_keycard_usage_history.keycard_id = tk.keycard_id
        left join tbl_user tu on tk.user_id = tu.user_id
        order by  tbl_keycard_usage_history.used_at DESC 
        "#,
        vec![],)
    ).all(db).await?;
    Ok(query_result.iter().map(|f|f.into()).collect())
}
