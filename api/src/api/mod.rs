use actix_web::{get, web::Data, HttpRequest, HttpResponse};
use sea_orm::DatabaseConnection;
pub mod key;
pub mod user;
#[get("/user")]
pub async fn get_user(
    db: Data<DatabaseConnection>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let cookie = req.cookie("token");

    Ok(HttpResponse::from(HttpResponse::NoContent()))
}
