use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub description: Option<String>,
    pub primary_image_id: i32,
    pub price: i32,
    pub stock_quantity: i32,
    pub size_id: i32,
}