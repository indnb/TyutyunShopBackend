use crate::data::products_components::product::Product;
use crate::data::products_components::size::Size;
use crate::error::api_error::ApiError;
use crate::tests::database::products::product_test_db::create_product;
use crate::tests::database::products::property::constant_images::*;
use crate::tests::database::products::property::image_test_db::upload_image;
use crate::tests::database::products::property::size_test_db::create_sizes;
use crate::tests::database::user_test_db::UserTest;

#[allow(dead_code)]
pub async fn create_t_shirt_black(user_test: &UserTest<'_>) -> Result<(), ApiError> {
    upload_image(user_test, T_SHIRT_BLACK, None).await?;
    create_product(
        user_test,
        &Product {
            id: None,
            name: "Футболка \"Залежність\" Чорна".to_string(),
            description: Some("Матеріал принту: ДТФ".to_string()),
            primary_image_id: Some(13),
            price: 900_f32,
            size_id: None,
            category_id: Some(2),
        },
    )
    .await?;
    create_sizes(
        user_test,
        &Size {
            product_id: 4,
            single_size: None,
            s: Some(10),
            m: Some(0),
            l: Some(20),
            xl: Some(20),
            xxl: Some(1),
        },
    )
    .await?;

    for image_name in [T_SHIRT_BLACK2, T_SHIRT_BLACK3, T_SHIRT_SIZE].iter() {
        upload_image(user_test, image_name, Some(&4)).await?;
    }
    Ok(())
}
#[allow(dead_code)]
pub async fn create_t_shirt_white(user_test: &UserTest<'_>) -> Result<(), ApiError> {
    upload_image(user_test, T_SHIRT_WHITE, None).await?;
    create_product(
        user_test,
        &Product {
            id: None,
            name: "Футболка \"Залежність\" Біла".to_string(),
            description: Some("Матеріал принту: ДТФ".to_string()),
            primary_image_id: Some(17),
            price: 900_f32,
            size_id: None,
            category_id: Some(2),
        },
    )
    .await?;
    create_sizes(
        user_test,
        &Size {
            product_id: 5,
            single_size: None,
            s: Some(0),
            m: Some(10),
            l: Some(0),
            xl: Some(20),
            xxl: Some(1),
        },
    )
    .await?;

    for image_name in [T_SHIRT_WHITE2, T_SHIRT_WHITE3, T_SHIRT_SIZE].iter() {
        upload_image(user_test, image_name, Some(&5)).await?;
    }
    Ok(())
}
