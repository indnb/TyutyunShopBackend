use crate::data::products_components::category::Category;
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};

#[post("/category", data = "<category_data>")]
pub async fn create_category(
    db_pool: &State<PgPool>,
    category_data: Json<Category>,
) -> Result<Json<&'static str>, ApiError> {
    let category = category_data.into_inner();

    sqlx::query(
        r#"
        INSERT INTO categories (
            name, created_at, updated_at
        )
        VALUES ($1, NOW(), NOW())
    "#,
    )
    .bind(&category.name)
    .execute(&**db_pool)
    .await?;

    Ok(Json("Category successfully created"))
}
#[get("/categories")]
pub async fn get_categories(db_pool: &State<PgPool>) -> Result<Json<Vec<Category>>, ApiError> {
    let category_rows = sqlx::query(
        r#"
        SELECT * FROM categories;
    "#,
    )
    .fetch_all(&**db_pool)
    .await?;

    let categories = category_rows
        .into_iter()
        .map(|row| Category {
            id: Some(row.get("id")),
            name: Some(row.get("name")),
        })
        .collect::<Vec<Category>>();

    Ok(Json(categories))
}

#[get("/category/<id>")]
pub async fn get_category(db_pool: &State<PgPool>, id: i32) -> Result<Json<Category>, ApiError> {
    let category_rows = sqlx::query(
        r#"
        SELECT * FROM categories
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&**db_pool)
    .await?;

    Ok(Json(Category {
        id: category_rows.get("id"),
        name: category_rows.get("name"),
    }))
}
