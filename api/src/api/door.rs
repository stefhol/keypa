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
#[get("/self/doors/{keycard_id}")]
pub async fn get_self_door_keycard(
    db: Data<DatabaseConnection>,
    keycard_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = &auth.try_get_user_id()?;
    let keys = crud::access::get_building_by_user_id_and_keycard_id_with_only_authorized_doors(
        &user_id,
        &keycard_id,
        &db,
    )
    .await?;
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
#[get("/users/{user_id}/doors/{keycard_id}")]
pub async fn get_user_authorized_doors_keycard(
    db: Data<DatabaseConnection>,
    user_id: Path<Uuid>,
    keycard_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let keys = crud::access::get_building_by_user_id_and_keycard_id_with_only_authorized_doors(
        &user_id,
        &keycard_id,
        &db,
    )
    .await?;
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

    let keys =
        crud::access::get_building_by_request_id_only_authorized_doors(&request_id, &db).await?;
    Ok(HttpResponse::Ok().json(keys))
}
