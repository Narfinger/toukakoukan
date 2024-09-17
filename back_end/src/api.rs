use anyhow::Context;
use axum::{
    extract::{self, Path, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Extension, Json, Router,
};

use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tracing::info;

const EXPENSE_REQUEST_LIMIT: i64 = 25;

use crate::{group::Group, types::CreateGroupJson};
use crate::{
    types::{AppState, Expense},
    users::User,
};

/// returns all groups fro the user_id in the session
async fn groups(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Group>>, AppError> {
    let groups = user.groups(&state.pool).await.context("In groups")?;
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
            "SELECT * FROM expense WHERE expense_group_id = ? ORDER BY time DESC LIMIT ?",
        )
        .bind(expense_group_id)
        .bind(EXPENSE_REQUEST_LIMIT)
        .fetch_all(&state.pool)
        .await;
        Ok(Json(rows.unwrap()))
    }
}

/// get total owed
async fn get_total(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> Result<Json<i64>, StatusCode> {
    let group = user
        .get_specific_group(&state.pool, expense_group_id as i64)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let total = group
        .get_oweds(&state.pool)
        .await
        .map(|s| s.user_owes(user.id))
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(total))
}

/// inserts a expense (without its id) into the database with the expense_group_id in the path
async fn post_expense(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
    Json(payload): extract::Json<Expense>,
) -> Result<(), StatusCode> {
    if !user
        .in_group(&state.pool, payload.expense_group_id as u32)
        .await
        && !user
            .get_specific_group(&state.pool, payload.expense_group_id)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .has_user_id(payload.payed_type.id() as i64)
    {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        info!("expense added: {:?}", payload);
        sqlx::query!(
            "INSERT INTO expense (payed_type, amount, name, time, expense_group_id) VALUES (?, ?, ?, ?, ?);
", payload.payed_type, payload.amount, payload.name, payload.time, expense_group_id
        )
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    }
}

/// updates an expense
async fn put_expense(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): extract::Json<Expense>,
) -> Result<(), StatusCode> {
    if !user
        .in_group(&state.pool, payload.expense_group_id as u32)
        .await
        && !user
            .get_specific_group(&state.pool, payload.expense_group_id)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .has_user_id(payload.payed_type.id() as i64)
    {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        sqlx::query!(
            "UPDATE expense SET payed_type = ?, amount = ?, name = ? WHERE id=?",
            payload.payed_type,
            payload.amount,
            payload.name,
            payload.id
        )
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    }
}

/// gets one expense from the id
async fn get_expense_details(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(expense_id): Path<u32>,
) -> Result<Json<Expense>, StatusCode> {
    let expense = sqlx::query_as::<_, Expense>("SELECT * FROM expense WHERE id=?")
        .bind(expense_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    if user.has_expense(&state.pool, &expense).await {
        Ok(Json(expense))
    } else {
        Err(StatusCode::UNAUTHORIZED)
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

/// returns all known users
async fn get_users(
    Extension(_): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = User::get_all_users(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}

/// returns yourself
async fn get_user(
    Extension(user): Extension<User>,
    State(_): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    Ok(Json(user))
}

/// this always adds the own user to it
async fn create_group(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(group): extract::Json<CreateGroupJson>,
) -> Result<(), StatusCode> {
    let mut g = group;
    g.users.push(user.id);
    Group::create_group(g, &state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// collecting all api endpoints
pub(crate) fn api_endpoints(state: AppState) -> Router<()> {
    Router::new()
        .route("/groups/", get(groups))
        .route("/expense/:id/", get(get_expenses))
        .route("/expense/:id/", post(post_expense))
        .route("/expense/", put(put_expense))
        .route("/details/:id/", get(get_expense_details))
        .route("/group/:id/", get(get_group))
        .route("/total/:id/", get(get_total))
        .route("/users/", get(get_users))
        .route("/user/", get(get_user))
        .route("/creategroup/", post(create_group))
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
    let user = User::get_user_from_session(&state, &session).await;
    if let Ok(user) = user {
        info!("user authenticated");
        request.extensions_mut().insert(user);
        Ok(next.run(request).await)
    } else {
        info!("Unauthorized");
        Err(StatusCode::UNAUTHORIZED)
    }
}

struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::info!("stacktrace: {}", self.0.backtrace());
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
