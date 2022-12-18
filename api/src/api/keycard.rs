use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    crud,
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
