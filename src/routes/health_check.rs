use axum::response::IntoResponse;
use http::StatusCode;

#[utoipa::path(
    get,
    path = "health_check",
    responses(
        (status = 200, description = "Check if API is online")
    )
)]
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
