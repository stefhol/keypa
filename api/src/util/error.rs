use std::fmt::Display;

use actix_web::ResponseError;
use paperclip::actix::api_v2_errors;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum CrudError {
    #[error("Error in Database")]
    DbError(#[from] sea_orm::error::DbErr),
    #[error("Not found")]
    NotFound,
    #[error("Error in Uuid Conversion")]
    UuidError(#[from] uuid::Error),
    #[error("Error in Dotenv Retrieval")]
    DotenvError(#[from] dotenv::Error),
    #[error("Error in JsonWebToken Generation")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
#[api_v2_errors(
    code = 401,
    description = "Unauthorized: Can't access",
    code = 404,
    description = "Can't find ressource"
)]
#[derive(Debug, Error)]
pub enum MyError {
    Unauthorized = 401,
    NotFound = 400,
}
impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unauthorized")
    }
}
impl ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}
