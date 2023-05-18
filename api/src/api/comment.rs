use actix_web::{
    get, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    crud::{self, comment::InsertComment},
    util::{
        error::CrudError,
        middleware::{extractor::Authenticated, SecurityLevel},
    },
};

#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetComment]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/request/{request_id}/comment")]
pub async fn get_comments(
    db: Data<DatabaseConnection>,
    request_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let comments = crud::comment::get_comments_of_request_id(&db, &request_id).await?;
    Ok(HttpResponse::Ok().json(comments))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[put("/request/{request_id}/comment")]
pub async fn insert_comment(
    db: Data<DatabaseConnection>,
    insert_comment: Json<InsertComment>,
    request_id: Path<Uuid>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = auth.try_get_user_id()?;
    let comments =
        crud::comment::insert_comment_into_request_id(&db, &user_id, &request_id, &insert_comment)
            .await?;

    Ok(HttpResponse::Ok().json(comments))
}
