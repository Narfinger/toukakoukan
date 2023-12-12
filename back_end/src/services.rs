use crate::{api::api_endpoints, types::AppState, usersecure::user_secure};
use axum::{
    error_handling::HandleErrorLayer,
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    middleware,
    routing::{get, post},
    BoxError, Router,
};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::{SessionManagerLayer, SessionStore};

use crate::{routes, FRONT_PUBLIC};

// *********
// FRONT END
// *********
// Front end to server svelte build bundle, css and index.html from public folder
pub fn front_public_route() -> Router {
    Router::new()
        .fallback_service(
            ServeDir::new(FRONT_PUBLIC).not_found_service(handle_error.into_service()),
        )
        .layer(TraceLayer::new_for_http())
}

#[allow(clippy::unused_async)]
async fn handle_error() -> (StatusCode, &'static str) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
}

// ********
// BACK END
// ********
// Back end server built form various routes that are either public, require auth, or secure login
pub fn backend<Store: SessionStore>(
    session_layer: SessionManagerLayer<Store>,
    state: AppState,
) -> Router {
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(session_layer);

    // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
    // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    Router::new()
        .merge(back_public_route(state))
        .merge(back_auth_route())
        //.merge(back_token_route(state))
        .layer(session_service)
}

// *********
// BACKEND NON-AUTH
// *********
//
pub fn back_public_route(state: AppState) -> Router {
    Router::new()
        .route("/auth/session", get(routes::data_handler)) // gets session data
        .route("/auth/login", post(routes::login)) // sets username in session
        .route("/auth/logout", get(routes::logout)) // deletes username in session
        .route("/test", get(routes::not_implemented_route))
        .with_state(state.clone())
        .nest("/api", api_endpoints(state))
}

// *********
// BACKEND SESSION
// *********
//
pub fn back_auth_route() -> Router {
    Router::new()
        .route("/secure", get(routes::session_handler))
        .route_layer(middleware::from_fn(user_secure))
}

// *********
// BACKEND API
// *********
//
// invoked with State that stores API that is checked by the `middleware::auth`
/*pub fn back_token_route<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/api", get(routes::api_handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth::<_>))
        .with_state(state)
}
*/
