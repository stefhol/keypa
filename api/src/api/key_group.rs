use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    crud::{self, key_group::CreateKeyGroup},
    util::{
        error::CrudError,
        middleware::{extractor::Authenticated, SecurityLevel},
    },
};
#[derive(Debug, Deserialize, IntoParams)]
pub struct UserQuery {
    user_id: Option<Uuid>,
}
#[utoipa::path(
    params(UserQuery),
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
#[get("/key-group")]
pub async fn get_key_group(
    db: Data<DatabaseConnection>,
    user_query: Query<UserQuery>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let key_groups = match user_query.user_id {
        Some(user_id) => crud::key_group::get_key_group_of_user(&db, &user_id).await?,
        None => crud::key_group::get_all_key_group(&db).await?,
    };

    Ok(HttpResponse::Ok().json(key_groups))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct KeyGroupQuery {
    key_id: Uuid,
}
#[utoipa::path(
    params(KeyGroupQuery),
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
#[put("/key-group/{key_group_id}")]
pub async fn add_key_into_key_group(
    key_group_id: Path<Uuid>,
    query: Query<KeyGroupQuery>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    crud::key_group::add_key_to_key_group(&query.key_id, &key_group_id, &db).await?;
    Ok(HttpResponse::Ok().finish())
}
#[utoipa::path(
    params(KeyGroupQuery),
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
#[delete("/key-group/{key_group_id}")]
pub async fn delete_key_from_key_group(
    key_group_id: Path<Uuid>,
    query: Query<KeyGroupQuery>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    is_self_or_security_level(SecurityLevel::Worker, &auth, &key_group_id, &db).await?;

    crud::key_group::remove_key_from_key_group(&query.key_id, &key_group_id, &db).await?;
    Ok(HttpResponse::Ok().finish())
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [Vec<GetKey>]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/key-group/{key_group_id}")]
pub async fn get_keys_of_key_group(
    key_group_id: Path<Uuid>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    is_self_or_security_level(SecurityLevel::Worker, &auth, &key_group_id, &db).await?;

    let keys = crud::key_group::get_keys_of_key_group(&db, &key_group_id).await?;
    Ok(HttpResponse::Ok().json(keys))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [Vec<GetKeyGroup>]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/key-group/self")]
pub async fn get_self_key_group(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = auth.try_get_user_id()?;
    let key_groups = crud::key_group::get_key_group_of_user(&db, &user_id).await?;
    Ok(HttpResponse::Ok().json(key_groups))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetKeyGroup]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[post("/key-group")]
pub async fn add_key_group(
    db: Data<DatabaseConnection>,
    key_group: Json<CreateKeyGroup>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let key_groups = crud::key_group::create_key_group(&key_group, &db).await?;
    Ok(HttpResponse::Ok().json(key_groups))
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetKeyGroup]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[put("/key-group/{key_group_id}")]
pub async fn upate_key_group(
    db: Data<DatabaseConnection>,
    key_group_id: Path<Uuid>,
    key_group: Json<CreateKeyGroup>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    is_self_or_security_level(SecurityLevel::Worker, &auth, &key_group_id, &db).await?;

    let key_groups = crud::key_group::update_key_group(&key_group, &key_group_id, &db).await?;
    Ok(HttpResponse::Ok().json(key_groups))
}
///Check if security_level is high enough or query database to check if it is the object of a user
async fn is_self_or_security_level(
    security_level: SecurityLevel,
    auth: &Authenticated,
    key_group_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<(), CrudError> {
    match auth.has_high_enough_security_level(security_level) {
        Ok(_) => {}
        Err(_) => {
            let user_id = auth.try_get_user_id()?;
            if !crud::key_group::get_key_group_of_user(&db, &user_id)
                .await?
                .iter()
                .any(|f| key_group_id.to_string() == f.key_group_id.to_string())
            {
                return Err(CrudError::Unauthorized);
            }
        }
    }
    Ok(())
}
