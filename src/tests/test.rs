#[cfg(test)]
mod test {
    use crate::database::init_db_pool;
    use crate::error::api_error::ApiError;
    use crate::server::set_up_rocket;
    use crate::tests::database::products::cap_test_db::*;
    use crate::tests::database::products::hoodie_test_db::create_hoodie_black;
    use crate::tests::database::products::product_test_db::*;
    use crate::tests::database::products::property::category_test_db::*;
    use crate::tests::database::products::t_shirt_test_db::*;
    use crate::tests::database::user_test_db::*;
    use crate::utils::constants::images_constants::PRODUCT_IMAGES;
    use reqwest::Client;
    use std::path::Path;
    use std::time::Duration;
    use std::{env, fs};
    use tokio::time::sleep;

    #[tokio::test]
    async fn bootstrap_test() -> Result<(), ApiError> {
        dotenv::dotenv().ok();
        let db_pool = init_db_pool().await;

        if !Path::new(PRODUCT_IMAGES).exists() {
            fs::create_dir(PRODUCT_IMAGES).expect("Failed to create images directory");
        }

        tokio::spawn(async move {
            set_up_rocket(db_pool.unwrap()).await;
        });

        sleep(Duration::from_secs(1)).await;

        let client = Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| ApiError::BadRequest)?;

        let base_url = format!(
            "http://{}:{}",
            env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string()),
            env::var("SERVER_PORT").unwrap_or("8181".to_string())
        );

        let user_test = UserTest::new(&client, &base_url).await?;
        user_test.update_user_profile().await?;
        user_test.get_user_profile().await?;
        create_category(&user_test, "Кепки").await?;
        create_cap_black(&user_test).await?;
        create_cap_red(&user_test).await?;
        create_cap_beige(&user_test).await?;
        create_category(&user_test, "Футболки").await?;
        create_t_shirt_black(&user_test).await?;
        create_t_shirt_white(&user_test).await?;
        create_category(&user_test, "Худі").await?;
        create_hoodie_black(&user_test).await?;

        get_product_by_id(&user_test).await?;

        Ok(())
    }
}
