use actix_web::{get, web::Data, HttpResponse};
use entities::model::tbl_temp_email;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::util::{error::CrudError, middleware::extractor::Authenticated};

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
#[get("/email")]
pub async fn get_email(db: Data<DatabaseConnection>) -> actix_web::Result<HttpResponse, CrudError> {
    let email = tbl_temp_email::Entity::find().all(db.as_ref()).await?;
    Ok(HttpResponse::Ok().json(email))
}
