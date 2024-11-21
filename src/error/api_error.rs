use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::Request;
use std::io::Cursor;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error occurred")]
    DatabaseError(#[from] sqlx::Error),
    #[error("User not found")]
    NotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Bad request")]
    BadRequest,
    #[allow(dead_code)]
    #[error("HTTP error")]
    HttpError,
    #[allow(dead_code)]
    #[error("Payment failed")]
    PaymentError,
    #[error("Email already exists")]
    EmailError,
    #[error("Phone already exists")]
    PhoneError,
    #[error("Username already exists")]
    UsernameError,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        log::error!("API error occurred: {:?}", self);

        let (status, message) = match self {
            ApiError::DatabaseError(_) => (Status::InternalServerError, "Database error occurred"),
            ApiError::NotFound => (Status::NotFound, "User not found"),
            ApiError::InternalServerError => (Status::InternalServerError, "Internal server error"),
            ApiError::Unauthorized => (Status::Unauthorized, "Unauthorized access"),
            ApiError::BadRequest => (Status::BadRequest, "Bad request"),
            ApiError::HttpError => (Status::InternalServerError, "HTTP error occurred"),
            ApiError::PaymentError => (Status::PaymentRequired, "Payment failed"),
            ApiError::EmailError => (Status::Conflict, "Таку пошту вже зареєстровано"),
            ApiError::PhoneError => (Status::Conflict, "Такий телефон вже зареєстровано"),
            ApiError::UsernameError => (Status::Conflict, "Такий логін вже зареєстровано"),
        };

        let body = serde_json::to_string(&ApiErrorBody {
            error: status.to_string(),
            message: message.to_string(),
        })
            .expect("Failed to serialize error body");

        Response::build()
            .status(status)
            .sized_body(body.len(), Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}

#[derive(Serialize)]
struct ApiErrorBody {
    error: String,
    message: String,
}
