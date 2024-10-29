use rocket::fs::TempFile;
use rocket::serde::{Deserialize, Serialize};
use rocket::data::Data;
use rocket::form::Form;
use rocket::State;
use sqlx::PgPool;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::error::api_error::ApiError;
use std::fs;
use crate::data::product_image::ProductImage;
use crate::utils::constants::images_constants::PRODUCT_IMAGES;

#[post("/product", data = "<image_form>")]
pub async fn create_product(db_pool: &State<PgPool>, image_form: Form<ProductImage<'_>>) -> Result<&'static str, ApiError> {
    let product = image_form.into_inner();

    let image_filename = format!("{}.png", Uuid::new_v4());
    let image_path = format!("{}/{}", PRODUCT_IMAGES, image_filename);
    let image_url = format!("/static/{}/{}", PRODUCT_IMAGES, image_filename);

    fs::create_dir_all(PRODUCT_IMAGES).map_err(|_| ApiError::InternalServerError)?;

    let mut file = File::create(&image_path).await.map_err(|_| ApiError::InternalServerError)?;
    let mut image_file = product.image.open().await.map_err(|_| ApiError::InternalServerError)?;
    tokio::io::copy(&mut image_file, &mut file).await.map_err(|_| ApiError::InternalServerError)?;




    Ok("Product successfully created")
}
