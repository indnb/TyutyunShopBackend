use crate::data::user_components::authorization::{LoginRequest, LoginResponse, RoleResponse};
use crate::data::user_components::claims::Claims;
use crate::data::user_components::user::{TempUser, User, UserProfile};
use crate::error::api_error::ApiError;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};
use std::env;
#[get("/user/role")]
pub async fn get_user_role(
    db_pool: &State<PgPool>,
    claims: Claims,
) -> Result<Json<RoleResponse>, Status> {
    let result = sqlx::query(
        r#"
        SELECT role FROM users WHERE id = $1
        "#,
    )
    .bind(claims.sub)
    .fetch_one(&**db_pool)
    .await
    .map_err(|_| ApiError::Unauthorized);

    match result {
        Ok(record) => Ok(Json(RoleResponse {
            role: record.get("role"),
        })),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[post("/user/login", data = "<login_data>")]
pub async fn login(
    db_pool: &State<PgPool>,
    login_data: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let login_data = login_data.into_inner();

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE email = $1
        "#,
    )
    .bind(&login_data.email)
    .fetch_one(&**db_pool)
    .await
    .map_err(|_| ApiError::NotFound)?;

    let is_password_valid = verify(&login_data.password, &user.password_hash)
        .map_err(|_| ApiError::InternalServerError)?;

    if !is_password_valid {
        return Err(ApiError::Unauthorized);
    }

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let claims = Claims::new(user.id);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(LoginResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        token,
    }))
}

#[get("/user/profile")]
pub async fn get_profile(
    claims: Claims,
    db_pool: &State<PgPool>,
) -> Result<Json<UserProfile>, Status> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, password_hash, username, first_name, last_name, email, phone_number, address
        FROM users WHERE id = $1
        "#,
    )
    .bind(claims.sub)
    .fetch_one(&**db_pool)
    .await
    .map_err(|_| Status::NotFound)?;

    Ok(Json(UserProfile {
        id: Option::from(user.id),
        username: user.username,
        email: user.email,
        first_name: user.first_name.unwrap_or_default(),
        last_name: user.last_name.unwrap_or_default(),
        phone_number: user.phone_number.unwrap_or_default(),
        address: user.address.unwrap_or_default(),
    }))
}

#[post("/user/registration", data = "<user_data>")]
pub async fn registration(
    db_pool: &State<PgPool>,
    user_data: Json<TempUser>,
) -> Result<(), ApiError> {
    let new_user = user_data.into_inner();

    sqlx::query(
        r#"
        INSERT INTO users (
            username, email, password_hash, first_name, last_name, phone_number, role, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
        "#
    ).bind(new_user.username)
        .bind(new_user.email)
        .bind(hash(new_user.password.unwrap(), DEFAULT_COST).expect("password hash should be valid"))
        .bind(new_user.first_name)
        .bind(new_user.last_name)
        .bind(new_user.phone_number)
        .bind(new_user.role.unwrap_or("USER".to_string()))
        .execute(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

    Ok(())
}

#[post("/user/update", data = "<user_data>")]
pub async fn update_profile(
    db_pool: &State<PgPool>,
    user_data: Json<TempUser>,
    claims: Claims,
) -> Result<Json<&'static str>, ApiError> {
    let temp_user = user_data.into_inner();

    let user_exists = sqlx::query("SELECT id FROM users WHERE id = $1")
        .bind(claims.sub)
        .fetch_optional(&**db_pool)
        .await?
        .is_some();

    if !user_exists {
        return Err(ApiError::BadRequest);
    }

    sqlx::query(
        r#"
        UPDATE users
        SET username = $1,
            email = $2,
            first_name = $3,
            last_name = $4,
            phone_number = $5,
            address = $6,
            role = $8,
            updated_at = NOW()
        WHERE id = $7
        "#,
    )
    .bind(temp_user.username)
    .bind(temp_user.email)
    .bind(temp_user.first_name)
    .bind(temp_user.last_name)
    .bind(temp_user.phone_number)
    .bind(temp_user.address)
    .bind(claims.sub)
    .bind(temp_user.role.unwrap_or("USER".to_string()))
    .execute(&**db_pool)
    .await?;

    Ok(Json("Data successfully updated"))
}
