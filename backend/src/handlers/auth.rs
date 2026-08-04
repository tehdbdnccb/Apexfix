use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};

use crate::{
    models::user::{RegisterRequest, LoginRequest, User, AuthResponse, UserResponse},
    errors::AppError,
    config::Config,
};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

pub async fn register(
    State(pool): State<PgPool>,
    State(config): State<Config>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| AppError::InternalServerError(format!("Hashing error: {}", e)))?;

    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, password_hash, full_name, phone_number, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, password_hash, full_name, phone_number, created_at
        "#,
    )
    .bind(user_id)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.full_name)
    .bind(&payload.phone_number)
    .bind(now)
    .fetch_one(&pool)
    .await?;

    let token = create_jwt(&user.id, &config.jwt_secret)?;

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            phone_number: user.phone_number,
        },
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(pool): State<PgPool>,
    State(config): State<Config>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, full_name, phone_number, created_at
        FROM users WHERE email = $1
        "#,
    )
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::AuthError("Invalid email or password".to_string()))?;

    let valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::AuthError("Invalid email or password".to_string()))?;

    if !valid {
        return Err(AppError::AuthError("Invalid email or password".to_string()));
    }

    let token = create_jwt(&user.id, &config.jwt_secret)?;

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            phone_number: user.phone_number,
        },
    };

    Ok(Json(response))
}

fn create_jwt(user_id: &Uuid, secret: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: *user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(format!("Token generation error: {}", e)))
}

