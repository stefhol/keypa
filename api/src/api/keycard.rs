use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    crud::{self, keycard::ChangeKeyboard},
    util::{
        error::CrudError,
        middleware::{extractor::Authenticated, SecurityLevel},
    },
};

#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetKeycard]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/self/keycard")]
pub async fn get_self_keycard(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = auth.try_get_user_id()?;
    let requests = crud::keycard::get_keycards_from_user(&db, &user_id).await?;
    Ok(HttpResponse::Ok().json(requests))
}

#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetKeycard]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/user/{user_id}/keycard")]
pub async fn get_user_keycard(
    db: Data<DatabaseConnection>,
    user_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let requests = crud::keycard::get_keycards_from_user(&db, &user_id).await?;
    Ok(HttpResponse::Ok().json(requests))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = GetKeycard),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/keycard/{keycard_id}")]
pub async fn get_single_request_keycard(
    db: Data<DatabaseConnection>,
    keycard_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let requests = crud::keycard::get_single_keycard(&db, &keycard_id).await?;
    Ok(HttpResponse::Ok().json(requests))
}
#[utoipa::path(
    context_path = "/api/v1",
    request_body = ChangeKeycard,
    responses(
    (status = 200),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[post("/keycard/{keycard_id}")]
pub async fn change_keycard(
    db: Data<DatabaseConnection>,
    keycard_id: Path<Uuid>,
    change_keycard: Json<ChangeKeyboard>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let worker_id = auth.try_get_user_id()?;
    crud::keycard::change_keycard(&worker_id, &db, &keycard_id, &change_keycard).await?;
    Ok(HttpResponse::Ok().finish())
}
