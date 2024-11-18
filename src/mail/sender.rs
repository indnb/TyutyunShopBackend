use lettre::message::{Message, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::Transport;
use std::env;
use crate::error::api_error::ApiError;

pub fn generate_registration_link(token: String) -> String {
    format!(
        "http://{}:{}/api/registration?token={}",
        env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string()),
        env::var("SERVER_PORT").unwrap_or("8181".to_string()),
        token
    )
}

pub fn send_mail(to_email: String, active_link: String) -> Result<String, ApiError> {
    let smtp_server = env::var("SMTP_SERVER").map_err(|_| ApiError::EmailError)?;
    let smtp_port: u16 = env::var("SMTP_PORT").unwrap_or("587".to_string()).parse().map_err(|_| ApiError::EmailError)?;
    let username = env::var("MAIL_USERNAME").map_err(|_| ApiError::EmailError)?;
    let password = env::var("MAIL_PASSWORD").map_err(|_| ApiError::EmailError)?;

    let email = Message::builder()
        .from("Tyutyun Shop <tyutyun-shop@yacode.dev>".parse().map_err(|_| ApiError::EmailError)?)
        .to(to_email.parse().map_err(|_| ApiError::EmailError)?)
        .subject("Account Activation - Tyutyun Shop")
        .singlepart(SinglePart::plain(format!(
            "Hello,\n\nWelcome to Tyutyun Shop!\nPlease activate your account by clicking the link below:\n\n{}\n\nThank you!",
            active_link
        ))).map_err(|_| ApiError::EmailError)?;

    let creds = Credentials::new(username.clone(), password.clone());
    let mailer = SmtpTransport::starttls_relay(&smtp_server).map_err(|_| ApiError::EmailError)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(|_| ApiError::EmailError)?;

    Ok(format!("Activation email sent successfully to {}", to_email))
}
