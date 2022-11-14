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
    crud::{
        self,
        door_group::{ChangeKeyGroup, CreateKeyGroup},
    },
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
    (status = 200 ,body = [GetKeyGroup]),
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
        Some(user_id) => crud::door_group::get_door_group_of_user(&db, &user_id).await?,
        None => crud::door_group::get_all_door_group(&db).await?,
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
<<<<<<< HEAD:api/src/api/door_group.rs
#[post("/key-group/{door_group_id}")]
=======
#[post("/key-group/{key_group_id}")]
>>>>>>> 9560445 (minor fixes):api/src/api/key_group.rs
pub async fn add_key_into_key_group(
    door_group_id: Path<Uuid>,
    query: Query<KeyGroupQuery>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::Worker)?;
    crud::door_group::add_door_to_door_group(&query.key_id, &door_group_id, &db).await?;
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
#[delete("/key-group/{door_group_id}")]
pub async fn delete_key_from_key_group(
    door_group_id: Path<Uuid>,
    query: Query<KeyGroupQuery>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    is_self_or_security_level(SecurityLevel::Worker, &auth, &door_group_id, &db).await?;

    crud::door_group::remove_key_from_key_group(&query.key_id, &door_group_id, &db).await?;
    Ok(HttpResponse::Ok().finish())
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
    (status = 200, body = [GetKey]),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[get("/key-group/{door_group_id}")]
pub async fn get_keys_of_key_group(
    door_group_id: Path<Uuid>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    is_self_or_security_level(SecurityLevel::Worker, &auth, &door_group_id, &db).await?;

    let keys = crud::door_group::get_doors_of_door_group(&db, &door_group_id).await?;
    Ok(HttpResponse::Ok().json(keys))
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
#[get("/self/key-group")]
pub async fn get_self_key_group(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let user_id = auth.try_get_user_id()?;
    let door_groups = crud::door_group::get_door_group_of_user(&db, &user_id).await?;
    Ok(HttpResponse::Ok().json(door_groups))
}
#[utoipa::path(
    context_path = "/api/v1",
    request_body = CreateKeyGroup,
    responses(
    (status = 200, body = GetKeyGroup),
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
    door_group: Json<CreateKeyGroup>,
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    auth.has_high_enough_security_level(SecurityLevel::User)?;
    let door_groups = crud::door_group::create_door_group(&door_group, &db).await?;
    Ok(HttpResponse::Ok().json(door_groups))
}
#[utoipa::path(
    context_path = "/api/v1",
    request_body = ChangeKeyGroup,
    responses(
    (status = 200, body = GetKeyGroup),
    (status = 400),
    (status = 401),
    (status = 404),
    (status = 406),
    (status = 500),
)
)]
#[put("/key-group/{door_group_id}")]
pub async fn upate_key_group(
    db: Data<DatabaseConnection>,
<<<<<<< HEAD:api/src/api/door_group.rs
    door_group_id: Path<Uuid>,
    door_group: Json<ChangeKeyGroup>,
=======
    key_group_id: Path<Uuid>,
    key_group: Json<ChangeKeyGroup>,
>>>>>>> 9560445 (minor fixes):api/src/api/key_group.rs
    auth: Authenticated,
) -> actix_web::Result<HttpResponse, CrudError> {
    is_self_or_security_level(SecurityLevel::Worker, &auth, &door_group_id, &db).await?;

    let door_groups = crud::door_group::update_door_group(&door_group, &door_group_id, &db).await?;
    Ok(HttpResponse::Ok().json(door_groups))
}
///Check if security_level is high enough or query database to check if it is the object of a user
async fn is_self_or_security_level(
    security_level: SecurityLevel,
    auth: &Authenticated,
    door_group_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<(), CrudError> {
    match auth.has_high_enough_security_level(security_level) {
        Ok(_) => {}
        Err(_) => {
            let user_id = auth.try_get_user_id()?;
            if !crud::door_group::get_door_group_of_user(&db, &user_id)
                .await?
                .iter()
                .any(|f| door_group_id.to_string() == f.door_group_id.to_string())
            {
                return Err(CrudError::Unauthorized);
            }
        }
    }
    Ok(())
}
