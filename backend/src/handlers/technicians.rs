use axum::{
    extract::{State, Query, Json},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::{
    models::technician::{
        Technician, CreateTechnicianRequest, MatchQueryRequest, 
        TechnicianOnboardingRequest, TechnicianOnboardingResponse,
        TechnicianMatchResult,
    },
    services::{spatial, matching::{rank_technicians, RankedTechnician}},
    errors::AppError,
};

// Validate onboarding data
fn validate_scores(cert: f64, auth: f64, speed: f64) -> Result<(), AppError> {
    const MIN_SCORE: f64 = 0.0;
    const MAX_SCORE: f64 = 5.0;

    if cert < MIN_SCORE || cert > MAX_SCORE {
        return Err(AppError::BadRequest(
            format!("Certification level must be between {} and {}", MIN_SCORE, MAX_SCORE)
        ));
    }
    if auth < MIN_SCORE || auth > MAX_SCORE {
        return Err(AppError::BadRequest(
            format!("Part authenticity score must be between {} and {}", MIN_SCORE, MAX_SCORE)
        ));
    }
    if speed < MIN_SCORE || speed > MAX_SCORE {
        return Err(AppError::BadRequest(
            format!("Speed score must be between {} and {}", MIN_SCORE, MAX_SCORE)
        ));
    }
    Ok(())
}

pub async fn onboard_technician(
    State(pool): State<PgPool>,
    Json(payload): Json<TechnicianOnboardingRequest>,
) -> Result<(StatusCode, Json<TechnicianOnboardingResponse>), AppError> {
    // Validate scores
    validate_scores(
        payload.certification_level,
        payload.part_authenticity_score,
        payload.speed_score,
    )?;

    // Validate experience
    if payload.years_experience < 0 || payload.years_experience > 100 {
        return Err(AppError::BadRequest(
            "Years of experience must be between 0 and 100".to_string()
        ));
    }

    // Validate location
    if payload.latitude < -90.0 || payload.latitude > 90.0 {
        return Err(AppError::BadRequest(
            "Latitude must be between -90 and 90".to_string()
        ));
    }
    if payload.longitude < -180.0 || payload.longitude > 180.0 {
        return Err(AppError::BadRequest(
            "Longitude must be between -180 and 180".to_string()
        ));
    }

    let technician_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let specializations_json = serde_json::to_string(&payload.specializations)
        .map_err(|_| AppError::InternalError("Failed to serialize specializations".to_string()))?;

    let response_time = payload.response_time_hours.unwrap_or(24);

    let result = sqlx::query(
        r#"
        INSERT INTO technicians (
            id, user_id, shop_name, phone_number, shop_address, location_name, 
            latitude, longitude, certification_level, part_authenticity_score, speed_score,
            years_experience, specializations, bio, response_time_hours,
            is_verified, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
        RETURNING id, user_id, shop_name, location_name, is_verified
        "#,
    )
    .bind(technician_id)
    .bind(payload.user_id)
    .bind(&payload.shop_name)
    .bind(&payload.phone_number)
    .bind(&payload.shop_address)
    .bind(&payload.location_name)
    .bind(payload.latitude)
    .bind(payload.longitude)
    .bind(payload.certification_level)
    .bind(payload.part_authenticity_score)
    .bind(payload.speed_score)
    .bind(payload.years_experience)
    .bind(&specializations_json)
    .bind(&payload.bio)
    .bind(response_time)
    .bind(false) // Start unverified
    .bind(now)
    .bind(now)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") {
            AppError::BadRequest("Technician profile already exists".to_string())
        } else {
            AppError::from(e)
        }
    })?;

    let (id, user_id, shop_name, location_name, is_verified): (Uuid, Uuid, String, String, bool) =
        sqlx::query_as("SELECT id, user_id, shop_name, location_name, is_verified FROM technicians WHERE id = $1")
            .bind(technician_id)
            .fetch_one(&pool)
            .await?;

    Ok((
        StatusCode::CREATED,
        Json(TechnicianOnboardingResponse {
            id,
            user_id,
            shop_name,
            location_name,
            is_verified,
            message: "Profile created successfully! You'll be verified by our team within 24 hours.".to_string(),
        }),
    ))
}

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
) -> Result<Json<Vec<TechnicianMatchResult>>, AppError> {
    let max_distance = params.max_distance_km.unwrap_or(25.0);

    let techs_with_dist = spatial::find_technicians_within_radius(
        &pool,
        params.latitude,
        params.longitude,
        max_distance,
    ).await?;

    let ranked = rank_technicians(techs_with_dist, None);

    // Fetch ratings for each technician
    let results: Vec<TechnicianMatchResult> = futures::future::join_all(
        ranked.iter().map(|rt| async {
            let rating_result: Result<(Option<f64>, i32), _> = sqlx::query_as(
                "SELECT rating, total_reviews FROM technicians WHERE id = $1"
            )
            .bind(&rt.technician.id)
            .fetch_optional(&pool)
            .await
            .map(|opt| opt.unwrap_or((None, 0)));

            let (rating, total_reviews) = rating_result.unwrap_or((None, 0));

            TechnicianMatchResult {
                technician: rt.technician.clone(),
                distance_km: rt.distance_km,
                match_score: rt.match_score,
                rating,
                total_reviews,
            }
        })
    ).await;

    Ok(Json(results))
}

