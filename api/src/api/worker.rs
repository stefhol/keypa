use crate::crud;

use crate::crud::worker::CreateWorker;
use crate::util::error::{CrudError};
use crate::util::middleware::extractor::Authenticated;
use crate::util::middleware::SecurityLevel;

use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, put, HttpResponse};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
#[utoipa::path(
    context_path = "/api/v1",
    request_body = CreateWorker,
    
    responses(
    (status = 201),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[post("/users/{user_id}/worker")]
pub async fn add_worker(
    worker: actix_web::web::Json<CreateWorker>,
    user_id: Path<Uuid>,
    auth: Authenticated,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Admin)?;
    crud::worker::create_worker_from_user_id(&user_id, &worker, &db).await?;
    Ok(HttpResponse::Created().finish())
    
}
#[utoipa::path(
    context_path = "/api/v1",
    request_body = CreateWorker,
    
    responses(
    (status = 200),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[put("/users/{user_id}/worker")]
pub async fn update_worker(
    auth: Authenticated,
    user_id: actix_web::web::Path<Uuid>,
    user: actix_web::web::Json<CreateWorker>,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Admin)?;
    
    crud::worker::update_worker_with_user_id(
        &user_id,
        &user.into_inner(),
        db.get_ref(),
    )
    .await?;
    Ok(HttpResponse::Ok().finish())
}
#[utoipa::path(
    context_path = "/api/v1",
    request_body = CreateWorker,
    
    responses(
    (status = 200),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[delete("/users/{user_id}/worker")]
pub async fn delete_worker(
    user_id: actix_web::web::Path<Uuid>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Admin)?;
    
    crud::worker::delete_worker_with_user_id(&user_id, db.get_ref()).await?;
    Ok(HttpResponse::Ok().finish())
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetWorker]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/users/{user_id}/worker")]
pub async fn get_worker(
    user_id: actix_web::web::Path<Uuid>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Leader)?;
    
    let worker = crud::worker::get_worker_by_user_id(db.get_ref(), &user_id).await?;
    Ok(HttpResponse::Ok().json(worker))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetWorker]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/self/worker")]
pub async fn get_self(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Leader)?;
    let user_id = auth.try_get_user_id()?;
    let worker = crud::worker::get_worker_by_user_id(db.get_ref(), &user_id).await?;
    Ok(HttpResponse::Ok().json(worker))    
}
