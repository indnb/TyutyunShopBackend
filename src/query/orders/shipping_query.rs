use crate::data::orders::shipping::Shipping;
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;

#[post("/shipping", data = "<shipping_data>")]
pub async fn add_shipping(
    db_pool: &State<PgPool>,
    shipping_data: Json<Shipping>,
) -> Result<Json<String>, ApiError> {
    let shipping_data = shipping_data.into_inner();
    sqlx::query(
        r#"
            INSERT INTO shipping_addresses (
            order_id, address, first_name, last_name, phone_number, email, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
        "#,
    )
    .bind(shipping_data.order_id)
    .bind(shipping_data.address)
    .bind(shipping_data.first_name)
    .bind(shipping_data.last_name)
    .bind(shipping_data.phone_number)
    .bind(shipping_data.email)
    .execute(&**db_pool)
    .await
    .map_err(ApiError::DatabaseError)?;
    Ok(Json("Succeed shipping address added".to_string()))
}

#[get("/shipping/<order_id>")]
pub async fn get_shipping_by_id(
    db_pool: &State<PgPool>,
    order_id: i32,
) -> Result<Json<Shipping>, ApiError> {
    let shipping = sqlx::query_as::<_, Shipping>(
        r#"
            SELECT * FROM shipping_address
            WHERE order_id = $1
        "#,
    )
    .bind(order_id)
    .fetch_one(&**db_pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok(Json(shipping))
}
