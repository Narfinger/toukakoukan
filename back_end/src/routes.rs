use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use serde::Deserialize;
use tower_sessions::Session;

/// imitating an API response
#[allow(clippy::unused_async)]
pub async fn api_handler() -> impl IntoResponse {
    tracing::info!("Seeking api data");
    Json(
        json!({"result": "ok", "message": "You've reached the backend API by using a valid token."}),
    )
}

/// route to handle log in
#[allow(clippy::unused_async)]
#[allow(clippy::missing_panics_doc)]
pub async fn login(session: Session, Json(login): Json<Login>) -> impl IntoResponse {
    tracing::info!("Logging in user: {}", login.username);

    if check_password(&login.username, &login.password) {
        session.insert("user_id", login.username).unwrap();
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

// assume all passwords work
const fn check_password(_username: &str, _password: &str) -> bool {
    true
}

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

pub(crate) async fn get_expenses(
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> Result<Json<Vec<Expense>>, StatusCode> {
    let rows =
        sqlx::query_as::<_, Expense>("SELECT * FROM expense WHERE expense_group_id = ? LIMIT >")
            .bind(expense_group_id)
            .bind(25)
            .fetch_all(&state.pool)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(rows))
}

pub async fn post_expense(
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
    Json(payload): extract::Json<Expense>,
) -> Result<(), StatusCode> {
    sqlx::query(
        "INSERT INTO expense (payed_type, amount, expense_group_id) VALUES (?, ?, ?);
",
    )
    .bind(payload.payed_type)
    .bind(payload.amount as i64)
    .bind(expense_group_id as i64)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

use axum::{body::Body, http::Request};

use crate::types::{AppState, Expense};

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
