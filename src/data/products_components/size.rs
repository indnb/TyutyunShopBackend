use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Size {
    pub product_id: i32,
    pub single_size: Option<i32>,
    pub s: Option<i32>,
    pub m: Option<i32>,
    pub l: Option<i32>,
    pub xl: Option<i32>,
    pub xxl: Option<i32>,
}
