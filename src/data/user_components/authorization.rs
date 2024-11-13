use rocket::serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;
use crate::data::user_components::claims::Claims;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub token: String,
    pub role: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct RoleResponse {
    pub role: String,
}
pub struct AuthenticatedUser {
    pub user_id: i32,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token = keys[0].replace("Bearer ", "");
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");

        match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
            Ok(token_data) => Outcome::Success(AuthenticatedUser { user_id: token_data.claims.sub }),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}