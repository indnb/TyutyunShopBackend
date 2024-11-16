use crate::data::orders::order::{DataOrder, Order};
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};

#[post("/order", data = "<data_order>")]
pub async fn place_new_order(
    db_pool: &State<PgPool>,
    data_order: Json<DataOrder>,
) -> Result<Json<Option<i32>>, ApiError> {
    let data_order = data_order.into_inner();

    let id: Option<i32> = sqlx::query(
        r#"
            INSERT INTO orders (
                user_id, total_price, status, online_payment, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            RETURNING id
        "#,
    )
    .bind(data_order.order.user_id)
    .bind(data_order.order.total_price)
    .bind(data_order.order.status)
    .bind(data_order.order.online_payment)
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

    Ok(Json(id))
}
#[get("/orders?<status>")]
pub async fn get_orders(
    db_pool: &State<PgPool>,
    status: Option<String>,
) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = match status {
        Some(status) => sqlx::query(
            r#"
           SELECT * FROM orders WHERE status = $1
        "#,
        )
        .bind(status)
        .fetch_all(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?,
        _ => sqlx::query(
            r#"
           SELECT * FROM orders
        "#,
        )
        .fetch_all(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?,
    };

    Ok(Json(
        orders
            .into_iter()
            .map(|row| Order {
                id: row.get("id"),
                user_id: row.get("user_id"),
                total_price: row.get("total_price"),
                status: row.get("status"),
                online_payment: row.get("online_payment"),
            })
            .collect::<Vec<Order>>(),
    ))
}
