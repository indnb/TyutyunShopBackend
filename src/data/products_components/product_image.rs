use rocket::fs::TempFile;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, FromForm)]
pub struct NewProductImage<'r> {
    pub image: TempFile<'r>,
    pub product_id: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductImage {
    pub id: i32,
    pub image_url: String,
}
