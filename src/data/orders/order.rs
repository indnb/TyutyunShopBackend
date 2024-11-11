use crate::data::orders::order_item::OrderItem;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub user_id: i32,
    pub total_price: f32,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataOrder {
    pub order: Order,
    pub order_items: Vec<OrderItem>,
}
