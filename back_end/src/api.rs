use axum::{
    debug_handler,
    extract::{self, Path, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Extension, Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::Row;
use tower_sessions::Session;
use tracing::info;

const EXPENSE_REQUEST_LIMIT: i64 = 25;

use crate::{
    types::{AppState, Expense, Group},
    users::User,
};

/// returns all groups fro the user_id in the session
async fn groups(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Group>>, StatusCode> {
    let groups = user
        .groups(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(groups))
}

/// returns all expenses for the user_id in session and the expense_group_id in path
async fn get_expenses(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> Result<Json<Vec<Expense>>, StatusCode> {
    info!("Doing {}", expense_group_id);
    if !user.in_group(&state.pool, expense_group_id).await {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let rows = sqlx::query_as::<_, Expense>(
            "SELECT * FROM expense WHERE expense_group_id = ? LIMIT ?",
        )
        .bind(expense_group_id)
        .bind(EXPENSE_REQUEST_LIMIT)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
        Ok(Json(rows))
    }
}

/// inserts a expense into the database with the expense_group_id in the path
async fn post_expense(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
    Json(payload): extract::Json<Expense>,
) -> Result<(), StatusCode> {
    if !user.in_group(&state.pool, expense_group_id).await {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        sqlx::query(
            "INSERT INTO expense (payed_type, amount, expense_group_id) VALUES (?, ?, ?);
",
        )
        .bind(payload.payed_type)
        .bind(payload.amount)
        .bind(expense_group_id as i64)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// A Group given as a response. This does only show the names of people and not their ids, etc.
struct GroupResponse {
    name: String,
    people: Vec<String>,
}

/// gets a specific group for the group_id given in the path
async fn get_group(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> Result<Json<Group>, StatusCode> {
    if !user.in_group(&state.pool, expense_group_id).await {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let g = user
            .get_specific_group(&state.pool, expense_group_id as i64)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
        Ok(Json(g))
    }
}

/// collecting all api endpoints
pub(crate) fn api_endpoints(state: AppState) -> Router<()> {
    Router::new()
        .route("/groups/", get(groups))
        .route("/expense/:id/", get(get_expenses))
        .route("/expense/:id/", post(post_expense))
        .route("/group/:id/", get(get_group))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state)
}

/// Authenticate and give the user to all the routes
async fn auth(
    session: Session,
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let user = User::get_user_from_session(&state.pool, &session).await;
    if let Ok(user) = user {
        request.extensions_mut().insert(user);
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
