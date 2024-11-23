extern crate rocket;

use crate::query::mail::mail_query::new_order_receive;
use crate::query::orders::orders_query::{
    delete_order, get_order_details, get_orders, place_new_order, update_order_status,
};
use crate::query::orders::shipping_query::{add_shipping, get_shipping_by_id};
use crate::query::products_components::category_query::{
    create_category, delete_category_by_id, get_categories, get_category, update_category_name,
};
use crate::query::products_components::product_image_query::{
    create_product_image, delete_product_image_by_id, get_all_product_images,
    get_one_product_image, update_product_image,
};
use crate::query::products_components::product_query::{
    create_product, delete_product, get_products, product_update,
};
use crate::query::products_components::size_query::{create_size, get_size, update_size};
use crate::query::user::user_query::{
    get_profile, get_user_role, login, registration_by_token, try_registration, update_password,
    update_profile,
};
use crate::utils::env_configuration::CONFIG;
use log::LevelFilter;
use reqwest::Client;
use rocket::figment::Figment;
use rocket::Config;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use sqlx::PgPool;
use std::net::IpAddr;

pub async fn set_up_rocket(db_pool: PgPool) {
    configure_logging();

    let config = get_server_config().expect("Failed to configure Rocket server");
    let cors = configure_cors();
    let client = Client::new();

    build_rocket(db_pool, config, cors, client).await;
}

fn configure_logging() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
}

fn get_server_config() -> Result<Config, rocket::figment::Error> {
    let (address, port) = parse_address_port();

    Figment::from(Config::default())
        .merge(("address", address.to_string()))
        .merge(("port", port))
        .extract()
}

fn parse_address_port() -> (IpAddr, u16) {
    let address = CONFIG
        .get()
        .unwrap()
        .server_address
        .parse()
        .expect("Invalid IP address");

    let port = CONFIG
        .get()
        .unwrap()
        .server_port
        .parse()
        .expect("Invalid port number");

    (address, port)
}

fn configure_cors() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&[
            "http://localhost:3000",
            format!(
                "http://{}:{}",
                CONFIG.get().unwrap().server_address,
                CONFIG.get().unwrap().server_port
            )
            .as_str(),
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

async fn build_rocket(db_pool: PgPool, config: Config, cors: Cors, client: Client) {
    rocket::custom(config)
        .attach(cors)
        .attach(rocket::shield::Shield::default())
        .manage(db_pool)
        .manage(client)
        .mount(
            format!("/{}", CONFIG.get().unwrap().dir_product_images.as_str()),
            rocket::fs::FileServer::from(CONFIG.get().unwrap().dir_product_images.as_str()),
        )
        .mount(
            "/api",
            routes![
                update_password,
                login,
                get_profile,
                update_profile,
                create_category,
                create_product_image,
                get_one_product_image,
                get_all_product_images,
                create_product,
                create_size,
                get_categories,
                get_category,
                get_products,
                get_size,
                place_new_order,
                get_user_role,
                delete_product_image_by_id,
                get_orders,
                get_shipping_by_id,
                add_shipping,
                get_order_details,
                update_order_status,
                delete_order,
                update_category_name,
                delete_category_by_id,
                try_registration,
                registration_by_token,
                new_order_receive,
                product_update,
                update_size,
                update_product_image,
                delete_product,
            ],
        )
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}
