use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse,
};
use chrono::{Local, Utc};
use entities::model::tbl_keycard_usage_history;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, Set, Statement,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[put("/use-keycard")]
pub async fn use_keycard(
    db: Data<DatabaseConnection>,
    data: Json<UseKeycard>,
) -> actix_web::Result<HttpResponse, CrudError> {
    let db: &DatabaseConnection = &db;

    tbl_keycard_usage_history::ActiveModel {
        keycard_id: Set(data.keycard_id.to_owned()),
        door_id: Set(data.door_id.to_owned()),
        used_at: Set(Local::now().naive_utc()),
        success: Set(is_sucess(db, &data).await?),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(HttpResponse::Ok().finish())
}
/// find out if access is allowed
/// to determine a user_id has to have a keycard_id and a door_id that matches our incoming data
async fn is_sucess(db: &DatabaseConnection, data: &UseKeycard) -> Result<bool, CrudError> {
    let query = query_active(db).await?;

    let keycard_query = query.iter().find(|f| f.keycard_id == Some(data.keycard_id));
    if let Some(keycard_query) = keycard_query {
        return Ok(query
            .iter()
            .filter(|f| f.user_id == keycard_query.user_id)
            .find(|f| f.door_id == Some(data.door_id))
            .is_some());
    }
    Ok(false)
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UseKeycard {
    door_id: Uuid,
    keycard_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromQueryResult)]
struct QueryActiveView {
    active_until: Option<sea_orm::prelude::DateTime>,
    door_id: Option<Uuid>,
    keycard_id: Option<Uuid>,
    user_id: Uuid,
}
async fn query_active(db: &DatabaseConnection) -> Result<Vec<QueryActiveView>, CrudError> {
    let query_result: Vec<QueryActiveView> =
        QueryActiveView::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
        select active_until, door_id, keycard_id, user_id  from view_active_keycards;
        "#,
            vec![],
        ))
        .all(db)
        .await?;
    Ok(query_result)
}
