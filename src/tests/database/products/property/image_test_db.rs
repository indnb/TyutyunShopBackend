use crate::error::api_error::ApiError;
use crate::tests::database::request_test_db::send_request;
use crate::tests::database::user_test_db::UserTest;
use reqwest::multipart;

pub async fn upload_image(
    user_test: &UserTest<'_>,
    file_id: &str,
    product_id: Option<&i32>,
) -> Result<(), ApiError> {
    let image_data = download_image_from_drive(file_id).await?;

    let form = match product_id {
        None => multipart::Form::new().part(
            "image",
            multipart::Part::bytes(image_data)
                .mime_str("image/jpeg")
                .unwrap(),
        ),
        Some(id) => multipart::Form::new()
            .text("product_id", id.to_string())
            .part(
                "image",
                multipart::Part::bytes(image_data)
                    .mime_str("image/jpeg")
                    .unwrap(),
            ),
    };

    let request = user_test
        .client
        .post(format!(
            "{}/api/product_image?position=null",
            user_test.base_url
        ))
        .header("Authorization", user_test.auth_header.as_str())
        .multipart(form);

    send_request(request).await?;
    Ok(())
}
async fn download_image_from_drive(file_id: &str) -> Result<Vec<u8>, ApiError> {
    let url = format!("https://drive.google.com/uc?export=download&id={}", file_id);
    let response = reqwest::get(&url).await.map_err(|_| ApiError::BadRequest)?;
    let bytes = response.bytes().await.map_err(|_| ApiError::BadRequest)?;
    Ok(bytes.to_vec())
}
