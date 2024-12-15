use std::{collections::BTreeMap, net::IpAddr, str::FromStr, sync::Arc};

use axum::{
    http::StatusCode,
    routing::get,
    Router,
    {response::IntoResponse, Extension},
};
use simple_metrics::RenderIntoMetrics;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

use crate::{metrics::MetricsData, model::LookupEntry};

#[derive(Debug, Clone)]
pub enum Error {
    ApplicationError(String),
    MaxMindError(Arc<maxminddb::MaxMindDBError>),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{self:?}")).into_response()
    }
}

impl From<maxminddb::MaxMindDBError> for Error {
    fn from(e: maxminddb::MaxMindDBError) -> Self {
        Self::MaxMindError(Arc::new(e))
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::ApplicationError(e.to_string())
    }
}

pub struct AppContext {
    pub namespace: Option<String>,
    pub db_reader: maxminddb::Reader<std::vec::Vec<u8>>,
    pub looked_up: BTreeMap<String, LookupEntry>,
}

impl AppContext {
    pub fn new(namespace: Option<&str>, db_reader: maxminddb::Reader<std::vec::Vec<u8>>) -> Self {
        Self {
            namespace: namespace.map(|r| r.to_string()),
            db_reader,
            looked_up: BTreeMap::new(),
        }
    }

    pub fn lookup_ip(&mut self, ip: &str) -> Result<LookupEntry, Error> {
        let parsed: IpAddr = ip
            .parse()
            .map_err(|e: <IpAddr as FromStr>::Err| Error::ApplicationError(e.to_string()))?;

        let city: maxminddb::geoip2::City = self.db_reader.lookup(parsed)?;

        let maybe_entry: Result<LookupEntry, &str> =
            LookupEntry::from_city(ip, city).ok_or("can't parse");

        let entry = maybe_entry?;
        self.looked_up.insert(ip.to_owned(), entry.clone());

        Ok(entry)
    }
}

#[axum::debug_handler]
pub async fn health(
    Extension(_app_context): Extension<Arc<Mutex<AppContext>>>,
) -> Result<axum::response::Response, Error> {
    Ok((StatusCode::OK, "OK").into_response())
}

#[axum::debug_handler]
pub async fn metrics(
    Extension(app_context): Extension<Arc<Mutex<AppContext>>>,
) -> Result<axum::response::Response, Error> {
    let ac = app_context.lock().await;
    let looked_up = &ac.looked_up;
    let namespace = &ac.namespace.clone();

    let mut metrics_data = MetricsData::new();

    for (_, item) in looked_up.iter() {
        metrics_data.looked_up.push(item.clone());
    }

    let store = metrics_data.into_metric_store();
    let data = store.render_into_metrics(namespace.as_deref());

    Ok((StatusCode::OK, data).into_response())
}

pub fn build_app(state: Arc<Mutex<AppContext>>) -> Router {
    let service = tower::ServiceBuilder::new()
        .layer(Extension(state))
        .layer(TraceLayer::new_for_http());

    Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics))
        .nest("/api/v1", crate::api::new_router())
        .layer(service)
}

pub async fn serve_tcp(app: Router, listener: tokio::net::TcpListener) {
    axum::serve(listener, app)
        .await
        .expect("Error running HTTP server.")
}
