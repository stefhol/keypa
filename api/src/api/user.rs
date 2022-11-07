use crate::crud;
use crate::crud::user::{ChangeUser, CreateUser};
use crate::util::error::CrudError;
use crate::util::middleware::extractor::Authenticated;
use crate::util::middleware::SecurityLevel;

use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, put, HttpResponse};
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
    let users = crud::user::get_single_user(db.get_ref(), &user_id).await?;
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
    let users = crud::user::get_single_user(db.get_ref(), user_id).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
    context_path = "/api/v1",
    request_body = CreateUser,

    responses(
    (status = 201, body = [GetUser]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[post("/users")]
pub async fn add_user(
    user: Json<CreateUser>,
    auth: Authenticated,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Admin)?;
    let user = crud::user::create_user(db.get_ref(), user.0).await?;
    Ok(HttpResponse::Created().json(user))
}
#[utoipa::path(
    context_path = "/api/v1",
    request_body = ChangeUser,

    responses(
    (status = 200, body = GetUser),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[put("/users/{user_id}")]
pub async fn update_user(
    auth: Authenticated,
    user_id: actix_web::web::Path<Uuid>,
    user: actix_web::web::Json<ChangeUser>,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Admin)?;

    let user = crud::user::update_user(db.get_ref(), user.0, &user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}
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
#[delete("/users/{user_id}")]
pub async fn delete_user(
    user_id: actix_web::web::Path<Uuid>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Admin)?;
    crud::user::delete_user(db.get_ref(), &user_id).await?;
    Ok(HttpResponse::Ok().finish())
}
