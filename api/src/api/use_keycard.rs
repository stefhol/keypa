use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse,
};
use chrono::Local;
use entities::model::tbl_keycard_usage_history;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbBackend, FromQueryResult, Set, Statement};
use serde::{Deserialize, Serialize};
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
    let success = is_sucess(db, &data).await?;
    tbl_keycard_usage_history::ActiveModel {
        keycard_id: Set(data.keycard_id.to_owned()),
        door_id: Set(data.door_id.to_owned()),
        used_at: Set(Local::now().naive_utc()),
        success: Set(success),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(HttpResponse::Ok().body(format!("{success}")))
}
/// find out if access is allowed
/// to determine a user_id has to have a keycard_id and a door_id that matches our incoming data
async fn is_sucess(db: &DatabaseConnection, data: &UseKeycard) -> Result<bool, CrudError> {
    let query = query_active(db).await?;
    let temp:Vec<_> = query.iter().filter(|f|f.door_id.is_some() && f.keycard_id.is_some()).collect();
    let base: Vec<_> = query.iter().filter(|f|f.door_id.is_some() != f.keycard_id.is_some()).collect();
    // Out of list user has to have one keycard and one door access
    fn check_base(data: &UseKeycard, query: Vec<&QueryActiveView> )-> bool {
        let keycard = query.iter().find(|f|
            f.keycard_id == Some(data.keycard_id) 
        );
        if let Some(keycard) = keycard{
              
             query.iter()
                .filter(|f|f.user_id == keycard.user_id)
             .any(|f|
                {f.door_id == Some(data.door_id)} 
            )
        }
        else{
            false
        }
        
    }
    // Has to be one entry that have both 
    fn check_temp(data: &UseKeycard, query: Vec<&QueryActiveView> )-> bool {
        query.iter().any(|f|
            f.keycard_id == Some(data.keycard_id) &&
            f.door_id== Some(data.door_id)
        )
    }
    Ok(check_temp(data, temp) || check_base(data, base))
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
