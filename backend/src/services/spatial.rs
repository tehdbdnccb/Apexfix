use sqlx::PgPool;
use crate::models::technician::Technician;
use crate::error::AppError;

pub async fn find_technicians_within_radius(
    pool: &PgPool,
    user_lat: f64,
    user_lon: f64,
    max_distance_km: f64,
) -> Result<Vec<(Technician, f64)>, AppError> {
    let max_distance_meters = max_distance_km * 1000.0;

    let query = r#"
        SELECT 
            id, user_id, shop_name, location_name, latitude, longitude, 
            certification_level, part_authenticity_score, speed_score, is_verified, created_at,
            ST_DistanceSphere(
                ST_MakePoint($1, $2)::geography,
                ST_MakePoint(longitude, latitude)::geography
            ) / 1000.0 AS distance_km
        FROM technicians
        WHERE ST_DWithin(
            ST_MakePoint($1, $2)::geography,
            ST_MakePoint(longitude, latitude)::geography,
            $3
        )
    "#;

    let rows = sqlx::query_as::<_, (
        uuid::Uuid, uuid::Uuid, String, String, f64, f64, f32, f32, f32, bool, chrono::DateTime<chrono::Utc>, f64
    )>(query)
    .bind(user_lon) // PostGIS expects longitude first (X), then latitude (Y)
    .bind(user_lat)
    .bind(max_distance_meters)
    .fetch_all(pool)
    .await?;

    let mut results = Vec::new();
    for row in rows {
        let technician = Technician {
            id: row.0,
            user_id: row.1,
            shop_name: row.2,
            location_name: row.3,
            latitude: row.4,
            longitude: row.5,
            certification_level: row.6,
            part_authenticity_score: row.7,
            speed_score: row.8,
            is_verified: row.9,
            created_at: row.10,
        };
        let distance_km = row.11;
        results.push((technician, distance_km));
    }

    Ok(results)
}