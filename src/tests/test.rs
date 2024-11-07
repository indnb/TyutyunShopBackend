#[cfg(test)]
mod test {
    use super::*;
    use crate::database::init_db_pool;
    use crate::server::set_up_rocket;
    use crate::tests::constant_images::{CAP_BLACK, CAP_BLACK2, CAP_BLACK_MODEL, CAP_BLACK_MODEL2};
    use crate::utils::constants::images_constants::PRODUCT_IMAGES;
    use reqwest::{multipart, Client};
    use rocket::serde::json::json;
    use std::error::Error;
    use std::path::Path;
    use std::time::Duration;
    use std::{env, fs};
    use tokio::time::sleep;
    #[tokio::test]
    async fn bootstrap_test() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().ok();
        let db_pool = init_db_pool().await;

        if !Path::new(PRODUCT_IMAGES).exists() {
            fs::create_dir(PRODUCT_IMAGES).expect("Failed to create images directory");
        }

        let server_handle = tokio::spawn(async move {
            set_up_rocket(db_pool.unwrap()).await;
        });

        sleep(Duration::from_secs(1)).await;

        let client = Client::builder().cookie_store(true).build()?;

        let base_url = format!(
            "http://{}:{}",
            env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string()),
            env::var("SERVER_PORT").unwrap_or("8181".to_string())
        );
        let images_dir = format!(
            "./{}",
            env::var("TEST_IMAGES_DIR").unwrap_or("test_images".to_string())
        );
        let client = Client::builder().cookie_store(true).build()?;

        user_registration(&client, &base_url).await?;
        let token = user_login(&client, &base_url).await?;
        let auth_header = format!("Bearer {}", token);

        get_user_profile(&client, &base_url, &auth_header).await?;
        update_user_profile(&client, &base_url, &auth_header).await?;
        create_category(&client, &base_url, &auth_header).await?;
        upload_image(
            &client,
            &base_url,
            &auth_header,
            &images_dir,
            CAP_BLACK,
            None,
        )
        .await?;
        create_product(&client, &base_url, &auth_header).await?;
        create_sizes(&client, &base_url, &auth_header).await?;

        for image_name in [CAP_BLACK2, CAP_BLACK_MODEL, CAP_BLACK_MODEL2].iter() {
            upload_image(
                &client,
                &base_url,
                &auth_header,
                &images_dir,
                image_name,
                Some(&1),
            )
            .await?;
        }

        get_product_by_id(&client, &base_url).await?;

        Ok(())
    }
    async fn send_request(
        _: &Client,
        request: reqwest::RequestBuilder,
    ) -> Result<String, Box<dyn Error>> {
        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;
        println!("Status: {}, Response: {}", status, text);
        Ok(text)
    }
    async fn user_registration(client: &Client, base_url: &str) -> Result<(), Box<dyn Error>> {
        println!("1. User Registration");
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
        send_request(client, request).await?;
        Ok(())
    }

    async fn user_login(client: &Client, base_url: &str) -> Result<String, Box<dyn Error>> {
        println!("\n2. User Login");
        let request = client
            .post(format!("{}/api/user/login", base_url))
            .json(&json!({
                "email": "lol@gmail.com",
                "password": "123123"
            }));
        let response_text = send_request(client, request).await?;
        let login_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let token = login_json["token"]
            .as_str()
            .ok_or("Failed to extract token")?;
        println!("Received JWT token: {}", token);
        Ok(token.to_string())
    }

    async fn get_user_profile(
        client: &Client,
        base_url: &str,
        auth_header: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("\n3. Get User Profile");
        let request = client
            .get(format!("{}/api/user/profile", base_url))
            .header("Authorization", auth_header);
        send_request(client, request).await?;
        Ok(())
    }

    async fn update_user_profile(
        client: &Client,
        base_url: &str,
        auth_header: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("\n4. Update User Profile");
        let request = client
            .post(format!("{}/api/user/update", base_url))
            .header("Authorization", auth_header)
            .json(&json!({
                "username": "lol_updated",
                "email": "lol_updated@gmail.com",
                "first_name": "Updated",
                "last_name": "User",
                "phone_number": "999999999",
                "address": "Updated Address",
                "password": "123123"
            }));
        send_request(client, request).await?;
        Ok(())
    }

    async fn create_category(
        client: &Client,
        base_url: &str,
        auth_header: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("\n5. Create Category");
        let request = client
            .post(format!("{}/api/category", base_url))
            .header("Authorization", auth_header)
            .json(&json!({ "name": "Кепки" }));
        send_request(client, request).await?;
        Ok(())
    }

    async fn download_image_from_drive(file_id: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let url = format!("https://drive.google.com/uc?export=download&id={}", file_id);
        let response = reqwest::get(&url).await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    async fn upload_image(
        client: &Client,
        base_url: &str,
        auth_header: &str,
        images_dir: &str,
        file_id: &str,
        product_id: Option<&i32>,
    ) -> Result<(), Box<dyn Error>> {
        println!("\n6. Upload Product Image Id: {}", file_id);
        let image_data = download_image_from_drive(file_id).await?;

        let form = match product_id {
            None => multipart::Form::new().part(
                "image",
                multipart::Part::bytes(image_data).mime_str("image/jpeg")?,
            ),
            Some(id) => multipart::Form::new()
                .text("product_id", id.to_string())
                .part(
                    "image",
                    multipart::Part::bytes(image_data).mime_str("image/jpeg")?,
                ),
        };

        let request = client
            .post(format!("{}/api/product_image", base_url))
            .header("Authorization", auth_header)
            .multipart(form);

        send_request(client, request).await?;
        Ok(())
    }

    async fn create_product(
        client: &Client,
        base_url: &str,
        auth_header: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("\n7. Create Product");
        let request = client
            .post(format!("{}/api/product", base_url))
            .header("Authorization", auth_header)
            .json(&json!({
                "name": "Кепка \"Кепкую\"",
                "description": "Пасує всім)",
                "primary_image_id": 1,
                "price": 700,
                "stock_quantity": 222,
                "category_id": 1
            }));
        send_request(client, request).await?;
        Ok(())
    }

    async fn create_sizes(
        client: &Client,
        base_url: &str,
        auth_header: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("\n8. Create Sizes");
        let request = client
            .post(format!("{}/api/size", base_url))
            .header("Authorization", auth_header)
            .json(&json!({
                "product_id": 1,
                "single_size": 100,
                "s": 0,
                "m": 0,
                "l": 0,
                "xl": 0,
                "xxl": 0
            }));
        send_request(client, request).await?;
        Ok(())
    }

    async fn get_product_by_id(client: &Client, base_url: &str) -> Result<(), Box<dyn Error>> {
        println!("\n9. Get Product by ID");
        let request = client.get(format!("{}/api/product/1", base_url));
        send_request(client, request).await?;
        Ok(())
    }
}
