use simple_metrics::{Labels, MetricDef, MetricStore, MetricType, ToMetricDef};

use crate::model::LookupEntry;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Metric {
    LookedUp,
}

impl ToMetricDef for Metric {
    fn to_metric_def(&self) -> MetricDef {
        match self {
            Metric::LookedUp => {
                MetricDef::new("looked_up", "looked up IP address", MetricType::Gauge).unwrap()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricsData {
    pub looked_up: Vec<LookupEntry>,
}

impl Default for MetricsData {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsData {
    pub fn new() -> Self {
        MetricsData {
            looked_up: Vec::new(),
        }
    }

    pub fn into_metric_store(&self) -> simple_metrics::MetricStore<Metric> {
        let mut store: MetricStore<Metric> = MetricStore::new();

        for item in &self.looked_up {
            let labels = Labels::new()
                .with("ip", &item.ip_str)
                .with("latitude", item.loc.latitude.to_string())
                .with("longitude", item.loc.longitude.to_string());

            store
                .add_value(Metric::LookedUp, &labels, true)
                .expect("invalid value looked up IP");
        }

        store
    }
}
