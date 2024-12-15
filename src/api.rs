use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};

use crate::{
    model::Location,
    server::{AppContext, Error},
};

#[axum::debug_handler]
pub async fn lookup(
    Extension(app_context): Extension<Arc<AppContext>>,
    Path(ip_str): Path<String>,
) -> Result<axum::response::Response, Error> {
    let city = app_context.lookup_ip(&ip_str).unwrap();

    tracing::info!("City: {:?}", city.location);

    let loc = Location::from_city_loc(city.location.unwrap()).unwrap();

    Ok((StatusCode::OK, Json(loc)).into_response())
}

pub fn new_router() -> Router {
    Router::new().route("/lookup/:ip", get(lookup))
}
