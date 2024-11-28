use crate::data::orders::order::OrderDetails;
use crate::error::api_error::ApiError;
use crate::utils::constants::routes::MAIN_URL;
use crate::utils::env_configuration::CONFIG;
use lettre::message::{Message, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::Transport;
use std::fmt::Write;

pub fn generate_registration_link(token: String) -> String {
    if CONFIG.get().unwrap().local {
        format!(
            "http://{}:{}/api/registration?token={}",
            CONFIG.get().unwrap().server_address,
            CONFIG.get().unwrap().server_port,
            token
        )
    } else {
        format!("{}/api/registration?token={}", MAIN_URL, token)
    }
}

pub fn send_mail_registration(to_email: String, active_link: String) -> Result<String, ApiError> {
    let smtp_address = CONFIG.get().unwrap().smtp_address.as_str();
    let smtp_port: u16 = CONFIG
        .get()
        .unwrap()
        .smtp_port
        .parse()
        .map_err(|_| ApiError::EmailError)?;
    let username = CONFIG.get().unwrap().mail_username.as_str();
    let password = CONFIG.get().unwrap().mail_password.as_str();

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
                        color: #000000 !important;
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
        .subject("Активація аккаунта - Tyutyun Shop")
        .singlepart(SinglePart::html(html_content))
        .map_err(|_| ApiError::EmailError)?;

    let creds = Credentials::new(username.to_string(), password.to_string());
    let mailer = SmtpTransport::starttls_relay(smtp_address)
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

pub fn send_mail_new_order(order_details: OrderDetails) -> Result<String, ApiError> {
    let smtp_address = CONFIG.get().unwrap().smtp_address.as_str();
    let smtp_port: u16 = CONFIG
        .get()
        .unwrap()
        .smtp_port
        .as_str()
        .parse()
        .map_err(|_| ApiError::EmailError)?;
    let username = CONFIG.get().unwrap().mail_username.as_str();
    let password = CONFIG.get().unwrap().mail_password.as_str();

    let mut items_html = String::new();
    for item in &order_details.items {
        write!(
            items_html,
            r#"<tr>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{} грн</td>
        </tr>"#,
            item.product_name,
            item.quantity,
            item.size.clone().unwrap_or_else(|| "N/A".to_string()),
            item.total_price
        )
        .map_err(|_| ApiError::EmailError)?;
    }
    let address = format!(
        "{}, {}",
        order_details.shipping.city, order_details.shipping.branch
    );
    let html_content = format!(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>
            body {{
                background-color: #1c1c1c;
                color: #FFA500;
                font-family: 'Namu', sans-serif;
                margin: 0;
                padding: 20px;
                word-wrap: break-word;
                overflow-wrap: break-word;
            }}
            .container {{
                max-width: 600px;
                margin: auto;
                padding: 20px;
                background-color: #1c1c1c;
                border-radius: 8px;
                border: 1px solid #FFA500;
            }}
            .header {{
                color: #FFA500;
                font-size: 24px;
                text-align: center;
                margin-bottom: 20px;
                font-weight: bold;
            }}
            .details {{
                margin-top: 20px;
            }}
            .details p {{
                margin: 5px 0;
                color: #e59400;
                word-wrap: break-word;
                overflow-wrap: break-word;
            }}
            table {{
                width: 100%;
                border-collapse: collapse;
                margin-top: 10px;
            }}
            th, td {{
                border: 1px solid #FFA500;
                padding: 12px;
                text-align: center;
                color: #e59400;
                word-wrap: break-word;
                overflow-wrap: break-word;
            }}
            th {{
                background-color: #FFA500;
                color: #000;
            }}
            tr:nth-child(even) {{
                background-color: #1c1c1c;
            }}
            tr:nth-child(odd) {{
                background-color: #292929;
            }}
            .button {{
                display: inline-block;
                margin-top: 20px;
                padding: 10px 20px;
                background-color: #FFA500;
                color: #000;
                text-decoration: none;
                border-radius: 4px;
                text-align: center;
                font-weight: bold;
                cursor: pointer;
            }}
            .button:hover {{
                background-color: #e59400;
                color: #1a1a1a;
            }}
            @media (max-width: 600px) {{
                body {{
                    padding: 10px;
                }}
                .container {{
                    padding: 15px;
                }}
                th, td {{
                    font-size: 12px;
                    padding: 8px;
                }}
                .header {{
                    font-size: 20px;
                }}
                .details p {{
                    font-size: 14px;
                }}
            }}
        </style>
    </head>
    <body>
        <div class="container">
            <h2 class="header">Деталі нового замовлення</h2>
            <div class="details">
                <h3 style="color: #FFA500;">Інформація про доставку</h3>
                <p><strong>Адреса:</strong> {address}</p>
                <p><strong>Ім'я:</strong> {first_name} {last_name}</p>
                <p><strong>Телефон:</strong> {phone}</p>
                <p><strong>Пошта:</strong> {email}</p>
            </div>
            <div class="details">
                <h3 style="color: #FFA500;">Товари</h3>
                <table>
                    <thead>
                        <tr>
                            <th>Назва</th>
                            <th>Кількість</th>
                            <th>Розмір</th>
                            <th>Ціна</th>
                        </tr>
                    </thead>
                    <tbody>
                        {items}
                    </tbody>
                </table>
                <p style="text-align: right; font-size: 18px; margin-top: 20px; color: #FFA500;">
                    <strong>Загальна сума:</strong> {total_price} грн
                </p>
            </div>
        </div>
    </body>
    </html>
    "#,
        first_name = order_details.shipping.first_name,
        last_name = order_details.shipping.last_name,
        address = address,
        phone = order_details.shipping.phone_number,
        email = order_details.shipping.email,
        items = items_html,
        total_price = order_details
            .items
            .iter()
            .map(|item| item.total_price)
            .sum::<f32>(),
    );

    let email = Message::builder()
        .from(
            "Tyutyun Shop <tyutyun-shop@yacode.dev>"
                .parse()
                .map_err(|_| ApiError::EmailError)?,
        )
        .to(order_details
            .shipping
            .email
            .parse()
            .map_err(|_| ApiError::EmailError)?)
        .subject("Деталі нового замовлення - Tyutyun Shop")
        .singlepart(SinglePart::html(html_content))
        .map_err(|_| ApiError::EmailError)?;

    let creds = Credentials::new(username.to_string(), password.to_string());
    let mailer = SmtpTransport::starttls_relay(smtp_address)
        .map_err(|_| ApiError::EmailError)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(|_| ApiError::EmailError)?;

    Ok(format!(
        "Order confirmation email sent successfully to {}",
        order_details.shipping.email
    ))
}
