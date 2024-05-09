use std::path::PathBuf;

use crate::{api::api_endpoints, types::AppState};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::Session;

use crate::routes;

/// Frontend routes
pub(crate) fn front_public_route(dir: PathBuf) -> Router {
    Router::new()
        .fallback_service(ServeDir::new(dir))
        .layer(TraceLayer::new_for_http())
}

async fn user_creation_enabled(
    _: Session,
    State(state): State<AppState>,
) -> Result<Json<bool>, StatusCode> {
    Ok(Json(state.args.user_creation))
}
/// backend routes
pub(crate) fn back_public_route(state: AppState) -> Router {
    Router::new()
        .route("/auth/session", get(routes::session))
        .route("/auth/login", post(routes::login)) // sets username in session
        .route("/auth/logout", get(routes::logout)) // deletes username in session
        .route("/auth/createuser", post(routes::create_user))
        .route("/user_creation/", get(user_creation_enabled))
        .with_state(state.clone())
        .nest("/api", api_endpoints(state))
}
