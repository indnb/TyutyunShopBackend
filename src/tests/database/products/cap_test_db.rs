use crate::data::products_components::product::Product;
use crate::data::products_components::size::Size;
use crate::error::api_error::ApiError;
use crate::tests::database::products::product_test_db::create_product;
use crate::tests::database::products::property::constant_images::*;
use crate::tests::database::products::property::image_test_db::upload_image;
use crate::tests::database::products::property::size_test_db::create_sizes;
use crate::tests::database::user_test_db::UserTest;

#[allow(dead_code)]
pub async fn create_cap_red(user_test: &UserTest<'_>) -> Result<(), ApiError> {
    upload_image(user_test, CAP_RED, None).await?;
    create_product(
        user_test,
        &Product {
            id: None,
            name: "Кепка \"Кепкую\" Червона".to_string(),
            description: Some(String::from(
                "Безрозмірна, з можливістю регулювання, матеріал принту вишивка",
            )),
            primary_image_id: Some(5),
            price: 700f32,
            size_id: None,
            category_id: Some(1),
        },
    )
    .await?;
    create_sizes(
        user_test,
        &Size {
            product_id: 2,
            single_size: Some(200),
            s: Some(0),
            m: Some(0),
            l: Some(0),
            xl: Some(0),
            xxl: Some(0),
        },
    )
    .await?;

    for image_name in [CAP_RED2, CAP_RED_MODEL, CAP_RED_MODEL2].iter() {
        upload_image(user_test, image_name, Some(&2)).await?;
    }
    Ok(())
}
#[allow(dead_code)]
pub async fn create_cap_beige(user_test: &UserTest<'_>) -> Result<(), ApiError> {
    upload_image(user_test, CAP_BEIGE, None).await?;
    create_product(
        user_test,
        &Product {
            id: None,
            name: "Кепка \"Кепкую\" Бежева".to_string(),
            description: Some(String::from(
                "Безрозмірна, з можливістю регулювання, матеріал принту вишивка",
            )),
            primary_image_id: Some(9),
            price: 700f32,
            size_id: None,
            category_id: Some(1),
        },
    )
    .await?;
    create_sizes(
        user_test,
        &Size {
            product_id: 3,
            single_size: Some(300),
            s: Some(0),
            m: Some(0),
            l: Some(0),
            xl: Some(0),
            xxl: Some(0),
        },
    )
    .await?;

    for image_name in [CAP_BEIGE2, CAP_BEIGE_MODEL, CAP_BEIGE_MODEL2].iter() {
        upload_image(user_test, image_name, Some(&3)).await?;
    }
    Ok(())
}
#[allow(dead_code)]
pub async fn create_cap_black(user_test: &UserTest<'_>) -> Result<(), ApiError> {
    upload_image(user_test, CAP_BLACK, None).await?;
    create_product(
        user_test,
        &Product {
            id: None,
            name: "Кепка \"Кепкую\" Чорна".to_string(),
            description: Some(String::from(
                "Безрозмірна, з можливістю регулювання, матеріал принту вишивка",
            )),
            primary_image_id: Some(1),
            price: 700_f32,
            size_id: None,
            category_id: Some(1),
        },
    )
    .await?;
    create_sizes(
        user_test,
        &Size {
            product_id: 1,
            single_size: Some(145),
            s: Some(0),
            m: Some(0),
            l: Some(0),
            xl: Some(0),
            xxl: Some(0),
        },
    )
    .await?;

    for image_name in [CAP_BLACK2, CAP_BLACK_MODEL, CAP_BLACK_MODEL2].iter() {
        upload_image(user_test, image_name, Some(&1)).await?;
    }
    Ok(())
}
