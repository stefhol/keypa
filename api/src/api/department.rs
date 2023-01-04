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
    (status = 200, body = [GetDepartment]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/department")]
pub async fn get_departments(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let keys = crud::department::get_department(&db).await?;
    Ok(HttpResponse::Ok().json(keys))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetDepartment]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/self/department")]
pub async fn get_departments_of_self(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = auth.try_get_user_id()?;
    let keys = crud::department::get_department_of_user_id(&db, &user_id).await?;
    Ok(HttpResponse::Ok().json(keys))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetDepartment]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/users/{user_id}/department")]
pub async fn get_departments_of_user(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
    user_id: Path<Uuid>,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    let keys = crud::department::get_department_of_user_id(&db, &user_id).await?;
    Ok(HttpResponse::Ok().json(keys))
}
