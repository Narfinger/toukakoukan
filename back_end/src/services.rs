use std::path::PathBuf;

use crate::{
    api::api_endpoints,
    types::{AppState, Args},
};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::routes;

const FRONT_PUBLIC: &str = "../front_end/dist";

/// Frontend routes
pub(crate) fn front_public_route(cli: Args) -> Router {
    let dir = cli.js.unwrap_or(PathBuf::from(FRONT_PUBLIC));
    Router::new()
        .fallback_service(ServeDir::new(dir))
        .layer(TraceLayer::new_for_http())
}

/// backend routes
pub(crate) fn back_public_route(state: AppState) -> Router {
    Router::new()
        .route("/auth/session", get(routes::session))
        .route("/auth/login", post(routes::login)) // sets username in session
        .route("/auth/logout", get(routes::logout)) // deletes username in session
        .route("/auth/createuser", post(routes::create_user))
        .with_state(state.clone())
        .nest("/api", api_endpoints(state))
}
