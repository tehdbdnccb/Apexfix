use sqlx::PgPool;
use crate::models::technician::Technician;
use crate::errors::AppError;

/// Haversine formula to calculate distance between two lat/lon points in km
fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2) +
            lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    EARTH_RADIUS_KM * c
}

pub async fn find_technicians_within_radius(
    pool: &PgPool,
    user_lat: f64,
    user_lon: f64,
    max_distance_km: f64,
) -> Result<Vec<(Technician, f64)>, AppError> {
    // Fetch all technicians (we'll filter by distance in memory)
    let technicians = sqlx::query_as::<_, Technician>(
        r#"
        SELECT 
            id, user_id, shop_name, location_name, latitude, longitude, 
            certification_level, part_authenticity_score, speed_score, is_verified, created_at
        FROM technicians
        WHERE is_verified = true
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    // Calculate distances and filter
    let mut results = Vec::new();
    for tech in technicians {
        let distance = haversine_distance(user_lat, user_lon, tech.latitude, tech.longitude);
        if distance <= max_distance_km {
            results.push((tech, distance));
        }
    }

    Ok(results)
}

