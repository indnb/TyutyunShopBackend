use rocket::serde::{Deserialize, Serialize};

#[derive(Debug,  Serialize, Deserialize)]
pub struct Category {
    pub name: String,
}
