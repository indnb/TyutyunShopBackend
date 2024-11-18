use crate::data::products_components::product_image::{NewProductImage, ProductImage};
use crate::data::user_components::claims::Claims;
use crate::error::api_error::ApiError;
use crate::utils::constants::images_constants::PRODUCT_IMAGES;
use dotenv::dotenv;
use rocket::form::Form;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::{Request, State};
use serde::Deserialize;
use sqlx::{PgPool, Row};
use std::{env, fs};
use tokio::fs::File;
use uuid::Uuid;
use crate::data::user_components::claims;

#[post("/product_image", data = "<image_form>")]
pub async fn create_product_image(
    db_pool: &State<PgPool>,
    image_form: Form<NewProductImage<'_>>,
    claims: Claims
) -> Result<&'static str, ApiError> {
    claims.check_admin()?;
    let product_image = image_form.into_inner();

    let image_filename = format!("{}.png", Uuid::new_v4());
    let image_path = format!("{}/{}", PRODUCT_IMAGES, image_filename);

    fs::create_dir_all(PRODUCT_IMAGES).map_err(|_| ApiError::InternalServerError)?;

    let mut file = File::create(&image_path)
        .await
        .map_err(|_| ApiError::InternalServerError)?;
    let mut image_file = product_image
        .image
        .open()
        .await
        .map_err(|_| ApiError::InternalServerError)?;
    tokio::io::copy(&mut image_file, &mut file)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    sqlx::query(
        r#"
            INSERT INTO product_images (
             image_url, product_id, created_at, updated_at
            )
            VALUES($1, $2, NOW(), NOW())
        "#,
    )
    .bind(&image_filename)
    .bind(product_image.product_id)
    .execute(&**db_pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok("Product successfully created")
}

#[get("/product_image/<id>")]
pub async fn get_one_product_image(
    db_pool: &State<PgPool>,
    id: i32,
) -> Result<Json<ProductImage>, ApiError> {
    let row = sqlx::query(
        r#"
            SELECT *
            FROM product_images
            WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&**db_pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    let id: i32 = row.get("id");
    let image_url: String = format!(
        "http://{}:{}/{}/{}",
        env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string()),
        env::var("SERVER_PORT").unwrap_or("8181".to_string()),
        PRODUCT_IMAGES,
        row.get::<String, &str>("image_url")
    );

    Ok(Json(ProductImage { id, image_url }))
}

#[delete("/product_image/<id>")]
pub async fn delete_product_image_by_id(
    db_pool: &State<PgPool>,
    id: i32,
    claims: Claims,
) -> Result<Json<String>, ApiError> {
    claims.check_admin()?;
    let path = sqlx::query("SELECT image_url FROM product_images WHERE id = $1")
        .bind(id)
        .fetch_one(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;
    let path: String = path.get("image_url");

    let absolute_path = env::current_dir()
        .expect("Failed to get current directory")
        .join(PRODUCT_IMAGES)
        .join(path);

    let absolute_path_str = absolute_path
        .to_str()
        .expect("Failed to convert path to string");

    sqlx::query(
        r#"
            DELETE
            FROM product_images
            WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&**db_pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    if let Err(e) = tokio::fs::remove_file(absolute_path_str).await {
        eprintln!("Failed to delete file: {}", e);
        return Err(ApiError::InternalServerError);
    }

    Ok(Json("Successfully deleted image".to_string()))
}

#[get("/product_image_all?<product_id>")]
pub async fn get_all_product_images(
    db_pool: &State<PgPool>,
    product_id: Option<i32>,
) -> Result<Json<Vec<ProductImage>>, ApiError> {
    dotenv().ok();
    let rows = match product_id {
        None => sqlx::query(r#"SELECT * FROM product_images"#)
            .fetch_all(&**db_pool)
            .await
            .map_err(ApiError::DatabaseError)?,
        Some(id) => sqlx::query(
            r#"
            SELECT *
            FROM product_images
            WHERE product_id = $1
        "#,
        )
        .bind(id)
        .fetch_all(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?,
    };

    let images: Vec<ProductImage> = rows
        .iter()
        .map(|row| ProductImage {
            id: row.get("id"),
            image_url: format!(
                "http://{}:{}/{}/{}",
                env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string()),
                env::var("SERVER_PORT").unwrap_or("8181".to_string()),
                PRODUCT_IMAGES,
                row.get::<String, _>("image_url")
            ),
        })
        .collect();

    Ok(Json(images))
}
