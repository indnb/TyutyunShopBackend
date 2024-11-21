use crate::data::products_components::category::Category;
use crate::error::api_error::ApiError;
use crate::tests::database::request_test_db::send_request;
use crate::tests::database::user_test_db::UserTest;
use rocket::serde::json::json;

#[allow(dead_code)]
pub async fn create_category(user_test: &UserTest<'_>, name: &str) -> Result<(), ApiError> {
    let request = user_test
        .client
        .post(format!("{}/api/category", user_test.base_url))
        .header("Authorization", user_test.auth_header.as_str())
        .json(&json!(Category {
            id: None,
            name: Some(name.to_string()),
        }));
    send_request(request).await?;
    Ok(())
}
