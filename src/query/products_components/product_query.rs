use crate::data::products_components::product::Product;
use crate::data::user_components::claims::Claims;
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{query, PgPool, Row};

#[post("/product", data = "<product>")]
pub async fn create_product(
    db_pool: &State<PgPool>,
    product: Json<Product>,
    claims: Claims,
) -> Result<Json<i32>, ApiError> {
    Claims::check_admin(db_pool, claims).await?;
    let product = product.into_inner();

    let product_id = sqlx::query(
        r#"
                INSERT INTO products(
                    name, description, primary_image_id, price, category_id, size_id, created_at, updated_at
                )
                VALUES($1, $2, $3, $4, $5, $6, NOW(), NOW())
                RETURNING id
        "#
    ).bind(product.name)
        .bind(product.description)
        .bind(product.primary_image_id)
        .bind(product.price)
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
        "#,
    )
    .bind(product_id)
    .bind(product.primary_image_id)
    .execute(&**db_pool)
    .await
    .expect("Error updating product_images with product_id in the database");

    Ok(Json(product_id))
}

#[get("/product?<category_id>&<selected_id>&<product_id>")]
pub async fn get_products(
    db_pool: &State<PgPool>,
    category_id: Option<i32>,
    selected_id: Option<i32>,
    product_id: Option<i32>,
) -> Result<Json<Vec<Product>>, ApiError> {
    let query = match (category_id, selected_id, product_id) {
        (None, None, Some(id)) => sqlx::query(
            r#"
            SELECT * FROM products
            WHERE id = $1
            "#,
        )
        .bind(id),
        (Some(id), None, None) => sqlx::query(
            r#"
            SELECT * FROM products
            WHERE category_id = $1
            "#,
        )
        .bind(id),
        (Some(id), Some(selected_id), None) => sqlx::query(
            r#"
            SELECT * FROM products
            WHERE category_id = $1 AND id != $2
            "#,
        )
        .bind(id)
        .bind(selected_id),
        _ => sqlx::query(
            r#"
            SELECT * FROM products
            "#,
        ),
    };

    let products = query.fetch_all(&**db_pool).await?;

    Ok(Json(
        products
            .into_iter()
            .map(|product| Product {
                id: product.get("id"),
                name: product.get("name"),
                description: product.get("description"),
                primary_image_id: product.get("primary_image_id"),
                price: product.get("price"),
                size_id: product.get("size_id"),
                category_id: product.get("category_id"),
            })
            .collect::<Vec<Product>>(),
    ))
}
#[put("/product/update", data = "<product>")]
pub async fn product_update(
    db_pool: &State<PgPool>,
    product: Json<Product>,
    claims: Claims,
) -> Result<String, ApiError> {
    Claims::check_admin(db_pool, claims).await?;
    let product = product.into_inner();

    let _ = query(
        r#"
        UPDATE products
        SET name = $1, description = $2, primary_image_id = $3, price = $4, category_id = $5, updated_at = NOW()
        WHERE id = $6
    "#,
    )
    .bind(product.name)
    .bind(product.description)
    .bind(product.primary_image_id)
    .bind(product.price)
    .bind(product.category_id)
    .bind(product.id)
    .execute(&**db_pool)
    .await?;

    Ok("Product succeed update!".to_string())
}
#[delete("/product/<id>")]
pub async fn delete_product(db_pool: &State<PgPool>, id: i32, claims: Claims) -> Result<String, ApiError> {
    Claims::check_admin(db_pool, claims).await?;
    let _ = query(r#"
        DELETE FROM products
        WHERE id = $1
    "#).bind(id).execute(&**db_pool).await?;
    Ok("Product was successfully deleted!".to_string())
}