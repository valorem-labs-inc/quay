use axum::response::IntoResponse;
use http::StatusCode;

/// Health check
///
/// Check if API is online
#[utoipa::path(
    get,
    path = "/health_check",
    responses(
        (status = 200, description = "API is online")
    )
)]
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
