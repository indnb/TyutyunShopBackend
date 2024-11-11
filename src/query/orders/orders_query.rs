use crate::data::orders::order::DataOrder;
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};

#[post("/order", data = "<data_order>")]
pub async fn place_new_order(
    db_pool: &State<PgPool>,
    data_order: Json<DataOrder>,
) -> Result<String, ApiError> {
    let data_order = data_order.into_inner();

    let id: Option<i32> = sqlx::query(
        r#"
            INSERT INTO orders (
                user_id, total_price, status, created_at, updated_at
            )
            VALUES ($1, $2, $3, NOW(), NOW())
            RETURNING id
        "#,
    )
    .bind(data_order.order.user_id)
    .bind(data_order.order.total_price)
    .bind(&data_order.order.status)
    .fetch_one(&**db_pool)
    .await
    .map_err(ApiError::DatabaseError)?
    .get("id");

    for item in data_order.order_items.into_iter() {
        sqlx::query(
            r#"
            INSERT INTO order_items (
                order_id, product_id, quantity, price, size
            )
            VALUES ($1, $2, $3, $4, $5)
        "#,
        )
        .bind(id)
        .bind(item.product_id)
        .bind(item.quantity)
        .bind(item.price)
        .bind(item.size)
        .execute(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;
    }

    Ok("New order succeed placed".to_string())
}
