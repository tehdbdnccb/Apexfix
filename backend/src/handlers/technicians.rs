use axum::{
    extract::{State, Query, Json},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::technician::{Technician, CreateTechnicianRequest, MatchQueryRequest},
    services::{spatial, matching::{rank_technicians, RankedTechnician}},
    error::AppError,
};

pub async fn create_technician(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTechnicianRequest>,
) -> Result<(StatusCode, Json<Technician>), AppError> {
    let technician_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let technician = sqlx::query_as::<_, Technician>(
        r#"
        INSERT INTO technicians (
            id, user_id, shop_name, location_name, latitude, longitude, 
            certification_level, part_authenticity_score, speed_score, is_verified, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, user_id, shop_name, location_name, latitude, longitude, 
                  certification_level, part_authenticity_score, speed_score, is_verified, created_at
        "#,
    )
    .bind(technician_id)
    .bind(payload.user_id)
    .bind(&payload.shop_name)
    .bind(&payload.location_name)
    .bind(payload.latitude)
    .bind(payload.longitude)
    .bind(payload.certification_level)
    .bind(payload.part_authenticity_score)
    .bind(payload.speed_score)
    .bind(true)
    .bind(now)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(technician)))
}

pub async fn match_technicians(
    State(pool): State<PgPool>,
    Query(params): Query<MatchQueryRequest>,
) -> Result<Json<Vec<RankedTechnician>>, AppError> {
    let max_distance = params.max_distance_km.unwrap_or(25.0);

    let techs_with_dist = spatial::find_technicians_within_radius(
        &pool,
        params.latitude,
        params.longitude,
        max_distance,
    ).await?;

    let ranked = rank_technicians(techs_with_dist, None);

    Ok(Json(ranked))
}