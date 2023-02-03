use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::{info, error};


use super::error::CrudError;
/// Function that sends an email
pub fn send_mail(mail: Email) -> Result<(), CrudError> {
    let email = Message::builder()
        .from("donotreply@system.net".parse().unwrap())
        .to(mail.email_to.parse().unwrap())
        .subject(mail.subject)
        .body(String::from(mail.message))
        .unwrap();
    let username = dotenv::var("SMTP_USERNAME")?;
    let password = dotenv::var("SMTP_PASSWORD")?;
    let creds = Credentials::new(username, password);

    let address = dotenv::var("SMTP_ADDRESS")?;
    let port = dotenv::var("SMTP_PORT")?;

    let mailer = SmtpTransport::builder_dangerous(address)
        .credentials(creds)
        .port(port.parse()?)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => info!("Email sent successfully!"),
        Err(e) => error!("Could not send email: {:?}", e),
    };
    Ok(())
}

pub const CREATE_COMMENT: &'static str = "New Comment on Request";

#[derive(Debug, Clone)]
pub struct Email {
    pub email_to: String,
    pub subject: String,
    pub message: String,
}