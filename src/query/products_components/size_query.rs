use crate::data::products_components::size::Size;
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};

#[post("/size", data = "<size>")]
pub async fn create_size(db_pool: &State<PgPool>, size: Json<Size>) -> Result<&'static str, ApiError> {
    let size = size.into_inner();

    let row = sqlx::query(
        r#"
        INSERT INTO product_sizes (
            product_id, s, m, l, xl, xxl
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#
    ).bind(size.product_id)
        .bind(size.s)
        .bind(size.m)
        .bind(size.l)
        .bind(size.xl)
        .bind(size.xxl)
        .fetch_one(&**db_pool)
        .await
        .expect("Error creating size in the database");

    let size_id: i32 = row.get("id");

    sqlx::query(
        r#"
        UPDATE products
        SET size_id = $1
        WHERE id = $2
        "#
    ).bind(size_id)
        .bind(size.product_id)
        .execute(&**db_pool)
        .await
        .expect("Error updating products_components with size_id in the database");

    Ok("Size successfully created and linked to products_components")
}
