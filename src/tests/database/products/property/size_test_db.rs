use crate::data::products_components::size::Size;
use crate::error::api_error::ApiError;
use crate::tests::database::request_test_db::send_request;
use crate::tests::database::user_test_db::UserTest;
use rocket::serde::json::json;

pub async fn create_sizes(user_test: &UserTest<'_>, size: &Size) -> Result<(), ApiError> {
    let request = user_test
        .client
        .post(format!("{}/api/size", user_test.base_url))
        .header("Authorization", user_test.auth_header.as_str())
        .json(&json!(size));
    send_request(request).await?;
    Ok(())
}
