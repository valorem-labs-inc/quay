use axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse};
use std::time::Instant;

use crate::metrics::{get_metric_storage_registry, ApiMetrics};

pub async fn track_prometheus_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let metrics = ApiMetrics::inst(get_metric_storage_registry()).unwrap();

    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let latency = start.elapsed();
    let status = response.status().as_u16();

    metrics.on_request_completed(&path, method.as_str(), status, latency);

    response
}
