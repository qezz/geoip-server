use std::sync::Arc;

use axum::{
    http::StatusCode,
    routing::get,
    Router,
    {response::IntoResponse, Extension},
};
use tower_http::trace::TraceLayer;

#[derive(Debug, Clone)]
pub enum Error {
    ApplicationError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{self:?}")).into_response()
    }
}

pub struct AppContext {
    pub namespace: Option<String>,
    pub db_reader: maxminddb::Reader<std::vec::Vec<u8>>,
}

impl AppContext {
    pub fn new(namespace: Option<&str>, db_reader: maxminddb::Reader<std::vec::Vec<u8>>) -> Self {
        Self {
            namespace: namespace.map(|r| r.to_string()),
            db_reader,
        }
    }
}

#[axum::debug_handler]
pub async fn health(
    Extension(_app_context): Extension<Arc<AppContext>>,
) -> Result<axum::response::Response, Error> {
    Ok((StatusCode::OK, "OK").into_response())
}

pub fn build_app(state: Arc<AppContext>) -> Router {
    let service = tower::ServiceBuilder::new()
        .layer(Extension(state))
        .layer(TraceLayer::new_for_http());

    Router::new()
        .route("/health", get(health))
        .nest("/api/v1", crate::api::new_router())
        .layer(service)
}

pub async fn serve_tcp(app: Router, listener: tokio::net::TcpListener) {
    axum::serve(listener, app)
        .await
        .expect("Error running HTTP server.")
}
