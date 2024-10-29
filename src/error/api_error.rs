use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("User not found")]
    NotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("User not unauthorized")]
    Unauthorized,
}

impl<'r> rocket::response::Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        log::error!("API error: {:?}", self);
        match self {
            ApiError::NotFound => Err(rocket::http::Status::NotFound),
            _ => Err(rocket::http::Status::InternalServerError),
        }
    }
}
