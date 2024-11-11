use crate::error::api_error::ApiError;
use crate::tests::database::request_test_db::send_request;
use reqwest::Client;
use rocket::serde::json::json;

pub struct UserTest<'a> {
    pub client: &'a Client,
    pub base_url: &'a str,
    pub auth_header: String,
}
impl UserTest<'_> {
    #[allow(dead_code)]
    pub async fn new<'a>(client: &'a Client, base_url: &'a str) -> Result<UserTest<'a>, ApiError> {
        Self::user_registration(client, base_url).await?;
        let auth_header = format!("Bearer {}", Self::user_login(client, base_url).await?);

        Ok(UserTest {
            client,
            base_url,
            auth_header,
        })
    }
    #[allow(dead_code)]
    pub async fn get_user_profile(&self) -> Result<(), ApiError> {
        let request = self
            .client
            .get(format!("{}/api/user/profile", self.base_url))
            .header("Authorization", self.auth_header.as_str());
        send_request(request).await?;
        Ok(())
    }
    #[allow(dead_code)]
    pub async fn update_user_profile(&self) -> Result<(), ApiError> {
        let request = self
            .client
            .post(format!("{}/api/user/update", self.base_url))
            .header("Authorization", self.auth_header.as_str())
            .json(&json!({
                "username": "lol_updated",
                "email": "lol_updated@gmail.com",
                "first_name": "Updated",
                "last_name": "User",
                "phone_number": "999999999",
                "address": "Updated Address",
                "password": "123123"
            }));
        send_request(request).await?;
        Ok(())
    }
    #[allow(dead_code)]
    async fn user_registration(client: &Client, base_url: &str) -> Result<(), ApiError> {
        let request = client
            .post(format!("{}/api/user/registration", base_url))
            .json(&json!({
                "username": "lol",
                "email": "lol@gmail.com",
                "first_name": "123123",
                "last_name": "123123",
                "phone_number": "1231232",
                "password": "123123",
                "address": ""
            }));
        send_request(request).await?;
        Ok(())
    }
    #[allow(dead_code)]
    async fn user_login(client: &Client, base_url: &str) -> Result<String, ApiError> {
        let request = client
            .post(format!("{}/api/user/login", base_url))
            .json(&json!({
                "email": "lol@gmail.com",
                "password": "123123"
            }));
        let response_text = send_request(request).await?;
        let login_json: serde_json::Value = serde_json::from_str(&response_text).unwrap();
        let token = login_json["token"].as_str().ok_or(ApiError::Unauthorized)?;
        println!("Received JWT token: {}", token);
        Ok(token.to_string())
    }
}
