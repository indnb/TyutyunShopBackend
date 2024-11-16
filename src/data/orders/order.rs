use crate::data::orders::order_item::OrderItem;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub total_price: f32,
    pub status: String,
    pub online_payment: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataOrder {
    pub order: Order,
    pub order_items: Vec<OrderItem>,
}
