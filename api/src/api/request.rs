use actix_web::{
    get, put,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    crud::{
        self,
        request::create::{create_request, CreateRequest},
    },
    util::{
        error::CrudError,
        middleware::{extractor::Authenticated, SecurityLevel},
    },
};

#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetRequestWithComments]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/self/request")]
pub async fn get_self_requests(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = auth.try_get_user_id()?;
    let requests = crud::request::get::get_request_from_user_id(&user_id, db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(requests))
}

#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetRequestWithComments]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/user/{user_id}/request")]
pub async fn get_requests_from_user(
    db: Data<DatabaseConnection>,
    user_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let requests = crud::request::get::get_request_from_user_id(&user_id, &db).await?;
    Ok(HttpResponse::Ok().json(requests))
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuery {
    request_id: Uuid,
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = GetRequestWithComments),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/self/request/{request_id}")]
pub async fn get_self_requests_from_request_id(
    db: Data<DatabaseConnection>,
    request_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = auth.try_get_user_id()?;
    let request = crud::request::get::get_request_from_user_id_and_request_id(
        &user_id,
        &request_id,
        db.get_ref(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(request))
}

#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = GetRequestWithComments),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/user/{user_id}/request")]
pub async fn get_single_requests_from_user(
    db: Data<DatabaseConnection>,
    user_id: Path<Uuid>,
    query: Query<RequestQuery>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let request = crud::request::get::get_request_from_user_id_and_request_id(
        &user_id,
        &query.request_id,
        db.get_ref(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(request))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetRequestWithComments]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/request")]
pub async fn get_all_pending_requests(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let request = crud::request::get::get_all_open_requests(&db).await?;
    Ok(HttpResponse::Ok().json(request))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = GetRequestWithComments),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/request/{request_id}")]
pub async fn get_single_requests(
    db: Data<DatabaseConnection>,
    request_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let request = crud::request::get::get_single_request(&db, &request_id).await?;
    Ok(HttpResponse::Ok().json(request))
}
#[utoipa::path(
    context_path = "/api/v1",
    request_body = CreateRequest,
    responses(
    (status = 200),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[put("/request")]
pub async fn create_requests(
    db: Data<DatabaseConnection>,
    request: Json<CreateRequest>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = auth.try_get_user_id()?;
    create_request(&db, &user_id, &request, auth.to_sercurity_level()).await?;
    Ok(HttpResponse::Ok().json(request))
}
