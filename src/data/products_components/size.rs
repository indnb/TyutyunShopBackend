use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Size {
    pub product_id: i32,
    pub s: bool,
    pub m: bool,
    pub l: bool,
    pub xl: bool,
    pub xxl: bool,
}
