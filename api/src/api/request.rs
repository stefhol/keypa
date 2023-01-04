use actix_web::{
    get, put,
    web::{Data, Json, Path, Query},
    HttpResponse, post,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    crud::{
        self,
        request::{
            create::{create_request, CreateRequest},
            get::RequestType, change::ChangeRequest,
        },
    },
    util::{
        error::CrudError,
        middleware::{extractor::Authenticated, SecurityLevel},
    },
};

#[utoipa::path(
    context_path = "/api/v1",
    params(RequestQueryType),
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
    query: Query<RequestQueryType>,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let user_id = auth.try_get_user_id()?;
    let request:Vec<_> = match &query.0.request_status {
        Some(query) => match query {
            RequestStatus::Pending => crud::request::get::get_all_pending_requests(&db).await?.iter().filter(|f|f.requester_id == user_id.to_owned()).cloned().collect(),
            RequestStatus::Reject => crud::request::get::get_all_reject_requests(&db).await?.iter().filter(|f|f.requester_id == user_id.to_owned()).cloned().collect(),
            RequestStatus::Accept => crud::request::get::get_all_accepted_requests(&db).await?.iter().filter(|f|f.requester_id == user_id.to_owned()).cloned().collect(),
        },
        _ => crud::request::get::get_all_requests(&db).await?.iter().filter(|f|f.requester_id == user_id.to_owned()).cloned().collect(),
    };
    Ok(HttpResponse::Ok().json(request))
}

#[utoipa::path(
    context_path = "/api/v1",
    params(RequestQueryType),
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
    query: Query<RequestQueryType>,
    
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let request:Vec<_> = match &query.0.request_status {
        Some(query) => match query {
            RequestStatus::Pending => crud::request::get::get_all_pending_requests(&db).await?.iter().filter(|f|f.requester_id == user_id.to_owned()).cloned().collect(),
            RequestStatus::Reject => crud::request::get::get_all_reject_requests(&db).await?.iter().filter(|f|f.requester_id == user_id.to_owned()).cloned().collect(),
            RequestStatus::Accept => crud::request::get::get_all_accepted_requests(&db).await?.iter().filter(|f|f.requester_id == user_id.to_owned()).cloned().collect(),
        },
        _ => crud::request::get::get_all_requests(&db).await?.iter().filter(|f|f.requester_id == user_id.to_owned()).cloned().collect(),
    };
    Ok(HttpResponse::Ok().json(request))
}
#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct RequestQuery {
    request_id: Uuid,
}
#[derive(Debug, Serialize, Deserialize, IntoParams)]
#[serde(rename_all = "lowercase")]
pub struct RequestQueryType {
    request_status: Option<RequestStatus>,
    request_type: Option<RequestType>,
    is_sensitive:Option<bool>
}
#[derive(Debug, Serialize, Deserialize,ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RequestStatus {
    Pending,
    Reject,
    Accept,
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
    params(RequestQuery,RequestQueryType),
    
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
    params(RequestQueryType),
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
pub async fn get_all_requests(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
    query: Query<RequestQueryType>,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let request = match &query.0.request_status {
        Some(query) => match query {
            RequestStatus::Pending => crud::request::get::get_all_pending_requests(&db).await?,
            RequestStatus::Reject => crud::request::get::get_all_reject_requests(&db).await?,
            RequestStatus::Accept => crud::request::get::get_all_accepted_requests(&db).await?,
        },
        _ => crud::request::get::get_all_requests(&db).await?,
    };
    
    let request = match &query.request_type {
        Some(request_type) => request
            .iter()
            
            .filter(|f| &f.request_type == request_type)
            
            .cloned()
            .collect(),
        _ => request,
    };
    let request = match query.is_sensitive {
        Some(val) => request
            .iter()
            // filter because of query param
        .filter(|f| {
            f.is_sensitive == Some(val)
        })
        .cloned()
        .collect(),
        None => request
    };
    Ok(HttpResponse::Ok().json(request.iter()
        // filter to only show is_sensitive true to leader
    .filter(|f|{
        if auth.to_sercurity_level() != SecurityLevel::Leader{
            f.is_sensitive == Some(false)
        }else{
            true
        }
    }).collect::<Vec<_>>()))
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
#[utoipa::path(
    context_path = "/api/v1",
    request_body = ChangeRequest,
    responses(
    (status = 200),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[post("/request/{request_id}")]
pub async fn change_requests(
    db: Data<DatabaseConnection>,
    request: Json<ChangeRequest>,
    request_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    crud::request::change::change_request(&db, &request_id, &request, auth.to_sercurity_level()).await?;
    Ok(HttpResponse::Ok().json(request))
}
