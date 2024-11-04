#[macro_use]
extern crate rocket;
mod data;
mod error;
mod query;
mod database;
mod utils;
mod server;

use crate::database::init_db_pool;
use crate::server::set_up_rocket;
use crate::utils::constants::images_constants::PRODUCT_IMAGES;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    let db_pool = init_db_pool().await;

    if !Path::new(PRODUCT_IMAGES).exists() {
        fs::create_dir(PRODUCT_IMAGES).expect("Failed to create images directory");
    }

    set_up_rocket(db_pool.unwrap()).await;
}

