use axum::response::IntoResponse;
use http::StatusCode;
use prometheus::Encoder;

use crate::metrics::get_metrics_registry;

pub async fn metrics_prometheus() -> impl IntoResponse {
    let prometheus_storage_registry = get_metrics_registry();
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();
    match encoder.encode(&prometheus_storage_registry.gather(), &mut buffer) {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("could not encode metrics: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let metrics = match String::from_utf8(buffer) {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };

    metrics.into_response()
}
