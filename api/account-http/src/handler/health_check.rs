use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub version: String,
    pub sha: String,
    pub build_timestamp: String,
}

impl Default for HealthCheckResponse {
    fn default() -> Self {
        Self {
            version: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
            sha: env!("VERGEN_GIT_SHA").to_string(),
            build_timestamp: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
        }
    }
}

#[tracing::instrument]
pub async fn health_check_handler() -> impl IntoResponse {
    let response = HealthCheckResponse::default();
    (StatusCode::OK, Json(response))
}
