use rocket::fs::TempFile;

#[derive(Debug, FromForm)]
pub struct ProductImage<'r> {
    pub image: TempFile<'r>,
}