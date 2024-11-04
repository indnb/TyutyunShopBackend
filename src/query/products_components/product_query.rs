use crate::data::products_components::product::Product;
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};
use crate::data::products_components::category::Category;

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
        .await?;

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

#[get("/product/<id>")]
pub async fn get_product(
    db_pool: &State<PgPool>,
    id: i32,
) -> Result<Json<Product>, ApiError> {
    let row = sqlx::query(
        r#"SELECT id, name, description, primary_image_id, price, stock_quantity, category_id, size_id, created_at, updated_at
            FROM products WHERE id = $1"#
    ).bind(id)
        .fetch_one(&**db_pool)
        .await
        .map_err(|_| ApiError::NotFound)?;

    let product = Product {
        name: row.get("name"),
        description: row.get("description"),
        primary_image_id: row.get("primary_image_id"),
        price: row.get("price"),
        stock_quantity: row.get("stock_quantity"),
        category_id: row.get("category_id"),
        size_id: row.get("size_id"),
    };

    Ok(Json(product))
}

#[get("/product?<category_id>")]
pub async fn get_product_category_id(db_pool: &State<PgPool>, category_id: Option<i32>) -> Result<Json<Vec<Product>>, ApiError> {
    let query = if let Some(id) = category_id {
        sqlx::query(
            r#"
            SELECT * FROM products
            WHERE category_id = $1
            "#
        ).bind(id)
    } else {
        return Ok(Json(Vec::new()));
    };

    let products = query
        .fetch_all(&**db_pool)
        .await?;

    Ok(Json(products.into_iter().map(|product| Product {
        name: product.get("name"),
        description: product.get("description"),
        primary_image_id: product.get("primary_image_id"),
        price: product.get("price"),
        stock_quantity: product.get("stock_quantity"),
        size_id: product.get("size_id"),
        category_id: product.get("category_id"),
    }).collect::<Vec<Product>>()))
}