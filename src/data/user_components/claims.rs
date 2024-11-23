use crate::error::api_error::ApiError;
use crate::query::user::user_query::get_user_role;
use crate::utils::env_configuration::CONFIG;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request, State};
use serde::{Deserialize, Serialize};
use sqlx::Error::RowNotFound;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: i32,
    pub role: Option<String>,
}

impl Claims {
    pub async fn check_admin(db_pool: &State<PgPool>, claims: Claims) -> Result<(), ApiError> {
        if get_user_role(db_pool, claims)
            .await
            .map_err(|_| ApiError::DatabaseError(RowNotFound))?
            .role
            != CONFIG.get().unwrap().admin_role.clone()
        {
            return Err(ApiError::Unauthorized);
        }
        Ok(())
    }
}

impl Claims {
    pub fn new(sub: i32, role: Option<String>) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        Claims {
            exp: expiration,
            sub,
            role,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "));

        match token {
            Some(token) => {
                let secret = CONFIG.get().unwrap().jwt_secret.as_str();

                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::new(Algorithm::HS512),
                ) {
                    Ok(token_data) => request::Outcome::Success(token_data.claims),
                    Err(e) => {
                        warn!("Error decoding token: {:?}", e);
                        request::Outcome::Error((Status::Unauthorized, ()))
                    }
                }
            }
            None => {
                warn!("Token not found in header \"Authorization\"");
                request::Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}
