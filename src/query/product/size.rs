use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;
use crate::data::product::size::Size;
use crate::error::api_error::ApiError;

#[post("/size", data = "<size>")]
pub async fn create_size(db_pool: &State<PgPool>, size: Json<Size>) -> Result<&'static str, ApiError> {
    let size = size.into_inner();

    sqlx::query(
        r#"
        INSERT INTO product_sizes (
            product_id, s, m, l, xl, xxl
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    ).bind(size.product_id)
        .bind(size.s)
        .bind(size.m)
        .bind(size.l)
        .bind(size.xl)
        .bind(size.xxl)
        .execute(&**db_pool)
        .await
        .expect("Error creating size into database");

    Ok("Size successfully created")
}