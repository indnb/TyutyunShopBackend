use crate::error::api_error::ApiError;
use lettre::message::{Message, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::Transport;
use std::env;

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
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or("587".to_string())
        .parse()
        .map_err(|_| ApiError::EmailError)?;
    let username = env::var("MAIL_USERNAME").map_err(|_| ApiError::EmailError)?;
    let password = env::var("MAIL_PASSWORD").map_err(|_| ApiError::EmailError)?;

    let html_content = format!(
        r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{
                        background-color: #1a1a1a;
                        color: white;
                        font-family: Namu, sans-serif;
                        margin: 0;
                        padding: 20px;
                    }}
                    .button {{
                        display: inline-block;
                        padding: 10px 20px;
                        font-size: 16px;
                        color: #000000;
                        background-color: #FFA500;
                        border: none;
                        border-radius: 5px;
                        text-decoration: none;
                        cursor: pointer;
                    }}
                    .button:hover {{
                        background-color: #e59400;
                    }}
                    .text {{
                        color: #FFA500;
                    }}
                    .container {{
                        max-width: 600px;
                        margin: 0 auto;
                        padding: 20px;
                        background-color: #1a1a1a;
                        border-radius: 10px;
                    }}
                </style>
            </head>
            <body>
                <div class="container">
                    <h2 class="text">Хелоу це Tyuntyun Shop!</h2>
                    <p class="text">Будь ласка активуй свій аккаунт, натисни кнопку нижче:</p>
                    <a href="{link}" class="button">Активувати аккаунт</a>
                    <p class="text">Дякуууую що ти з нами!</p>
                </div>
            </body>
            </html>
        "#,
        link = active_link
    );

    let email = Message::builder()
        .from(
            "Tyutyun Shop <tyutyun-shop@yacode.dev>"
                .parse()
                .map_err(|_| ApiError::EmailError)?,
        )
        .to(to_email.parse().map_err(|_| ApiError::EmailError)?)
        .subject("Account Activation - Tyutyun Shop")
        .singlepart(SinglePart::html(html_content))
        .map_err(|_| ApiError::EmailError)?;

    let creds = Credentials::new(username.clone(), password.clone());
    let mailer = SmtpTransport::starttls_relay(&smtp_server)
        .map_err(|_| ApiError::EmailError)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(|_| ApiError::EmailError)?;

    Ok(format!(
        "Activation email sent successfully to {}",
        to_email
    ))
}
