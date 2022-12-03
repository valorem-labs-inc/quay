use std::time::Duration;

use prometheus::{HistogramVec, IntCounterVec};
use prometheus_metric_storage::{MetricStorage, StorageRegistry};

#[derive(MetricStorage, Clone, Debug)]
#[metric(subsystem = "api")]
pub struct ApiMetrics {
    /// Number of completed API requests.
    #[metric(labels("path", "method", "status_code"))]
    requests_complete: IntCounterVec,

    /// Execution time for each API request.
    #[metric(labels("path", "method"))]
    requests_duration_miliseconds: HistogramVec,
}

impl ApiMetrics {
    pub fn inst(registry: &StorageRegistry) -> Result<&ApiMetrics, prometheus::Error> {
        ApiMetrics::instance(registry)
    }

    pub fn on_request_completed(
        &self,
        path: &str,
        method: &str,
        status: u16,
        request_time: Duration,
    ) {
        let rt = (request_time.as_nanos() as f64) / 1_000_000.0;

        self.requests_complete
            .with_label_values(&[path, method, &status.to_string()])
            .inc();
        self.requests_duration_miliseconds
            .with_label_values(&[path, method])
            .observe(rt);
    }
}
