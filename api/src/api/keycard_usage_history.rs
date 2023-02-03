use actix_web::{get, web::Data, HttpResponse};
use sea_orm::{DatabaseConnection };

use crate::{util::{
    error::CrudError,
    middleware::{extractor::Authenticated, SecurityLevel},
}, crud::keycard_usage_history::query_keycard_usage};

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
