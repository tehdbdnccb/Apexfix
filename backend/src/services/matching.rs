use crate::models::technician::Technician;

#[derive(Clone, Copy)]
pub struct MatchWeights {
    pub w1: f32, // Certification weight
    pub w2: f32, // Part Authenticity weight
    pub w3: f32, // Speed weight
    pub w4: f32, // Distance penalty weight
}

impl Default for MatchWeights {
    fn default() -> Self {
        Self {
            w1: 0.35,
            w2: 0.35,
            w3: 0.20,
            w4: 0.10,
        }
    }
}

pub fn compute_technician_score(
    technician: &Technician,
    distance_km: f64,
    weights: &MatchWeights,
) -> f32 {
    let cert = technician.certification_level;
    let auth = technician.part_authenticity_score;
    let speed = technician.speed_score;
    let dist = distance_km as f32;

    (weights.w1 * cert) + (weights.w2 * auth) + (weights.w3 * speed) - (weights.w4 * dist)
}

#[derive(serde::Serialize)]
pub struct RankedTechnician {
    pub technician: Technician,
    pub distance_km: f64,
    pub match_score: f32,
}

pub fn rank_technicians(
    technicians_with_distances: Vec<(Technician, f64)>,
    weights: Option<MatchWeights>,
) -> Vec<RankedTechnician> {
    let weights = weights.unwrap_or_default();
    let mut ranked: Vec<RankedTechnician> = technicians_with_distances
        .into_iter()
        .map(|(tech, distance_km)| {
            let match_score = compute_technician_score(&tech, distance_km, &weights);
            RankedTechnician {
                technician: tech,
                distance_km,
                match_score,
            }
        })
        .collect();

    // Sort descending by match score
    ranked.sort_by(|a, b| b.match_score.partial_cmp(&a.match_score).unwrap_or(std::cmp::Ordering::Equal));
    ranked
}