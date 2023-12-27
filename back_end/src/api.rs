use axum::{
    debug_handler,
    extract::{self, Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::Row;
use tower_sessions::Session;
use tracing::info;

use crate::{
    types::{AppState, Expense, Group},
    users::User,
};

/// returns all groups fro the user_id in the session
async fn groups(
    session: Session,
    State(state): State<AppState>,
) -> Result<Json<Vec<Group>>, StatusCode> {
    let user = User::get_user_from_session(&state.pool, &session)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let groups = user
        .groups(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(groups))
}

/// returns all expenses for the user_id in session and the expense_group_id in path
async fn get_expenses(
    session: Session,
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> Result<Json<Vec<Expense>>, StatusCode> {
    info!("Doing {}", expense_group_id);
    let user = User::get_user_from_session(&state.pool, &session)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if !user.in_group(&state.pool, expense_group_id).await {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let rows = sqlx::query_as::<_, Expense>(
            "SELECT * FROM expense WHERE expense_group_id = ? LIMIT ?",
        )
        .bind(expense_group_id)
        .bind(25)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
        Ok(Json(rows))
    }
}

/// inserts a expense into the database with the expense_group_id in the path
async fn post_expense(
    session: Session,
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
    Json(payload): extract::Json<Expense>,
) -> Result<(), StatusCode> {
    let user = User::get_user_from_session(&state.pool, &session)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if !user.in_group(&state.pool, expense_group_id).await {
        Err(StatusCode::UNAUTHORIZED)
    } else {
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
}

#[derive(Debug, Serialize, Deserialize)]
/// A Group given as a response. This does only show the names of people and not their ids, etc.
struct GroupResponse {
    name: String,
    people: Vec<String>,
}

/// gets a group for the group_id given in the path
async fn get_group(
    session: Session,
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> Result<Json<GroupResponse>, StatusCode> {
    let user = User::get_user_from_session(&state.pool, &session)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if !user.in_group(&state.pool, expense_group_id).await {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let people =
            sqlx::query("SELECT name FROM expense_group_people WHERE expense_group_id = ?")
                .bind(expense_group_id)
                .fetch_all(&state.pool)
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?;
        let name_future = sqlx::query("SELECT name FROM expense_group WHERE id=?")
            .bind(expense_group_id)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
        let name = name_future.get(0);
        Ok(Json(GroupResponse {
            people: people.iter().map(|r| r.get(0)).collect(),
            name,
        }))
    }
}

/// collecting all api endpoints
pub(crate) fn api_endpoints(state: AppState) -> Router<()> {
    Router::new()
        .route("/groups/", get(groups))
        .route("/expense/:id/", get(get_expenses))
        .route("/expense/:id/", post(post_expense))
        .route("/group/:id/", get(get_group))
        .with_state(state)
}
