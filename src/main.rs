#[macro_use]
extern crate rocket;
mod data;
mod error;
mod query;
mod database;
use crate::database::init_db_pool;
use crate::query::user::user_query::{get_profile, login, registration, update_profile};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::env;
use std::str::FromStr;
#[tokio::main]
async fn main() {
    let db_pool = init_db_pool().await;
    rocket(db_pool.unwrap()).await;
}

async fn rocket(db_pool: sqlx::PgPool) {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

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
        .mount("/api", routes![
            registration,
            login,
            get_profile,
            update_profile])

        .attach(cors)
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}