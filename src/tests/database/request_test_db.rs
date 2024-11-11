use crate::error::api_error::ApiError;

pub async fn send_request(request: reqwest::RequestBuilder) -> Result<String, ApiError> {
    let response = request.send().await.map_err(|_| ApiError::BadRequest)?;
    let status = response.status();
    let text = response.text().await.map_err(|_| ApiError::BadRequest)?;
    println!("Status: {}, Response: {}", status, text);
    Ok(text)
}
