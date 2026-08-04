use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/kisumu_apple_repair".to_string());
        
        let redis_url = env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
            
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "super_secret_jwt_key_change_in_production".to_string());
            
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        Ok(Config {
            database_url,
            redis_url,
            jwt_secret,
            port,
        })
    }
}