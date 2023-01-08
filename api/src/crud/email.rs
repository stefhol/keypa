use entities::model::tbl_temp_email;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::util::error::CrudError;
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Email {
    email_from: String,
    email_to: String,
    subject: String,
    message: String,
}
pub(crate) async fn create_email(db: &DatabaseConnection, email: Email) -> Result<(), CrudError> {
    tbl_temp_email::ActiveModel {
        email_from: Set(String::from("donotreply@system.net")),
        email_to: Set(email.email_to.to_string()),
        subject: Set(email.subject.to_string()),
        message: Set(email.message.to_string()),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(())
}
