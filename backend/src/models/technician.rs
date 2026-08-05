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
pub struct MatchQueryRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub max_distance_km: Option<f64>,
}

