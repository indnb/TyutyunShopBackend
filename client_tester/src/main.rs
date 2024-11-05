use reqwest::multipart;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8181".to_string());
    let images_dir = env::var("IMAGES_DIR").unwrap_or_else(|_| "./images".to_string());
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
        "cap_black.jpg",
        None,
    )
    .await?;
    create_product(&client, &base_url, &auth_header).await?;
    create_sizes(&client, &base_url, &auth_header).await?;

    for image_name in [
        "cap_black2.jpg",
        "cap_black_model.jpg",
        "cap_black_model2.jpg",
    ]
    .iter()
    {
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

async fn upload_image(
    client: &Client,
    base_url: &str,
    auth_header: &str,
    images_dir: &str,
    image_name: &str,
    product_id: Option<&i32>,
) -> Result<(), Box<dyn Error>> {
    println!("\n6. Upload Product Image: {}", image_name);
    let form = match product_id {
        None => {
            multipart::Form::new()
                .file("image", format!("{}/{}", images_dir, image_name))
                .await?
        }
        Some(id) => {
            multipart::Form::new()
                .text("product_id", id.to_string())
                .file("image", format!("{}/{}", images_dir, image_name))
                .await?
        }
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
            "s": true,
            "m": true,
            "l": true,
            "xl": true,
            "xxl": false
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
