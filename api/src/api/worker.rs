use crate::crud;

use crate::crud::worker::{CreateWorker, GetWorker};
use crate::util::error::MyError;
use crate::util::middleware::extractor::Authenticated;
use crate::util::middleware::SecurityLevel;

use actix_web::web::{Data, Path};
use paperclip::actix::{api_v2_operation, delete, post, AcceptedJson, NoContent};
use paperclip::actix::{get, put};
use sea_orm::DatabaseConnection;

#[api_v2_operation(summary = "Add Worker")]
#[post("/users/{user_id}/worker")]
pub async fn add_worker(
    worker: actix_web::web::Json<CreateWorker>,
    user_id: Path<String>,
    auth: Authenticated,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<paperclip::actix::NoContent, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Admin {
        return Err(MyError::Unauthorized);
    }
    let users = crud::worker::create_worker_from_user_id(user_id.as_str(), &worker, &db).await;
    match users {
        Ok(_) => Ok(NoContent),
        _ => Err(MyError::NotFound),
    }
}
#[api_v2_operation(summary = "Update Worker")]
#[put("/users/{user_id}/worker")]
pub async fn update_worker(
    auth: Authenticated,
    user_id: actix_web::web::Path<String>,
    user: actix_web::web::Json<CreateWorker>,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<NoContent, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Admin {
        return Err(MyError::Unauthorized);
    }
    let user = crud::worker::update_worker_with_user_id(
        &user_id.into_inner(),
        &user.into_inner(),
        db.get_ref(),
    )
    .await;
    match user {
        Ok(_) => Ok(NoContent),
        _ => Err(MyError::NotFound),
    }
}
#[api_v2_operation(summary = "Delete Worker")]
#[delete("/users/{user_id}/worker")]
pub async fn delete_worker(
    user_id: actix_web::web::Path<String>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<paperclip::actix::NoContent, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Admin {
        return Err(MyError::Unauthorized);
    }
    let user = crud::worker::delete_worker_with_user_id(user_id.as_str(), db.get_ref()).await;
    match user {
        Ok(_users) => Ok(NoContent),
        _ => Err(MyError::NotFound),
    }
}
#[api_v2_operation(
    summary = "Get Worker",
    description = "",
    operation_id = "get_worker",
    tags("Api reference")
)]
#[get("/users/{user_id}/worker")]
pub async fn get_worker(
    user_id: actix_web::web::Path<String>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<AcceptedJson<GetWorker>, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Leader {
        return Err(MyError::Unauthorized);
    }
    let worker = crud::worker::get_worker_by_user_id(db.get_ref(), user_id.as_str()).await;
    match worker {
        Ok(worker) => Ok(AcceptedJson(worker)),
        _ => Err(MyError::NotFound),
    }
}
#[api_v2_operation]
#[get("/self/worker")]
pub async fn get_self(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<AcceptedJson<GetWorker>, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Worker {
        return Err(MyError::Unauthorized);
    }
    match auth.try_get_user_id() {
        Some(user_id) => {
            let worker = crud::worker::get_worker_by_user_id(db.get_ref(), &user_id).await;
            match worker {
                Ok(worker) => Ok(AcceptedJson(worker)),
                _ => Err(MyError::NotFound),
            }
        }
        _ => Err(MyError::Unauthorized),
    }
}
