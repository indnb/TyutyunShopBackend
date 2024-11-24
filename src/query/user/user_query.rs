use crate::data::user_components::authorization::{LoginRequest, LoginResponse, RoleResponse};
use crate::data::user_components::claims::Claims;
use crate::data::user_components::user::{JwtUser, TempUser, User, UserProfile};
use crate::error::api_error::ApiError;
use crate::mail::sender::{generate_registration_link, send_mail_registration};
use crate::utils::constants::routes::LOGIN;
use crate::utils::env_configuration::CONFIG;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{PgPool, Row};

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

    let secret = CONFIG.get().unwrap().jwt_secret.as_str();
    let claims = Claims::new(user.id, user.role);
    let token = encode(
        &Header::new(Algorithm::HS512),
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
    db_pool: &State<PgPool>,
    claims: Claims,
) -> Result<Json<UserProfile>, ApiError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, password_hash, username, first_name, last_name, email, phone_number, address, role
        FROM users WHERE id = $1
        "#,
    )
        .bind(claims.sub)
        .fetch_one(&**db_pool)
        .await?;

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

pub async fn registration(db_pool: &State<PgPool>, user_data: TempUser) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO users (
            username, email, password_hash, first_name, last_name, phone_number, role, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
        "#
    ).bind(user_data.username)
        .bind(user_data.email)
        .bind(hash(user_data.password.unwrap(), DEFAULT_COST).expect("password hash should be valid"))
        .bind(user_data.first_name)
        .bind(user_data.last_name)
        .bind(user_data.phone_number)
        .bind(user_data.role.unwrap_or("USER".to_string()))
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
    let mut temp_user = user_data.into_inner();
    temp_user.role = claims.role;
    if temp_user.role != Some(CONFIG.get().unwrap().admin_role.to_string()) {
        temp_user.role = Some("USER".to_string());
    }
    let user_exists = sqlx::query("SELECT id FROM users WHERE id = $1")
        .bind(claims.sub)
        .fetch_optional(&**db_pool)
        .await?
        .is_some();

    if !user_exists {
        return Err(ApiError::Unauthorized);
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
#[post("/user/try_registration", data = "<user_data>")]
pub async fn try_registration(
    db_pool: &State<PgPool>,
    user_data: Json<TempUser>,
) -> Result<(), ApiError> {
    let mut new_user = user_data.into_inner();
    new_user.role = Some("USER".to_string());
    let exist = sqlx::query(
        r#"
        SELECT email, phone_number, username FROM users
        WHERE email = $1 OR phone_number = $2 OR username = $3
        "#,
    )
    .bind(&new_user.email)
    .bind(&new_user.phone_number)
    .bind(&new_user.username)
    .fetch_optional(&**db_pool)
    .await?;

    if let Some(row) = exist {
        let existing_email: String = row.get("email");
        if existing_email == new_user.email {
            return Err(ApiError::EmailError);
        }
        let existing_phone: String = row.get("phone_number");
        if existing_phone == new_user.phone_number.clone().unwrap_or_default() {
            return Err(ApiError::PhoneError);
        }
        let existing_username: String = row.get("username");
        if existing_username == new_user.username {
            return Err(ApiError::UsernameError);
        }
    }

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(5))
        .expect("Failed to compute expiration time")
        .timestamp() as usize;

    let new_user = JwtUser {
        username: new_user.username,
        email: new_user.email.clone(),
        password: new_user.password,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        phone_number: new_user.phone_number,
        role: new_user.role,
        exp: expiration,
        address: None,
    };

    let header = Header::new(Algorithm::HS256);
    let token = encode(
        &header,
        &new_user,
        &EncodingKey::from_secret(CONFIG.get().unwrap().jwt_secret.as_ref()),
    )
    .map_err(|_| ApiError::InternalServerError)?;

    send_mail_registration(new_user.email, generate_registration_link(token))?;
    Ok(())
}

#[allow(dead_code)]
#[get("/registration?<token>")]
pub async fn registration_by_token(
    db_pool: &State<PgPool>,
    token: String,
) -> Result<Redirect, ApiError> {
    let decoded = match decode::<JwtUser>(
        &token,
        &DecodingKey::from_secret(CONFIG.get().unwrap().jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(user) => user,
        Err(_) => return Ok(Redirect::to(format!("{}/", LOGIN))),
    };
    let current_time = chrono::Utc::now().timestamp() as usize;
    if decoded.claims.exp < current_time {
        return Ok(Redirect::to(format!("{}/", LOGIN)));
    }

    registration(
        db_pool,
        TempUser {
            username: decoded.claims.username,
            email: decoded.claims.email,
            password: decoded.claims.password,
            first_name: decoded.claims.first_name,
            last_name: decoded.claims.last_name,
            phone_number: decoded.claims.phone_number,
            role: decoded.claims.role,
            address: decoded.claims.address,
        },
    )
    .await?;

    Ok(Redirect::to(format!("{}/", LOGIN)))
}

#[post("/user/update_password?<old_password>&<new_password>")]
pub async fn update_password(
    db_pool: &State<PgPool>,
    old_password: Option<String>,
    new_password: Option<String>,
    claims: Claims,
) -> Result<Json<&'static str>, ApiError> {
    let old_password = match old_password {
        Some(p) => p,
        None => return Err(ApiError::BadRequest),
    };

    let new_password = match new_password {
        Some(p) => p,
        None => return Err(ApiError::BadRequest),
    };

    let user_exists = sqlx::query("SELECT id, password_hash FROM users WHERE id = $1")
        .bind(claims.sub)
        .fetch_optional(&**db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

    let user = match user_exists {
        Some(user) => user,
        None => return Err(ApiError::Unauthorized),
    };

    let stored_password_hash = user.get::<String, &str>("password_hash");

    let is_valid =
        verify(&old_password, &stored_password_hash).map_err(|_| ApiError::InternalServerError)?;

    if !is_valid {
        return Err(ApiError::Unauthorized);
    }

    let new_password_hash =
        hash(&new_password, DEFAULT_COST).map_err(|_| ApiError::InternalServerError)?;

    sqlx::query(
        r#"
        UPDATE users
        SET password_hash = $2,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(claims.sub)
    .bind(new_password_hash)
    .execute(&**db_pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok(Json("Password successfully updated"))
}
