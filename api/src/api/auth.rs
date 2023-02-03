use actix_web::{
    cookie::{time::Duration, Cookie},
    get, post,
    web::{Data, Json},
    HttpResponse,
};

use sea_orm::DatabaseConnection;
use serde::Deserialize;
use tracing::error;
use utoipa::ToSchema;

use crate::{
    crud::{
        self,
        user::{is_admin_by_user_id, is_leader_by_user_id, is_worker_by_user_id},
    },
    util::{crypto::create_jwt, error::CrudError},
};
#[derive(ToSchema, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}
#[utoipa::path(
    request_body = Login,
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
#[post("/login")]
pub async fn login(
    db: Data<DatabaseConnection>,
    login: Json<Login>,
) -> actix_web::Result<HttpResponse, CrudError> {
    let model = crud::user::get_user_by_email(&db, &login.email).await?;
    if let Some(user) = model {
        //create password hash from provided input password takes a long time
        let password =
            orion::pwhash::Password::from_slice(&login.password.to_string().into_bytes())
                .map_err(|f| {
                    error!("{}", f);
                    f
                })
                .unwrap();
                // use the encoded password from the db 
        if let Ok(user_password) = orion::pwhash::PasswordHash::from_encoded(&user.password) {
            // compare password and verify it
            if let Err(err) = orion::pwhash::hash_password_verify(&user_password, &password) {
                error!("{}", err);
            } else {
                //user is verified
                let user_id = &user.user_id;

                // prepare tokens
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
                // add token to response
                if let Ok(token) = token {
                    return Ok(HttpResponse::Ok()
                        //same cookie
                        //for ui state
                        .cookie(
                            Cookie::build("token", &token)
                                .max_age(Duration::hours(8))
                                .path("/")
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
    Err(CrudError::NotFound)
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
#[get("/logout")]
pub async fn logout(_db: Data<DatabaseConnection>) -> actix_web::Result<HttpResponse, CrudError> {
    // send new empty token and bearer in response 
    let mut token = Cookie::build("token", "").path("/").finish();
    token.make_removal();
    let mut bearer = Cookie::build("bearer", "").http_only(true).finish();
    bearer.make_removal();
    return Ok(HttpResponse::Ok().cookie(token).cookie(bearer).finish());
}
