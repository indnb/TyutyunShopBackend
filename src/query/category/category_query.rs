use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;
use crate::error::api_error::ApiError;
use crate::data::category::Category;

#[post("/category", data = "<category_data>")]
pub async fn create_category(
    db_pool: &State<PgPool>,
    category_data: Json<Category>
) -> Result<Json<&'static str>, ApiError> {
    let category = category_data.into_inner();

    sqlx::query(r#"
        INSERT INTO categories (
            name, created_at
        )
        VALUES ($1, NOW())
    "#).bind(&category.name)
        .execute(&**db_pool)
        .await?;

    Ok(Json("Category successfully created"))
}
