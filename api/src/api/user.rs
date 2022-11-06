use crate::crud;
use crate::crud::user::{ChangeUser, CreateUser, GetUser};
use crate::util::error::MyError;
use crate::util::middleware::extractor::Authenticated;
use crate::util::middleware::SecurityLevel;

use actix_web::web::{Data, Json, Path};
use paperclip::actix::{api_v2_operation, delete, get, post, AcceptedJson};
use paperclip::actix::{put, CreatedJson};
use sea_orm::DatabaseConnection;

#[api_v2_operation(summary = "Get Users")]
#[get("/users")]
pub async fn get_users(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<AcceptedJson<Vec<GetUser>>, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Worker {
        return Err(MyError::Unauthorized);
    }
    let users = crud::user::get_all_user(db.get_ref()).await;
    return if let Ok(users) = users {
        Ok(AcceptedJson(users))
    } else {
        Err(MyError::Unauthorized)
    };
}
#[api_v2_operation(summary = "Get Single User")]
#[get("/users/{user_id}")]
pub async fn get_single_user(
    db: Data<DatabaseConnection>,
    user_id: Path<String>,
    auth: Authenticated,
) -> actix_web::Result<AcceptedJson<GetUser>, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Worker {
        return Err(MyError::Unauthorized);
    }
    let users = crud::user::get_single_user(db.get_ref(), &user_id).await;
    return match users {
        Ok(users) => match users {
            Some(users) => Ok(AcceptedJson(users)),
            _ => Err(MyError::NotFound),
        },
        _ => Err(MyError::Unauthorized),
    };
}
#[api_v2_operation(summary = "Get self")]
#[get("/self")]
pub async fn get_self(
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<AcceptedJson<GetUser>, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::User {
        return Err(MyError::Unauthorized);
    }
    match &auth.try_get_user_id() {
        Some(user_id) => {
            let users = crud::user::get_single_user(db.get_ref(), user_id).await;
            match users {
                Ok(users) => match users {
                    Some(users) => Ok(AcceptedJson(users)),
                    _ => Err(MyError::NotFound),
                },
                _ => Err(MyError::NotFound),
            }
        }
        _ => Err(MyError::Unauthorized),
    }
}
#[api_v2_operation(summary = "Add User")]
#[post("/users")]
pub async fn add_user(
    user: Json<CreateUser>,
    auth: Authenticated,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<CreatedJson<GetUser>, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Admin {
        return Err(MyError::Unauthorized);
    }
    let user = crud::user::create_user(db.get_ref(), user.0).await;
    return if let Ok(users) = user {
        Ok(CreatedJson(users))
    } else {
        Err(MyError::Unauthorized)
    };
}
#[api_v2_operation(summary = "Update User")]
#[put("/users/{user_id}")]
pub async fn update_user(
    auth: Authenticated,
    user_id: actix_web::web::Path<String>,
    user: actix_web::web::Json<ChangeUser>,
    db: Data<DatabaseConnection>,
) -> actix_web::Result<AcceptedJson<GetUser>, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Admin || !auth.compare_user_id(user_id.as_str()) {
        return Err(MyError::Unauthorized);
    }
    let user = crud::user::update_user(db.get_ref(), user.0, &user_id.into_inner()).await;
    return if let Ok(users) = user {
        Ok(AcceptedJson(users))
    } else {
        Err(MyError::Unauthorized)
    };
}
#[api_v2_operation(summary = "Delete User")]
#[delete("/users/{user_id}")]
pub async fn delete_user(
    user_id: actix_web::web::Path<String>,
    db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<paperclip::actix::NoContent, MyError> {
    if auth.to_sercurity_level() < SecurityLevel::Admin {
        return Err(MyError::Unauthorized);
    }
    let user = crud::user::delete_user(db.get_ref(), &user_id.into_inner()).await;
    return if let Ok(_users) = user {
        Ok(paperclip::actix::NoContent)
    } else {
        Err(MyError::Unauthorized)
    };
}
