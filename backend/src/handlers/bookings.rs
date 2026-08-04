use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::repair::{RepairBooking, CreateBookingRequest, UpdateStatusRequest},
    errors::AppError,
};

pub async fn create_booking(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateBookingRequest>,
) -> Result<(StatusCode, Json<RepairBooking>), AppError> {
    let booking_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let initial_status = "PENDING".to_string();

    let booking = sqlx::query_as::<_, RepairBooking>(
        r#"
        INSERT INTO repair_bookings (
            id, user_id, technician_id, device_model, issue_description, status, agreed_price, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, technician_id, device_model, issue_description, status, agreed_price, created_at
        "#,
    )
    .bind(booking_id)
    .bind(payload.user_id)
    .bind(payload.technician_id)
    .bind(&payload.device_model)
    .bind(&payload.issue_description)
    .bind(initial_status)
    .bind(payload.agreed_price)
    .bind(now)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(booking)))
}

pub async fn update_booking_status(
    State(pool): State<PgPool>,
    Path(booking_id): Path<Uuid>,
    Json(payload): Json<UpdateStatusRequest>,
) -> Result<Json<RepairBooking>, AppError> {
    let booking = sqlx::query_as::<_, RepairBooking>(
        r#"
        UPDATE repair_bookings
        SET status = $1
        WHERE id = $2
        RETURNING id, user_id, technician_id, device_model, issue_description, status, agreed_price, created_at
        "#,
    )
    .bind(&payload.status)
    .bind(booking_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Booking not found".to_string()))?;

    Ok(Json(booking))
}

