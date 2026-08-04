use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Technician {
    pub id: Uuid,
    pub user_id: Uuid,
    pub shop_name: String,
    pub location_name: String, // e.g., "Mega City, Kisumu", "Milimani"
    pub latitude: f64,
    pub longitude: f64,
    pub certification_level: f32, // e.g., 1.0 for Apple Certified, 0.7 for Independent Expert
    pub part_authenticity_score: f32, // e.g., 0.0 to 1.0 rating based on supply chain ledger
    pub speed_score: f32, // Turnaround efficiency score
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
    pub certification_level: f32,
    pub part_authenticity_score: f32,
    pub speed_score: f32,
}

#[derive(Deserialize)]
pub struct MatchQueryRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub max_distance_km: Option<f64>,
}