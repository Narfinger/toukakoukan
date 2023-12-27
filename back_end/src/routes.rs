use crate::types::AppState;
use crate::users::User;
use anyhow::{anyhow, Context, Result};
use axum::debug_handler;
use axum::http::StatusCode;
use axum::{body::Body, http::Request};
use axum::{extract::State, response::IntoResponse, Json};
use password_auth::verify_password;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{query_as, Pool, Sqlite};
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
    /*
    let user = check_password(&state.pool, &login.username, &login.password)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    session
        .insert("user_id", user.id)
        .context("Error inserting into session")
        .map_err(|_| StatusCode::NOT_FOUND)?;
    */
    tracing::info!("We are only looking at user_id 1 and hardcoding it");
    session
        .insert("user_id", 1)
        .await
        .expect("Could not insert into session");
    Json(json!({"result": "ok"}))
}

/// route to handle log out
#[allow(clippy::unused_async)]
pub(crate) async fn logout(session: Session) -> impl IntoResponse {
    let user = session.get_value("user_id").await.unwrap_or_default();
    tracing::info!("Logging out user: {}", user.unwrap());
    // drop session
    session.flush().await.expect("Error in flushing session");
    Json(json!({"result": "ok"}))
}

/// password checking with database
async fn check_password(
    pool: &Pool<Sqlite>,
    username: &str,
    password: &str,
) -> anyhow::Result<User> {
    //return true;

    let user = User::get_user_from_username(pool, username)
        .await
        .context("Could not find user")?;
    if user.check_password(password) {
        Ok(user)
    } else {
        Err(anyhow!("password missmatch"))
    }
}

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[allow(clippy::unused_async)]
pub(crate) async fn not_implemented_route(req: Request<Body>) -> impl IntoResponse {
    // add which route is requesting this?
    format!("Route is planned but not yet implemented for {}", req.uri())
}

pub(crate) async fn session(session: Session) -> Result<Json<Value>, StatusCode> {
    tracing::info!("Seeking session data");
    let user_id = session.get_value("user_id").await.unwrap_or(None);
    Ok(Json(json!({ "user_id": user_id })))
}
