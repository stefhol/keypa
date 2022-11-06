use actix_web::{
    cookie::{time::Duration, Cookie},
    web::Data,
};
use log::error;
use paperclip::actix::{
    api_v2_operation, get,
    web::{HttpResponse, Json},
    Apiv2Schema, NoContent,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    crud::{
        self,
        user::is_admin_by_user_id,
        worker::{is_leader_by_user_id, is_worker_by_user_id},
    },
    util::{crypto::create_jwt, error::MyError, middleware::extractor::Authenticated},
};
#[derive(Apiv2Schema, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}
#[api_v2_operation]
#[get("/login")]
pub async fn login(
    db: Data<DatabaseConnection>,
    login: Json<Login>,
    // ) -> actix_web::Result<HttpResponse, MyError> {
) -> actix_web::Result<HttpResponse, MyError> {
    let model = crud::user::get_user_by_email(&db, &login.email).await;
    if let Ok(Some(user)) = model {
        let password =
            orion::pwhash::Password::from_slice(&login.password.to_string().into_bytes())
                .map_err(|f| {
                    error!("{}", f);
                    f
                })
                .unwrap();
        if let Ok(user_password) = orion::pwhash::PasswordHash::from_encoded(&user.password) {
            if let Err(err) = orion::pwhash::hash_password_verify(&user_password, &password) {
                error!("{}", err);
            } else {
                let user_id = &user.user_id.to_string();
                let is_worker = is_worker_by_user_id(user_id, &db).await.unwrap_or(false);
                let is_admin = is_admin_by_user_id(user_id, &db).await.map_err(|f| {
                    error!("{}", f);
                    f
                });
                let is_leader = is_leader_by_user_id(user_id, &db).await.unwrap_or(false);
                let token = create_jwt(
                    &user.user_id.to_string(),
                    is_admin
                        .map_err(|f| {
                            error!("{}", f);
                            f
                        })
                        .unwrap_or(false),
                    is_worker,
                    is_leader,
                );
                if let Ok(token) = token {
                    return Ok(HttpResponse::Ok()
                        //same cookie
                        //for ui state
                        .cookie(
                            Cookie::build("token", &token)
                                .max_age(Duration::hours(8))
                                .finish(),
                        )
                        //used in auth on server
                        //securer through http only
                        .cookie(
                            Cookie::build("bearer", &token)
                                .http_only(true)
                                .max_age(Duration::hours(8))
                                .finish(),
                        )
                        .finish());
                }
            }
        }
    }
    Err(MyError::Unauthorized)
    // HttpResponseWrapper(HttpResponse::Unauthorized().finish())
}
#[api_v2_operation]
#[get("/register")]
pub async fn register(
    _db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<paperclip::actix::NoContent, MyError> {
    println!("{:?}", auth);
    Ok(NoContent)
}
