use rocket::State;
use sqlx::PgPool;
use crate::error::api_error::ApiError;
use crate::mail::sender::send_mail_new_order;
use crate::query::orders::orders_query::get_order_details;
#[post("/mail/new_order?<id>")]
pub async fn new_order_receive(db_pool: &State<PgPool>, id: i32) -> Result<String, ApiError> {
    Ok(send_mail_new_order(get_order_details(db_pool, id).await?.into_inner())?)
}
