use crate::data::products_components::product_image::{NewProductImage, ProductImage};
use crate::data::user_components::claims::Claims;
use crate::error::api_error::ApiError;
use crate::utils::env_configuration::CONFIG;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};
use std::{env, fs};
use sqlx::postgres::PgRow;
use tokio::fs::File;
use uuid::Uuid;

#[post("/product_image?<position>", data = "<image_form>")]
pub async fn create_product_image(
    db_pool: &State<PgPool>,
    image_form: Form<NewProductImage<'_>>,
    claims: Claims,
    position: Option<i32>,
) -> Result<&'static str, ApiError> {
    Claims::check_admin(db_pool, claims).await?;
    let product_image = image_form.into_inner();

    let image_filename = format!("{}.png", Uuid::new_v4());
    let image_path = format!(
        "{}/{}",
        CONFIG.get().unwrap().dir_product_images,
        image_filename
    );

    fs::create_dir_all(CONFIG.get().unwrap().dir_product_images.as_str())
        .map_err(|_| ApiError::InternalServerError)?;

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

    let mut tx = db_pool.begin().await.map_err(ApiError::DatabaseError)?;

    let id = sqlx::query(
        r#"
            INSERT INTO product_images (
             image_url, product_id, position, created_at, updated_at
            )
            VALUES($1, $2, $3, NOW(), NOW())
            RETURNING id
        "#,
    )
    .bind(&image_filename)
    .bind(product_image.product_id)
    .bind(position)
    .fetch_one(&mut *tx)
    .await
    .map_err(ApiError::DatabaseError)?;

    if position == Some(1) {
        sqlx::query(
            r#"
            UPDATE products
            SET primary_image_id = $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(id.get::<i32, &str>("id"))
        .bind(product_image.product_id)
        .execute(&mut *tx)
        .await
        .map_err(ApiError::DatabaseError)?;
    }

    tx.commit().await.map_err(ApiError::DatabaseError)?;

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
    let image_url: String = get_image_path(&row);

    Ok(Json(ProductImage {
        id: row.get("id"),
        image_url,
        product_id: row.get("product_id"),
        position: row.get("position"),
    }))
}

fn get_image_path(row: &PgRow) -> String {
    if CONFIG.get().unwrap().local {
        format!(
            "http://{}:{}/{}/{}",
            CONFIG.get().unwrap().server_address,
            CONFIG.get().unwrap().server_port,
            CONFIG.get().unwrap().dir_product_images,
            row.get::<String, &str>("image_url")
        )
    } else {
        format!(
            "/{}/{}",
            CONFIG.get().unwrap().dir_product_images,
            row.get::<String, &str>("image_url")
        )
    }
}

#[delete("/product_image/<id>")]
pub async fn delete_product_image_by_id(
    db_pool: &State<PgPool>,
    id: i32,
    claims: Claims,
) -> Result<Json<String>, ApiError> {
    Claims::check_admin(db_pool, claims).await?;
    let path = sqlx::query("SELECT image_url FROM product_images WHERE id = $1")
        .bind(id)
        .fetch_one(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;
    let path: String = path.get("image_url");

    let absolute_path = env::current_dir()
        .expect("Failed to get current directory")
        .join(CONFIG.get().unwrap().dir_product_images.as_str())
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
            image_url: get_image_path(&row),
            product_id,
            position: row.get("position"),
        })
        .collect();

    Ok(Json(images))
}
#[put("/product_image/update", data = "<product_image>")]
pub async fn update_product_image(
    db_pool: &State<PgPool>,
    product_image: Json<ProductImage>,
    claims: Claims,
) -> Result<String, ApiError> {
    Claims::check_admin(db_pool, claims).await?;
    let product_image = product_image.into_inner();

    let mut tx = db_pool.begin().await?;

    sqlx::query(
        r#"
        UPDATE product_images
        SET product_id = $1, position = $3, updated_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(product_image.product_id)
    .bind(product_image.id)
    .bind(product_image.position)
    .execute(&mut *tx)
    .await?;

    if product_image.position == Some(1) {
        sqlx::query(
            r#"
            UPDATE products
            SET primary_image_id = $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(product_image.id)
        .bind(product_image.product_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            UPDATE product_images
            SET position = NULL
            WHERE product_id = $1 AND id != $2 AND position = 1
            "#,
        )
        .bind(product_image.product_id)
        .bind(product_image.id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok("Product image updated successfully".to_string())
}
