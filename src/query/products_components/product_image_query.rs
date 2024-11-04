use crate::data::products_components::product_image::ProductImage;
use crate::error::api_error::ApiError;
use crate::utils::constants::images_constants::PRODUCT_IMAGES;
use rocket::form::Form;
use rocket::serde::json::{json, Json};
use rocket::State;
use sqlx::{PgPool, Row};
use std::fs;
use tokio::fs::File;
use uuid::Uuid;

#[post("/product_image", data = "<image_form>")]
pub async fn create_product_image(db_pool: &State<PgPool>, image_form: Form<ProductImage<'_>>) -> Result<&'static str, ApiError> {
    let product_image = image_form.into_inner();

    let image_filename = format!("{}.png", Uuid::new_v4());
    let image_path = format!("{}/{}", PRODUCT_IMAGES, image_filename);

    fs::create_dir_all(PRODUCT_IMAGES).map_err(|_| ApiError::InternalServerError)?;

    let mut file = File::create(&image_path).await.map_err(|_| ApiError::InternalServerError)?;
    let mut image_file = product_image.image.open().await.map_err(|_| ApiError::InternalServerError)?;
    tokio::io::copy(&mut image_file, &mut file).await.map_err(|_| ApiError::InternalServerError)?;

    sqlx::query(
        r#"
            INSERT INTO product_images (
             image_url, product_id, created_at, updated_at
            )
            VALUES($1, $2, NOW(), NOW())
        "#
    ).bind(&image_filename).
        bind(&product_image.product_id)
        .execute(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;


    Ok("Product successfully created")
}

#[get("/product_image/<id>")]
pub async fn get_product_image(db_pool: &State<PgPool>, id: i32) -> Result<Json<serde_json::Value>, ApiError> {
    let row = sqlx::query(
        r#"
            SELECT image_url
            FROM product_images
            WHERE id = $1
        "#
    ).bind(id)
        .fetch_one(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

    let image_url: String = row.get("image_url");

    Ok(Json(json!({
        "image_url": format!("http://127.0.0.1:8181/{}/{}", PRODUCT_IMAGES, image_url)
    })))
}
