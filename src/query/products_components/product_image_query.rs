use rocket::form::Form;
use rocket::State;
use sqlx::{PgPool, Row};
use tokio::fs::File;
use uuid::Uuid;
use crate::error::api_error::ApiError;
use std::fs;
use rocket::serde::json::{json, Json};
use crate::data::products_components::product_image::ProductImage;
use crate::utils::constants::images_constants::PRODUCT_IMAGES;

#[post("/product_image", data = "<image_form>")]
pub async fn create_product_image(db_pool: &State<PgPool>, image_form: Form<ProductImage<'_>>) -> Result<&'static str, ApiError> {
    let product = image_form.into_inner();

    let image_filename = format!("{}.png", Uuid::new_v4());
    let image_path = format!("{}/{}", PRODUCT_IMAGES, image_filename);

    fs::create_dir_all(PRODUCT_IMAGES).map_err(|_| ApiError::InternalServerError)?;

    let mut file = File::create(&image_path).await.map_err(|_| ApiError::InternalServerError)?;
    let mut image_file = product.image.open().await.map_err(|_| ApiError::InternalServerError)?;
    tokio::io::copy(&mut image_file, &mut file).await.map_err(|_| ApiError::InternalServerError)?;

    sqlx::query(
        r#"
            INSERT INTO product_images (
             image_url, created_at, updated_at
            )
            VALUES($1, NOW(), NOW())
        "#
    ).bind(&image_filename)
        .execute(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;


    Ok("Product successfully created")
}

#[get("/product_image")]
pub async fn get_product_image(db_pool: &State<PgPool>) -> Result<Json<serde_json::Value>, ApiError> {
    let row = sqlx::query(
        r#"
            SELECT image_url
            FROM product_images
            WHERE id = $1
        "#
    ).bind(2)
        .fetch_one(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

    let image_url: String = row.get(0);

    Ok(Json(json!({
        "image_url": format!("http://127.0.0.1:8181/{}/{}", PRODUCT_IMAGES, image_url)
    })))
}

