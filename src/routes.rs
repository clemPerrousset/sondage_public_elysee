use axum::{
    extract::{State, Json},
    http::StatusCode,
    routing::{get, post, delete},
    Router, middleware,
};
use sqlx::SqlitePool;
use crate::models::{VoteRequest, PercentageResult, DeleteCandidateRequest};
use crate::security;

pub fn create_router(pool: SqlitePool) -> Router {
    let api_routes = Router::new()
        .route("/vote", post(vote));

    let admin_routes = Router::new()
        .route("/candidate", delete(delete_candidate))
        .layer(middleware::from_fn(security::verify_admin));

    let public_routes = Router::new()
        .route("/percentage", get(get_percentage));

    Router::new()
        .merge(api_routes)
        .merge(admin_routes)
        .merge(public_routes)
        .with_state(pool)
}

async fn vote(
    State(pool): State<SqlitePool>,
    Json(payload): Json<VoteRequest>,
) -> Result<StatusCode, StatusCode> {
    // 0. Verify Integrity
    let is_valid = match payload.os.as_str() {
        "android" => security::verify_google_play_integrity(&payload.token).await,
        "ios" => security::verify_apple_check(&payload.token).await,
        _ => Err("Unknown OS".to_string()),
    };

    match is_valid {
        Ok(true) => {},
        Ok(false) => return Err(StatusCode::UNAUTHORIZED),
        Err(e) => {
            tracing::warn!("Integrity check failed: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    // 1. Ensure candidate exists or create it
    let candidate_id = sqlx::query_scalar!(
        "INSERT INTO candidates (name) VALUES (?) 
         ON CONFLICT(name) DO UPDATE SET name=name 
         RETURNING id",
        payload.candidate_name
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 2. Upsert vote
    sqlx::query!(
        "INSERT INTO votes (device_id, candidate_id, timestamp) 
         VALUES (?, ?, CURRENT_TIMESTAMP)
         ON CONFLICT(device_id) DO UPDATE SET 
            candidate_id = excluded.candidate_id,
            timestamp = CURRENT_TIMESTAMP",
        payload.device_id,
        candidate_id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

async fn get_percentage(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<PercentageResult>>, StatusCode> {
    let total_votes: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM votes")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if total_votes == 0 {
        return Ok(Json(vec![]));
    }

    let results = sqlx::query!(
        "SELECT c.name, COUNT(v.device_id) as count 
         FROM candidates c 
         JOIN votes v ON c.id = v.candidate_id 
         GROUP BY c.id"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let percentages: Vec<PercentageResult> = results
        .into_iter()
        .map(|r| PercentageResult {
            name: r.name,
            percentage: (r.count as f64 / total_votes as f64) * 100.0,
        })
        .collect();

    Ok(Json(percentages))
}

async fn delete_candidate(
    State(pool): State<SqlitePool>,
    Json(payload): Json<DeleteCandidateRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let candidate_id = sqlx::query_scalar!(
        "SELECT id FROM candidates WHERE name = ?",
        payload.name
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(id) = candidate_id {
        // Delete votes first (if not cascading, but good to be explicit or rely on FK cascade if set)
        // Our schema defined FK but not ON DELETE CASCADE, so we must delete manually or update schema.
        // Let's delete manually for safety.
        sqlx::query!("DELETE FROM votes WHERE candidate_id = ?", id)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        sqlx::query!("DELETE FROM candidates WHERE id = ?", id)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
