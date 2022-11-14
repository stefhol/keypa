use crate::crud;
use crate::util::{
    error::CrudError,
    middleware::{extractor::Authenticated, SecurityLevel},
};
use actix_web::get;
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

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
#[get("user/{user_id}/keycard")]
pub async fn get_keycard_of_user_id(
    db: Data<DatabaseConnection>,
    user_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let keycard = crud::keycard::get_keycards_by_user_id(&user_id, &db).await?;
    Ok(HttpResponse::Ok().json(keycard))
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
#[get("/self/keycard")]
pub async fn get_self_keycard(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = auth.try_get_user_id()?;
    let keycard = crud::keycard::get_keycards_by_user_id(&user_id, &db).await?;
    Ok(HttpResponse::Ok().json(keycard))
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
pub async fn get_single_keycard(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
    key_card_id: Path<Uuid>,
) -> actix_web::Result<HttpResponse, CrudError> {
    //TODO: change to include self and worker
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let keycard = crud::keycard::get_single_keycard(&key_card_id, &db).await?;
    if &keycard.user.user_id == &auth.try_get_user_id()? {
        Ok(HttpResponse::Ok().json(keycard))
    } else {
        auth.has_high_enough_security_level(SecurityLevel::Worker)?;
        Ok(HttpResponse::Ok().json(keycard))
    }
}
