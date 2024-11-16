use crate::data::orders::order_item::OrderItem;
use crate::data::orders::shipping::Shipping;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub total_price: f32,
    pub status: String,
    pub online_payment: bool,
    pub date: Option<NaiveDateTime>,
}
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct OrderItemDetails {
    pub id: i32,
    pub product_name: String,
    pub quantity: i32,
    pub size: Option<String>,
    pub total_price: f32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DataOrder {
    pub order: Order,
    pub order_items: Vec<OrderItem>,
}
#[derive(serde::Serialize)]
pub struct OrderDetails {
    pub shipping: Shipping,
    pub items: Vec<OrderItemDetails>,
}
