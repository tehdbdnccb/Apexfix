mod config;
mod error;
mod db;
mod models;
mod services;
mod handlers;

use axum::{
    extract::FromRef,
    routing::{get, post, put},
    Router,
};
use tower_http::cors::CorsLayer;
use sqlx::PgPool;

use crate::{
    config::Config,
    handlers::{
        auth::{register, login},
        technicians::{create_technician, match_technicians},
        bookings::{create_booking, update_booking_status},
    },
    db::{pool::init_pool, redis::init_redis},
};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    config: Config,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment configuration
    let config = Config::from_env();

    // Initialize PostgreSQL connection pool
    let pool = init_pool(&config).await?;

    // Initialize Redis client
    let _redis_client = init_redis(&config)?;

    let state = AppState {
        pool,
        config,
    };

    // Build Axum router and apply middleware
    let app = Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/technicians", post(create_technician))
        .route("/api/technicians/match", get(match_technicians))
        .route("/api/bookings", post(create_booking))
        .route("/api/bookings/:id/status", put(update_booking_status))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Bind and start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running on http://0.0.0.0:8080");
    axum::serve(listener, app).await?;

    Ok(())
}