use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Candidate {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Vote {
    pub device_id: String,
    pub candidate_id: i64,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub device_id: String,
    pub candidate_name: String,
    pub os: String, // "android" or "ios"
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct PercentageResult {
    pub name: String,
    pub percentage: f64,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCandidateRequest {
    pub name: String,
}
