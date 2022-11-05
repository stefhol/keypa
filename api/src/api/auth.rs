use actix_web::{
    cookie::{time::Duration, Cookie},
    web::{Data, Json},
    HttpResponse,
};
use log::error;
use paperclip::actix::{api_v2_operation, get, Apiv2Schema, HttpResponseWrapper, NoContent};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    crud::{
        self,
        user::is_admin_by_user_id,
        worker::{get_worker_by_user_id, is_leader_by_worker_id},
    },
    util::{crypto::create_jwt, error::MyError, middleware::extractor::Authenticated},
};
#[derive(Apiv2Schema, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

#[get("/login")]
pub async fn login(
    db: Data<DatabaseConnection>,
    login: Json<Login>,
) -> paperclip::actix::HttpResponseWrapper {
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
                let worker = get_worker_by_user_id(&db, &user.user_id.to_string()).await;
                let is_admin = is_admin_by_user_id(user.user_id, &db).await.map_err(|f| {
                    error!("{}", f);
                    f
                });
                let is_leader = match &worker {
                    Ok(Some(worker)) => is_leader_by_worker_id(&worker.worker_id, &db)
                        .await
                        .map_err(|f| {
                            error!("{}", f);
                            f
                        })
                        .unwrap_or(false),
                    _ => false,
                };
                let token = create_jwt(
                    &user.user_id.to_string(),
                    is_admin
                        .map_err(|f| {
                            error!("{}", f);
                            f
                        })
                        .unwrap_or(false),
                    worker
                        .map_err(|f| {
                            error!("{}", f);
                            f
                        })
                        .unwrap_or(None)
                        .is_some(),
                    is_leader,
                );
                if let Ok(token) = token {
                    return HttpResponseWrapper(
                        HttpResponse::Ok()
                            .cookie(
                                Cookie::build("token", token)
                                    .http_only(true)
                                    .max_age(Duration::hours(8))
                                    .finish(),
                            )
                            .finish(),
                    );
                }
            }
        }
    }
    HttpResponseWrapper(HttpResponse::Unauthorized().finish())
}
#[api_v2_operation(
    summary = "My awesome handler",
    description = "It creates a pretty JSON object",
    /// A few other parameters are also supported
    operation_id = "login",
    tags("Api reference"),
)]
#[get("/register")]
pub async fn register(
    _db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<paperclip::actix::NoContent, MyError> {
    println!("{:?}", auth);
    Ok(NoContent)
}
