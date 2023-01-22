use actix_web::{get, web::Data, HttpResponse};
use sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::{
    error::CrudError,
    middleware::{extractor::Authenticated, SecurityLevel},
};

#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [KeycardUsageHistory]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/keycard-usage-history")]
pub async fn get_keycard_usage_history(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let usage_history = query_keycard_usage(&db).await?;
    Ok(HttpResponse::Ok().json(usage_history))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = String),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/csv/keycard-usage-history")]
pub async fn get_csv_keycard_usage_history(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let usage_history = query_keycard_usage(&db).await?;

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(';' as u8)
        .from_writer(vec![]);

    for history in usage_history {
        wtr.serialize(history)?;
    }
    let string = String::from_utf8(wtr.into_inner().unwrap())?;
    Ok(HttpResponse::Ok().body(string))
}
#[derive(Debug, Clone, ToSchema, FromQueryResult, Serialize, Deserialize)]
pub struct KeycardUsageHistory {
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
    let query_result = KeycardUsageHistory::find_by_statement(
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
    Ok(query_result)
}
