extern crate rocket;
use crate::query::products_components::category_query::{
    create_category, get_categories, get_category,
};
use crate::query::products_components::product_image_query::{
    create_product_image, get_all_product_images, get_one_product_image,
};
use crate::query::products_components::product_query::{
    create_product, get_product, get_product_category_id,
};
use crate::query::products_components::size_query::{create_size, get_size};
use crate::query::user::user_query::{get_profile, login, registration, update_profile};
use crate::utils::constants::images_constants::PRODUCT_IMAGES;
use log::LevelFilter;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use sqlx::PgPool;
use std::env;
use std::net::IpAddr;

pub async fn set_up_rocket(db_pool: PgPool) {
    configure_logging();

    let config = get_server_config();
    let cors = configure_cors();

    build_rocket(db_pool, config, cors).await;
}

fn configure_logging() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
}

fn get_server_config() -> rocket::Config {
    let (address, port) = parse_address_port();

    rocket::Config {
        address,
        port,
        ..Default::default()
    }
}

fn parse_address_port() -> (IpAddr, u16) {
    let address = env::var("SERVER_ADDRESS")
        .unwrap_or("127.0.0.1".to_string())
        .parse()
        .expect("Invalid IP address");

    let port = env::var("PORT")
        .unwrap_or("8181".to_string())
        .parse()
        .expect("Invalid port number");

    (address, port)
}

fn configure_cors() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&[
            "http://localhost:3000",
            "http://127.0.0.1:3000",
        ]),
        allowed_methods: vec!["GET", "POST", "PUT", "DELETE"]
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error while building CORS")
}

async fn build_rocket(db_pool: PgPool, config: rocket::Config, cors: Cors) {
    rocket::custom(config)
        .manage(db_pool)
        .mount(
            format!("/{}", PRODUCT_IMAGES),
            rocket::fs::FileServer::from(PRODUCT_IMAGES),
        )
        .mount(
            "/api",
            routes![
                registration,
                login,
                get_profile,
                update_profile,
                create_category,
                create_product_image,
                get_one_product_image,
                get_all_product_images,
                create_product,
                create_size,
                get_product,
                get_categories,
                get_category,
                get_product_category_id,
                get_size
            ],
        )
        .attach(cors)
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}
