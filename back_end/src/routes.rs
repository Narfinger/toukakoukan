use crate::types::{AppState, Expense};
use axum::debug_handler;
use axum::{body::Body, http::Request};
use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use password_auth::verify_password;
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, query_as, Pool, Sqlite};
use tower_sessions::Session;
use tracing::info;

/// imitating an API response
#[allow(clippy::unused_async)]
pub async fn api_handler() -> impl IntoResponse {
    tracing::info!("Seeking api data");
    Json(
        json!({"result": "ok", "message": "You've reached the backend API by using a valid token."}),
    )
}

/// route to handle log in
#[debug_handler]
pub(crate) async fn login(
    session: Session,
    State(state): State<AppState>,
    Json(login): Json<Login>,
) -> impl IntoResponse {
    tracing::info!("Logging in user: {}", login.username);

    if check_password(&state.pool, &login.username, &login.password).await {
        session.insert("user_id", login.username).unwrap();
        todo!("add a random session token here and check it (probably)");
        Json(json!({"result": "ok"}))
    } else {
        Json(json!({"result": "error"}))
    }
}

/// route to handle log out
#[allow(clippy::unused_async)]
pub async fn logout(session: Session) -> impl IntoResponse {
    let user = session.get_value("user_id").unwrap_or_default();
    tracing::info!("Logging out user: {}", user);
    // drop session
    session.flush();
    Json(json!({"result": "ok"}))
}

/// password checking with database
async fn check_password(pool: &Pool<Sqlite>, username: &str, password: &str) -> bool {
    return true;
    /// working login thing
    let pw_hash: Result<(String,), _> = query_as("select password_hash from users where name = ?")
        .bind(username)
        .fetch_one(pool)
        .await;
    if let Ok((pw_hash,)) = pw_hash {
        verify_password(password, &pw_hash).is_ok()
    } else {
        info!("Somethign is wrong with the db connection");
        false
    }
}

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[allow(clippy::unused_async)]
pub async fn not_implemented_route(req: Request<Body>) -> impl IntoResponse {
    // add which route is requesting this?
    format!("Route is planned but not yet implemented for {}", req.uri())
}
/// output entire session object
#[allow(clippy::unused_async)]
pub async fn session_handler(session: Session) -> impl IntoResponse {
    tracing::info!("Seeking session info");
    Json(json!({ "session": format!("{:?}", session) }))
}

/// output session data in json
#[allow(clippy::unused_async)]
pub async fn data_handler(session: Session) -> impl IntoResponse {
    tracing::info!("Seeking session data");
    let user_id = session.get_value("user_id").unwrap_or_else(|| "".into());
    Json(json!({ "user_id": user_id }))
}
