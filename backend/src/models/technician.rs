use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Technician {
    pub id: Uuid,
    pub user_id: Uuid,
    pub shop_name: String,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub certification_level: f64,
    pub part_authenticity_score: f64,
    pub speed_score: f64,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct TechnicianProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub shop_name: String,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub certification_level: f64,
    pub part_authenticity_score: f64,
    pub speed_score: f64,
    pub is_verified: bool,
    pub years_experience: i32,
    pub specializations: String, // JSON array of specializations
    pub phone_number: String,
    pub shop_address: String,
    pub bio: Option<String>,
    pub rating: Option<f64>,
    pub total_reviews: i32,
    pub completion_rate: f64,
    pub response_time_hours: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTechnicianRequest {
    pub user_id: Uuid,
    pub shop_name: String,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub certification_level: f64,
    pub part_authenticity_score: f64,
    pub speed_score: f64,
}

#[derive(Deserialize)]
pub struct TechnicianOnboardingRequest {
    pub user_id: Uuid,
    pub shop_name: String,
    pub phone_number: String,
    pub shop_address: String,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub years_experience: i32,
    pub specializations: Vec<String>, // e.g., ["iPhone", "Samsung", "iPad"]
    pub certification_level: f64, // 0.0 to 5.0
    pub part_authenticity_score: f64, // 0.0 to 5.0
    pub speed_score: f64, // 0.0 to 5.0
    pub bio: Option<String>,
    pub response_time_hours: Option<i32>,
}

#[derive(Serialize)]
pub struct TechnicianOnboardingResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub shop_name: String,
    pub location_name: String,
    pub is_verified: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct MatchQueryRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub max_distance_km: Option<f64>,
}

#[derive(Serialize)]
pub struct TechnicianMatchResult {
    pub technician: Technician,
    pub distance_km: f64,
    pub match_score: f64,
    pub rating: Option<f64>,
    pub total_reviews: i32,
}

