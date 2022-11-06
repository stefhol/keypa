use actix_web::{http::StatusCode, ResponseError};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum CrudError {
    #[error("Error in Database")]
    DbError(#[from] sea_orm::error::DbErr),
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Error in Uuid Conversion")]
    // #[error(transparent)]
    UuidError(#[from] uuid::Error),
    #[error("Error in Dotenv Retrieval")]
    DotenvError(#[from] dotenv::Error),
    #[error("Error in JsonWebToken Generation")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    #[error("Token Invalid, try Login again")]
    InvalidToken,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
// #[derive(Debug, Error, ToSchema)]
// pub enum CrudError {
//     /// when not authrorized
//     #[error("Unauthorized")]
//     Unauthorized,
//     /// when not found
//     #[error("Not found")]
//     NotFound,
//     #[error("Invalid Uuid format")]
//     UnprocessableUUid(#[from] uuid::Error),
//     #[error("Invalid Uuid format")]
//     #[error("Internal Error")]
//     DotenvError(#[from] dotenv::Error),
// }
impl ResponseError for CrudError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            CrudError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CrudError::NotFound => StatusCode::NOT_FOUND,
            CrudError::UuidError(_) => StatusCode::BAD_REQUEST,
            CrudError::DotenvError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CrudError::JsonWebTokenError(_) => StatusCode::NOT_ACCEPTABLE,
            CrudError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            CrudError::Unauthorized => StatusCode::UNAUTHORIZED,
            CrudError::InvalidToken => StatusCode::UNAUTHORIZED,
            // Self::Unauthorized => StatusCode::UNAUTHORIZED,
            // Self::NotFound => StatusCode::NOT_FOUND,
            // Self::UnprocessableUUid(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}
