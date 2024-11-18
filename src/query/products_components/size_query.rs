use crate::data::products_components::size::Size;
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};
use crate::data::user_components::claims::Claims;

#[post("/size", data = "<size>")]
pub async fn create_size(
    db_pool: &State<PgPool>,
    size: Json<Size>,
    claims: Claims
) -> Result<&'static str, ApiError> {
    claims.check_admin()?;
    let size = size.into_inner();

    let row = sqlx::query(
        r#"
        INSERT INTO product_sizes (
            product_id, s, m, l, xl, xxl, single_size
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(size.product_id)
    .bind(size.s)
    .bind(size.m)
    .bind(size.l)
    .bind(size.xl)
    .bind(size.xxl)
    .bind(size.single_size)
    .fetch_one(&**db_pool)
    .await
    .expect("Error creating size in the database");

    let size_id: i32 = row.get("id");

    sqlx::query(
        r#"
        UPDATE products
        SET size_id = $1
        WHERE id = $2
        "#,
    )
    .bind(size_id)
    .bind(size.product_id)
    .execute(&**db_pool)
    .await
    .expect("Error updating products_components with size_id in the database");

    Ok("Size successfully created and linked to products_components")
}
#[get("/size/<product_id>")]
pub async fn get_size(db_pool: &State<PgPool>, product_id: i32) -> Result<Json<Size>, ApiError> {
    let row = sqlx::query(
        r#"
            SELECT * FROM product_sizes
            WHERE product_id = $1
        "#,
    )
    .bind(product_id)
    .fetch_one(&**db_pool)
    .await
    .expect("Error creating size in the database");

    Ok(Json(Size {
        product_id: row.get("product_id"),
        single_size: row.get("single_size"),
        s: row.get("s"),
        m: row.get("m"),
        l: row.get("l"),
        xl: row.get("xl"),
        xxl: row.get("xxl"),
    }))
}
