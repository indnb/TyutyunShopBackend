#[macro_use]
extern crate rocket;
mod data;
mod database;
mod error;
mod mail;
mod query;
mod server;
mod tests;
mod utils;

use crate::database::init_db_pool;
use crate::server::set_up_rocket;
use crate::utils::constants::routes::PATH_PRODUCT_IMAGES;
use crate::utils::env_configuration::EnvConfiguration;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    EnvConfiguration::init_config();
    let db_pool = init_db_pool().await;

    if !Path::new(PATH_PRODUCT_IMAGES).exists() {
        fs::create_dir(PATH_PRODUCT_IMAGES).expect("Failed to create images directory");
    }

    set_up_rocket(db_pool.unwrap()).await;
}
