use actix_web::web::Data;
use paperclip::actix::{api_v2_operation, get, Apiv2Schema, NoContent};
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::util::{error::MyError, middleware::extractor::Authenticated};
#[derive(Serialize, Apiv2Schema)]
struct Login {
    message: String,
}
#[api_v2_operation(
    summary = "My awesome handler",
    description = "It creates a pretty JSON object",
    /// A few other parameters are also supported
    operation_id = "login",
    tags("Api reference"),
)]
#[get("/login")]
pub async fn login(
    _db: Data<DatabaseConnection>,
    auth: Authenticated,
) -> actix_web::Result<paperclip::actix::NoContent, MyError> {
    println!("{:?}", auth);
    Ok(NoContent)
}
