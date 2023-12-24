use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::{self, Path, State},
    http::StatusCode,
    middleware,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Row};
use tower_sessions::Session;
use tracing::info;

use crate::{
    types::{
        group_query_result_to_group_query_return, AppState, Expense, Group, GroupQueryResult,
        GroupQueryReturn, GROUP_QUERY_STRING,
    },
    usersecure::user_secure,
};

async fn groups(
    session: Session,
    State(state): State<AppState>,
) -> Result<Json<Vec<GroupQueryReturn>>, StatusCode> {
    let user_id = session.get_value("user_id");

    let groups: Vec<GroupQueryResult> = sqlx::query_as::<_, GroupQueryResult>(GROUP_QUERY_STRING)
        .bind(user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(group_query_result_to_group_query_return(groups)))
}

async fn get_expenses(
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> Result<Json<Vec<Expense>>, StatusCode> {
    info!("Doing {}", expense_group_id);
    let rows =
        sqlx::query_as::<_, Expense>("SELECT * FROM expense WHERE expense_group_id = ? LIMIT ?")
            .bind(expense_group_id)
            .bind(25)
            .fetch_all(&state.pool)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(rows))
}

async fn post_expense(
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

#[derive(Debug, Serialize, Deserialize)]
struct GroupResponse {
    name: String,
    people: Vec<String>,
}

async fn get_group(
    State(state): State<AppState>,
    Path(group_id): Path<u32>,
) -> Result<Json<GroupResponse>, StatusCode> {
    let people = sqlx::query("SELECT name FROM expense_group_people WHERE expense_group_id = ?")
        .bind(group_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let name_future = sqlx::query("SELECT name FROM expense_group WHERE id=?")
        .bind(group_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let name = name_future.get(0);
    Ok(Json(GroupResponse {
        people: people.iter().map(|r| r.get(0)).collect(),
        name,
    }))
}

pub(crate) fn api_endpoints(state: AppState) -> Router<()> {
    Router::new()
        .route("/groups/", get(groups))
        .route("/expense/:id/", get(get_expenses))
        .route("/expense/:id/", post(post_expense))
        .route("/group/:id/", get(get_group))
        .with_state(state)
        .route_layer(middleware::from_fn(user_secure))
}
