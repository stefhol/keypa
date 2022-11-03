use crate::crud;
use crate::crud::user::{ChangeUser, CreateUser, GetUser};
use crate::util::error::MyError;

use actix_web::web::Data;
use paperclip::actix::{api_v2_operation, delete, get, post, AcceptedJson};
use paperclip::actix::{put, CreatedJson};
use sea_orm::DatabaseConnection;

#[api_v2_operation(
    summary = "My awesome handler",
    description = "It creates a pretty JSON object",
    /// A few other parameters are also supported
    operation_id = "get_user",
    produces = "application/yaml, application/json",
    tags("Api reference"),
)]
#[get("/users")]
pub async fn get_users(
    db: Data<DatabaseConnection>,
) -> actix_web::Result<AcceptedJson<Vec<GetUser>>, MyError> {
    let users = crud::user::get_all_user(db.get_ref()).await;

    return if let Ok(users) = users {
        Ok(AcceptedJson(users))
    } else {
        Err(MyError::Unauthorized)
    };
}
#[api_v2_operation(
    summary = "Add new User",
    description = "Adds new User to Database",
    operation_id = "add_user",
    consumes = "application/yaml, application/json",
    produces = "application/yaml, application/json",
    tags("Api reference")
)]
#[post("/users")]
pub async fn add_user(
    user: actix_web::web::Json<CreateUser>,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<CreatedJson<GetUser>, MyError> {
    let user = crud::user::create_user(db.get_ref(), user.0).await;
    return if let Ok(users) = user {
        Ok(CreatedJson(users))
    } else {
        Err(MyError::Unauthorized)
    };
}
#[api_v2_operation(
    summary = "Update User",
    description = "Update User in Database",
    operation_id = "update_user",
    consumes = "application/yaml, application/json",
    produces = "application/yaml, application/json",
    tags("Api reference")
)]
#[put("/users/{id}")]
pub async fn update_user(
    id: actix_web::web::Path<String>,
    user: actix_web::web::Json<ChangeUser>,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<AcceptedJson<GetUser>, MyError> {
    let user = crud::user::update_user(db.get_ref(), user.0, id.into_inner()).await;
    return if let Ok(users) = user {
        Ok(AcceptedJson(users))
    } else {
        Err(MyError::Unauthorized)
    };
}
#[api_v2_operation(
    summary = "Update User",
    description = "Update User in Database",
    operation_id = "update_user",
    tags("Api reference")
)]
#[delete("/users/{id}")]
pub async fn delete_user(
    id: actix_web::web::Path<String>,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<paperclip::actix::NoContent, MyError> {
    let user = crud::user::delete_user(db.get_ref(), id.into_inner()).await;
    return if let Ok(_users) = user {
        Ok(paperclip::actix::NoContent)
    } else {
        Err(MyError::Unauthorized)
    };
}
