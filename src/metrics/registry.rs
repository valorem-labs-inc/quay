use once_cell::sync::Lazy;
use prometheus::Registry;
use prometheus_metric_storage::StorageRegistry;

static METRICS_REGISTRY: Lazy<StorageRegistry> = Lazy::new(|| {
    let prometheus_registry =
        Registry::new_custom(Some("quay".to_string()), None).unwrap();

    StorageRegistry::new(prometheus_registry)
});

pub fn get_metric_storage_registry() -> &'static StorageRegistry {
    &METRICS_REGISTRY
}

pub fn get_metrics_registry() -> &'static Registry {
    get_metric_storage_registry().registry()
}
