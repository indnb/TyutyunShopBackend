use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub order_id: Option<i32>,
    pub product_id: i32,
    pub quantity: i32,
    pub price: f32,
    pub size: Option<String>,
    pub total_price: f32,
}
