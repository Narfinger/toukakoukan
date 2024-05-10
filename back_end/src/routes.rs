use std::path::PathBuf;

use crate::api::api_endpoints;
use crate::types::AppState;
use crate::users::User;
use anyhow::{anyhow, Context, Result};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{Pool, Sqlite};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_sessions::Session;

/// route to handle log in
async fn login(
    session: Session,
    State(state): State<AppState>,
    Json(login): Json<Login>,
) -> Result<Json<Value>, StatusCode> {
    tracing::info!("Logging in user: {}", login.username);
    if state.args.release {
        let user = check_password(&state.pool, &login.username, &login.password)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
        session
            .insert("user_id", user.id)
            .await
            .context("Error inserting into session")
            .map_err(|_| StatusCode::NOT_FOUND)?;
    } else {
        tracing::info!("We are only looking at user_id 1 and hardcoding it");
        session
            .insert("user_id", 1)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
    }
    Ok(Json(json!({"result": "ok"})))
}

/// route to handle log out
async fn logout(session: Session) -> impl IntoResponse {
    let user = session.get_value("user_id").await.unwrap_or_default();
    tracing::info!("Logging out user: {}", user.unwrap());
    // drop session
    session.flush().await.expect("Error in flushing session");
    Json(json!({"result": "ok"}))
}

/// Route to create a user
async fn create_user(
    _: Session,
    State(state): State<AppState>,
    Json(login): Json<Login>,
) -> Result<Json<Value>, StatusCode> {
    if state.args.user_creation {
        let pass = password_auth::generate_hash(login.password);

        sqlx::query("INSERT INTO user (name, password_hash) VALUES (?,?)")
            .bind(login.username)
            .bind(pass)
            .execute(&state.pool)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(Json(json!({"result": "ok"})))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// password checking with database
async fn check_password(
    pool: &Pool<Sqlite>,
    username: &str,
    password: &str,
) -> anyhow::Result<User> {
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
/// The login datastructure
struct Login {
    username: String,
    password: String,
}

async fn session(session: Session) -> Result<Json<Value>, StatusCode> {
    let user_id_val = session
        .get_value("user_id")
        .await
        .ok()
        .flatten()
        .context("Cannot find user_id in session")
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let user_id: i64 = serde_json::from_value(user_id_val)
        .context("Cannot make into json")
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(json!({ "user_id": user_id })))
}

/// Frontend routes
pub(crate) fn front_public_route(dir: PathBuf) -> Router {
    Router::new()
        .fallback_service(ServeDir::new(dir))
        .layer(TraceLayer::new_for_http())
}

/// is the user creation enabled
async fn user_creation_enabled(
    _: Session,
    State(state): State<AppState>,
) -> Result<Json<bool>, StatusCode> {
    Ok(Json(state.args.user_creation))
}
/// backend routes
pub(crate) fn back_public_route(state: AppState) -> Router {
    Router::new()
        .route("/auth/session", get(session))
        .route("/auth/login", post(login)) // sets username in session
        .route("/auth/logout", get(logout)) // deletes username in session
        .route("/auth/createuser", post(create_user))
        .route("/user_creation/", get(user_creation_enabled))
        .with_state(state.clone())
        .nest("/api", api_endpoints(state))
}
