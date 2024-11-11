use crate::error::api_error::ApiError;

pub async fn send_request(request: reqwest::RequestBuilder) -> Result<String, ApiError> {
    let response = request.send().await.unwrap();
    let status = response.status();
    let text = response.text().await.unwrap();
    println!("Status: {}, Response: {}", status, text);
    Ok(text)
}
