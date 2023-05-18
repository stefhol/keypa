use crate::crud;
use crate::util::error::CrudError;
use crate::util::middleware::extractor::Authenticated;
use crate::util::middleware::SecurityLevel;

use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetUser]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
x
#[get("/users")]
pub async fn get_users(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let users = crud::user::get_all_user(db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(users))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = GetUser),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/users/{user_id}")]
pub async fn get_single_user(
    db: Data<DatabaseConnection>,
    user_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let users = crud::user::get_single_user(&user_id, &db).await?;
    Ok(HttpResponse::Ok().json(users))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = GetUser),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/self")]
pub async fn get_self(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = &auth.try_get_user_id()?;
    let users = crud::user::get_single_user(user_id, db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(users))
}
