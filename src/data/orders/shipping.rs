use rocket::serde::{Deserialize, Serialize};
use sqlx_macros::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Shipping {
    pub order_id: i32,
    pub city: String,
    pub branch: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
}
