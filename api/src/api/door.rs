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
    let keys =
        crud::access::get_building_by_user_id_with_only_authorized_doors(user_id, &db).await?;
    Ok(HttpResponse::Ok().json(keys))
}
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
#[get("/users/{user_id}/doors")]
pub async fn get_user_authorized_doors(
    db: Data<DatabaseConnection>,
    user_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let keys =
        crud::access::get_building_by_user_id_with_only_authorized_doors(&user_id, &db).await?;
    Ok(HttpResponse::Ok().json(keys))
}
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
#[get("/request/{request_id}/doors")]
pub async fn get_doors_of_door_group(
    db: Data<DatabaseConnection>,
    request_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let request = crud::request::get::get_single_request(&db, &request_id).await?;
    let keys = crud::access::get_building_by_user_id_with_only_authorized_doors(
        &request.requester_id,
        &db,
    )
    .await?;
    Ok(HttpResponse::Ok().json(keys))
}
