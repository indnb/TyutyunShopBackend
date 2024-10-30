use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};
use crate::data::products_components::product::Product;
use crate::error::api_error::ApiError;

#[post("/product", data = "<product>")]
pub async fn create_product(db_pool: &State<PgPool>, product: Json<Product>) -> Result<Json<&'static str>, ApiError> {
    let product = product.into_inner();

    let product_id = sqlx::query(
        r#"
                INSERT INTO products(
                    name, description, primary_image_id, price, stock_quantity, category_id, size_id, created_at, updated_at
                )
                VALUES($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
                RETURNING id
        "#
    ).bind(product.name)
        .bind(product.description)
        .bind(product.primary_image_id)
        .bind(product.price)
        .bind(product.stock_quantity)
        .bind(product.category_id)
        .bind(product.size_id)
        .fetch_one(&**db_pool)
        .await
        .expect("Error created products_components into database");

    let product_id: i32 = product_id.get("id");

    sqlx::query(
        r#"
                UPDATE product_images
                SET product_id=$1
                WHERE id=$2
        "#
    ).bind(product_id)
        .bind(product.primary_image_id)
        .execute(&**db_pool)
        .await.expect("Error updating product_images with product_id in the database");


    Ok(Json("Product successfully created"))
}