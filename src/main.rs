#[macro_use]
extern crate rocket;
mod data;
mod error;
mod query;
mod database;
mod utils;

use crate::database::init_db_pool;
use crate::query::user::user_query::{get_profile, login, registration, update_profile};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::{env, fs};
use std::path::Path;
use query::product::category_query::create_category;
use crate::query::product::product_image_query::{create_product_image, get_product_image};
use crate::query::product::product_query::create_product;
use crate::query::product::size::create_size;
use crate::utils::constants::images_constants::PRODUCT_IMAGES;

#[tokio::main]
async fn main() {
    let db_pool = init_db_pool().await;

    if !Path::new(PRODUCT_IMAGES).exists() {
        fs::create_dir(PRODUCT_IMAGES).expect("Failed to create images directory");
    }

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
        allowed_origins: AllowedOrigins::some_exact(&["http://localhost:3000", "http://127.0.0.1:3000"]),
        allowed_methods: vec!["GET", "POST", "PUT", "DELETE"]
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()
        .expect("Error while building CORS");

    rocket::custom(config)
        .manage(db_pool)
        .mount(format!("/{}",  PRODUCT_IMAGES), rocket::fs::FileServer::from(PRODUCT_IMAGES))
        .mount("/api", routes![
            registration,
            login,
            get_profile,
            update_profile,
            create_category,
            create_product_image,
            get_product_image,
            create_product,
            create_size
        ])
        .attach(cors)
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}
