#[macro_use]
extern crate rocket;
mod data;
mod error;

use crate::data::claims::Claims;
use crate::data::user::{NewUser, User};
use crate::error::api_error::ApiError;
use bcrypt::{hash, verify, DEFAULT_COST};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket_cors::{AllowedOrigins, CorsOptions};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::str::FromStr;

fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Ошибка при хешировании пароля")
}

#[post("/user/registration", data = "<user_data>")]
async fn registration(
    db_pool: &State<sqlx::PgPool>,
    user_data: Json<NewUser>,
) -> Result<(), ApiError> {
    let new_user = user_data.into_inner();

    sqlx::query!(
        r#"
        INSERT INTO users (
            username, email, password_hash, first_name, last_name, phone_number, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
        "#,
        new_user.username,
        new_user.email,
        hash_password(new_user.password.as_str()),
        new_user.first_name,
        new_user.last_name,
        new_user.phone_number
    )
        .execute(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

    Ok(())
}
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub token: String,
}
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[post("/user/login", data = "<login_data>")]
async fn login(
    db_pool: &State<sqlx::PgPool>,
    login_data: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let login_data = login_data.into_inner();

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, first_name, last_name, phone_number, address
        FROM users WHERE email = $1
        "#,
        login_data.email
    )
    .fetch_one(&**db_pool)
    .await
    .map_err(|_| ApiError::NotFound)?;

    let is_password_valid = verify(&login_data.password, &user.password_hash)
        .map_err(|_| ApiError::InternalServerError)?;

    if !is_password_valid {
        return Err(ApiError::Unauthorized);
    }

    // Генерация JWT токена
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let claims = Claims::new(user.id);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(LoginResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        token,
    }))
}

#[get("/user/profile")]
async fn get_profile(claims: Claims, db_pool: &State<PgPool>) -> Result<Json<UserProfile>, Status> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, password_hash, username, first_name, last_name, email, phone_number, address
        FROM users WHERE id = $1
        "#,
        claims.sub
    )
    .fetch_one(&**db_pool)
    .await
    .map_err(|_| Status::NotFound)?;

    Ok(Json(UserProfile {
        username: user.username,
        email: user.email,
        first_name: user.first_name.unwrap_or("".to_string()),
        last_name: user.last_name.unwrap_or("".to_string()),
        phone_number: user.phone_number.unwrap_or("".to_string()),
        address: "".to_string(),
    }))
}
#[derive(Debug, Serialize, Deserialize)]
struct UserProfile {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub address: String,
}

async fn init_db_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

#[launch]
async fn rocket() -> _ {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    let db_pool = init_db_pool().await;
    let server_address =
        env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8181".to_string());

    let config = rocket::Config {
        address: server_address
            .split(':')
            .next()
            .unwrap_or("127.0.0.1")
            .parse()
            .expect("Invalid IP address"),
        port: server_address
            .split(':')
            .nth(1)
            .unwrap_or("8000")
            .parse()
            .expect("Invalid port number"),
        ..Default::default()
    };

    // Set up CORS options
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&["http://localhost:3000"]),
        allowed_methods: ["GET", "POST"]
            .iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS");

    rocket::custom(config)
        .manage(db_pool)
        .mount("/api", routes![registration, login, get_profile])
        .attach(cors)
}
