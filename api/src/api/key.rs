use actix_web::{get, web::Data, HttpResponse};
use sea_orm::DatabaseConnection;

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
    (status = 200, body=[GetKey]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/self/keys")]
pub async fn get_self_key(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = &auth.try_get_user_id()?;
    let keys = crud::access::get_keys_of_user_id(user_id, db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(keys))
}
