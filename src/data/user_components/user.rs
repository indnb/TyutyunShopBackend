use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TempUser {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub address: Option<String>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JwtUser {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub address: Option<String>,
    pub exp: usize,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub address: String,
}
