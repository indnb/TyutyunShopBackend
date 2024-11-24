use once_cell::sync::OnceCell;
use std::env;

pub static CONFIG: OnceCell<EnvConfiguration> = OnceCell::new();
pub struct EnvConfiguration {
    pub main_database_url: String,
    pub database_url: String,
    pub database_name: String,
    pub server_port: String,
    pub server_address: String,
    pub smtp_address: String,
    pub smtp_port: String,
    pub mail_username: String,
    pub mail_password: String,
    pub jwt_secret: String,
    pub admin_role: String,
    pub admin_password: String,
    pub dir_product_images: String,
    pub local: bool,
}

impl EnvConfiguration {
    pub fn init_config() {
        dotenv::dotenv().ok();
        CONFIG.get_or_init(|| EnvConfiguration {
            main_database_url: env::var("MAIN_DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost/postgres".to_string()),
            database_url: env::var("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost/database".to_string()),
            database_name: env::var("DATABASE_NAME").unwrap_or("database".to_string()),
            server_port: env::var("SERVER_PORT").unwrap_or(8181.to_string()),
            server_address: env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string()),
            smtp_address: env::var("SMTP_ADDRESS").unwrap_or("mail.mail".to_string()),
            smtp_port: env::var("SMTP_PORT").unwrap_or(587.to_string()),
            mail_username: env::var("MAIL_USERNAME")
                .unwrap_or("example@example.example".to_string()),
            mail_password: env::var("MAIL_PASSWORD").unwrap_or("password".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or("JWT_SECRET".to_string()),
            admin_role: env::var("ADMIN_ROLE").unwrap_or("ROLE".to_string()),
            admin_password: env::var("ADMIN_PASSWORD").unwrap_or("P@$$W0RD".to_string()),
            dir_product_images: env::var("DIR_PRODUCT_IMAGES")
                .unwrap_or("product_images".to_string()),
            local: env::var("LOCAL")
                .unwrap_or("false".to_string())
                .parse::<bool>()
                .unwrap_or(false),
        });
    }
}
