use actix_web::{get, web::Data, HttpResponse};
use sea_orm::DatabaseConnection;

use crate::{
    crud::log::{get_all_logs, get_all_logs_raw},
    util::{
        error::CrudError,
        middleware::{extractor::Authenticated, SecurityLevel},
    },
};

#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetLogs]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/logs")]
pub async fn get_logs(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let logs = get_all_logs(&db).await?;
    Ok(HttpResponse::Ok().json(logs))
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
#[get("/csv/logs")]
pub async fn get_logs_as_csv(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let logs = get_all_logs_raw(&db).await?;

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(';' as u8)
        .from_writer(vec![]);

    for log in logs {
        wtr.serialize(log)?;
    }
    let string = String::from_utf8(wtr.into_inner().unwrap())?;
    Ok(HttpResponse::Ok().body(string))
}
