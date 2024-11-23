use crate::data::orders::order::{DataOrder, Order, OrderDetails, OrderItemDetails};
use crate::data::orders::shipping::Shipping;
use crate::data::user_components::claims::Claims;
use crate::error::api_error::ApiError;
use crate::query::orders::shipping_query::get_shipping_by_id;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::Value;
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
#[get("/orders?<status>&<user_id>")]
pub async fn get_orders(
    db_pool: &State<PgPool>,
    status: Option<String>,
    user_id: Option<i32>,
) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = match user_id {
        Some(id) => match status {
            Some(status) => sqlx::query(
                r#"
                       SELECT * FROM orders WHERE status = $1 AND user_id = $2
                    "#,
            )
            .bind(status)
            .bind(id)
            .fetch_all(&**db_pool)
            .await
            .map_err(ApiError::DatabaseError)?,
            _ => sqlx::query(
                r#"
                       SELECT * FROM orders WHERE user_id = $1
                    "#,
            )
            .bind(id)
            .fetch_all(&**db_pool)
            .await
            .map_err(ApiError::DatabaseError)?,
        },
        _ => match status {
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
        },
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
                date: row.get("created_at"),
            })
            .collect::<Vec<Order>>(),
    ))
}
#[get("/orders/<order_id>/details")]
pub async fn get_order_details(
    db_pool: &State<PgPool>,
    order_id: i32,
) -> Result<Json<OrderDetails>, ApiError> {
    let shipping_details: Json<Shipping> = get_shipping_by_id(db_pool, order_id).await?;

    let order_items: Vec<OrderItemDetails> = sqlx::query_as::<_, OrderItemDetails>(
        r#"
        SELECT
            oi.id,
            p.name AS product_name,
            oi.quantity,
            oi.size,
            oi.total_price
        FROM order_items oi
        JOIN products p ON oi.product_id = p.id
        WHERE oi.order_id = $1
        "#,
    )
    .bind(order_id)
    .fetch_all(&**db_pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok(Json(OrderDetails {
        shipping: shipping_details.into_inner(),
        items: order_items,
    }))
}

#[put("/order/<id>", data = "<status>")]
pub async fn update_order_status(
    db_pool: &State<PgPool>,
    status: Json<Value>,
    id: i32,
    claims: Claims,
) -> Result<String, ApiError> {
    Claims::check_admin(db_pool, claims).await?;

    let status = status
        .get("status")
        .and_then(Value::as_str)
        .ok_or(ApiError::BadRequest)?;

    sqlx::query(
        r#"
            UPDATE orders
            SET status = $2, updated_at = NOW()
            WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(status)
    .execute(&**db_pool)
    .await?;

    Ok("Succeed update status".to_string())
}
#[delete("/order/<id>")]
pub async fn delete_order(
    db_pool: &State<PgPool>,
    id: i32,
    claims: Claims,
) -> Result<String, ApiError> {
    Claims::check_admin(db_pool, claims).await?;
    sqlx::query(
        r#"
            DELETE FROM orders
            WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&**db_pool)
    .await?;

    Ok("Succeed delete order".to_string())
}
