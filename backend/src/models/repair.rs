use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct RepairBooking {
    pub id: Uuid,
    pub user_id: Uuid,
    pub technician_id: Uuid,
    pub device_model: String,
    pub issue_description: String,
    pub status: String, // "PENDING", "ESCROW_LOCKED", "IN_REPAIR", "COMPLETED", "CANCELLED"
    pub agreed_price: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateBookingRequest {
    pub user_id: Uuid,
    pub technician_id: Uuid,
    pub device_model: String,
    pub issue_description: String,
    pub agreed_price: f64,
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

