use prometheus::HistogramVec;
use prometheus_metric_storage::{MetricStorage, StorageRegistry};

use super::get_metric_storage_registry;

#[derive(MetricStorage, Clone, Debug)]
#[metric(subsystem = "database")]
pub struct DatabaseMetrics {
    /// Execution times of DB queries.
    #[metric(labels("type"))]
    database_queries: HistogramVec,
}

impl DatabaseMetrics {
    pub fn inst(registry: &StorageRegistry) -> Result<&Self, prometheus::Error> {
        DatabaseMetrics::instance(registry)
    }

    pub fn get() -> &'static Self {
        DatabaseMetrics::instance(get_metric_storage_registry()).unwrap()
    }
}
