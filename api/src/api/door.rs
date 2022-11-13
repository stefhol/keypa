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
    (status = 200, body=[GetCompleteBuilding]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/self/doors")]
pub async fn get_self_door(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = &auth.try_get_user_id()?;
    let keys = crud::access::get_building_with_only_authorized_doors(user_id, &db).await?;
    Ok(HttpResponse::Ok().json(keys))
}
