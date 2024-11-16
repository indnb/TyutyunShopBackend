use crate::data::products_components::product::Product;
use crate::error::api_error::ApiError;
use crate::tests::database::request_test_db::send_request;
use crate::tests::database::user_test_db::UserTest;
use rocket::serde::json::json;

pub async fn create_product<'a>(
    user_test: &UserTest<'a>,
    product: &Product,
) -> Result<(), ApiError> {
    let request = user_test
        .client
        .post(format!("{}/api/product", user_test.base_url))
        .header("Authorization", user_test.auth_header.as_str())
        .json(&json!(product));
    send_request(request).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn get_product_by_id<'a>(user: &UserTest<'a>) -> Result<(), ApiError> {
    let request = user
        .client
        .get(format!("{}/api/product?product_id=1", user.base_url));
    send_request(request).await?;
    Ok(())
}
