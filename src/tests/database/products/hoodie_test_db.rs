use crate::data::products_components::product::Product;
use crate::data::products_components::size::Size;
use crate::error::api_error::ApiError;
use crate::tests::database::products::product_test_db::create_product;
use crate::tests::database::products::property::constant_images::*;
use crate::tests::database::products::property::image_test_db::upload_image;
use crate::tests::database::products::property::size_test_db::create_sizes;
use crate::tests::database::user_test_db::UserTest;

#[allow(dead_code)]
pub async fn create_hoodie_black<'a>(user_test: &UserTest<'a>) -> Result<(), ApiError> {
    upload_image(user_test, HOODIE_BLACK, None).await?;
    create_product(
        user_test,
        &Product {
            id: None,
            name: "Худі \"Залежність\" Чорне".to_string(),
            description: Some("Матеріал принту: ДТФ".to_string()),
            primary_image_id: Some(21),
            price: 1700_f32,
            size_id: None,
            category_id: Some(3),
        },
    )
    .await?;
    create_sizes(
        user_test,
        &Size {
            product_id: 6,
            single_size: None,
            s: Some(0),
            m: Some(0),
            l: Some(10),
            xl: Some(20),
            xxl: Some(0),
        },
    )
    .await?;

    for image_name in [HOODIE_BLACK2, HOODIE_BLACK3, HOODIE_SIZE].iter() {
        upload_image(user_test, image_name, Some(&6)).await?;
    }
    Ok(())
}
