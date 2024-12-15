use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use tokio::sync::Mutex;

use crate::server::{AppContext, Error};

#[axum::debug_handler]
pub async fn lookup(
    Extension(app_context): Extension<Arc<Mutex<AppContext>>>,
    Path(ip_str): Path<String>,
) -> Result<axum::response::Response, Error> {
    let mut ac = app_context.lock().await;
    let entry = ac.lookup_ip(&ip_str).unwrap();

    tracing::info!("Loc: {:?}", entry.loc);

    Ok((StatusCode::OK, Json(entry)).into_response())
}

pub fn new_router() -> Router {
    Router::new().route("/lookup/:ip", get(lookup))
}
