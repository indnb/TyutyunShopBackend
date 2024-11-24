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
    use crate::utils::env_configuration::{EnvConfiguration, CONFIG};
    use reqwest::Client;
    use rocket::State;
    use sqlx::Error;
    use std::fs;
    use std::path::Path;
    use std::time::Duration;
    use tokio::time::sleep;
    use crate::utils::constants::routes::PATH_PRODUCT_IMAGES;

    #[tokio::test]
    async fn bootstrap_test() -> Result<(), ApiError> {
        EnvConfiguration::init_config();
        let db_pool = init_db_pool()
            .await
            .map_err(|_| ApiError::DatabaseError(Error::RowNotFound))?;
        let db_ref = &db_pool;

        if !Path::new(PATH_PRODUCT_IMAGES).exists() {
            fs::create_dir(PATH_PRODUCT_IMAGES)
                .expect("Failed to create images directory");
        }

        tokio::spawn({
            let db_pool_clone = db_pool.clone();
            async move {
                set_up_rocket(db_pool_clone).await;
            }
        });

        sleep(Duration::from_secs(1)).await;

        let client = Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| ApiError::BadRequest)?;

        let base_url = format!(
            "http://{}:{}",
            CONFIG.get().unwrap().server_address,
            CONFIG.get().unwrap().server_port
        );

        let user_test = UserTest::new(State::from(db_ref), &client, &base_url).await?;
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
