#[macro_use]
extern crate rocket;
mod data;
mod database;
mod error;
mod query;
mod server;
mod tests;
mod utils;
mod mail;

use crate::database::init_db_pool;
use crate::server::set_up_rocket;
use std::fs;
use std::path::Path;
use crate::utils::env_configuration::{EnvConfiguration, CONFIG};

#[tokio::main]
async fn main() {
    EnvConfiguration::init_config();
    let db_pool = init_db_pool().await;

    if !Path::new(CONFIG.get().unwrap().dir_product_images.as_str()).exists() {
        fs::create_dir(CONFIG.get().unwrap().dir_product_images.as_str()).expect("Failed to create images directory");
    }

    set_up_rocket(db_pool.unwrap()).await;
}
