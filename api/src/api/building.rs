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
    (status = 200 ,body = [GetCompleteBuilding]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/buildings")]
pub async fn get_buldings(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let buildings = crud::building::get_building_complex(&db).await?;

    Ok(HttpResponse::Ok().json(buildings))
}
