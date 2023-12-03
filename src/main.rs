use axum::{
    error_handling::HandleErrorLayer,
    extract::{self, FromRef, Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    BoxError, Form, Router,
};
use axum_login::tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};
use axum_login::{login_required, AuthManagerLayerBuilder};
use axum_macros::debug_handler;
use axum_template::{engine::Engine, Key, RenderHtml};
use log::{info, warn};
use sqlx::{sqlite::SqliteRow, Connection, Pool, Row, Sqlite, SqliteConnection, SqlitePool};
use std::{io, time::Duration};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use types::{Backend, PayedType, User};

mod types;

const EXPENSE_LIMIT: i64 = 25;
type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Clone, FromRef)]
struct AppState {
    pool: Pool<Sqlite>,
}

async fn index(_: AppState) -> impl IntoResponse {
    String::from("<p>test</p>")
}

/// this calculates how much `user_id` ows the group `expense_group_id`
/// A positive value means that `user_id` is owed x amount of money
/// A negative value means that `user_id` owes x amount of money
async fn get_total_owed(
    State(state): State<AppState>,
    Path((user_id, expense_group_id)): Path<(u32, u32)>,
) -> Result<Json<i64>, StatusCode> {
    let rows = sqlx::query(
        "SELECT payed_type,sum(amount) FROM expense WHERE expense_group_id = ? GROUP BY payed_type",
    )
    .bind(expense_group_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let res = rows
        .iter()
        .map(|i| (i.get::<PayedType, _>(0), i.get::<i64, _>(1)));

    let mut sum = 0;
    for (i, amount) in res {
        match i {
            PayedType::EvenSplit(j) => {
                if j == user_id as usize {
                    sum += amount / 2;
                } else {
                    sum -= amount / 2;
                }
            }
            PayedType::OwedTotal(j) => {
                if j == user_id as usize {
                    sum += amount;
                } else {
                    sum -= amount;
                }
            }
        }
    }

    Ok(Json(sum))
}

async fn get_expenses(
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> Result<Json<Vec<types::Expense>>, StatusCode> {
    let rows = sqlx::query_as::<_, types::Expense>(
        "SELECT * FROM expense WHERE expense_group_id = ? LIMIT >",
    )
    .bind(expense_group_id)
    .bind(EXPENSE_LIMIT)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(rows))
}

async fn post_expense(
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
    Json(payload): extract::Json<types::Expense>,
) -> Result<(), StatusCode> {
    sqlx::query("INSERT INTO expense (payed_type, amount, expense_group_id) VALUES (?, ?, ?);")
        .bind(payload.payed_type)
        .bind(payload.amount as i64)
        .bind(expense_group_id as i64)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

async fn add_expense_group(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<types::ExpenseGroup>,
) -> Result<(), StatusCode> {
    let row: SqliteRow = sqlx::query("INSERT INTO expense_group (name) VALUES (?) RETURNING id;")
        .bind(payload.name)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let key: i64 = row.get(0);
    for i in payload.people {
        sqlx::query("INSERT INTO expense_group_people (expense_group_id, name) VALUES (?,?)")
            .bind(key)
            .bind(i)
            .execute(&state.pool)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
    }
    Ok(())
}

async fn login(
    mut auth_session: AuthSession,
    Form(creds): Form<types::Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/protected").into_response()
}

fn protected_routes() -> Router {
    Router::new()
        .route(
            "/protected",
            get(|| async { "Gotta be logged in to see me!" }),
        )
        .route_layer(login_required!(Backend, login_url = "/login"))
}

async fn get_login() -> impl IntoResponse {
    String::from("PLEASE LOGIN").into_response()
}

async fn post_login(
    mut auth_session: AuthSession,
    Form(creds): Form<types::Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return String::from("LOGIN ERROR").into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/").into_response()
}

async fn protected(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => format!("You are a user {:?}", user).into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[tokio::main]
async fn main() {
    let pool = Pool::<Sqlite>::connect("test.db")
        .await
        .expect("Error in db");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "axum_login=debug,sqlx=warn,tower_http=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = pool;
    let auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayerBuilder::new(backend, session_layer).build());

    let auth_routes = Router::new()
        .route("/login", post(post_login))
        .route("/login", get(get_login));

    let app = protected::router()
        .route_layer(login_required!(Backend, login_url = "/login"))
        .merge(auth_routes)
        .layer(auth_service);

    // build our application with a single route
    let state = AppState { pool };
    let static_files = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/js", ServeDir::new("json"));
    /*
    let app = Router::new()
        //.route("/", get(index))
        .route("/expense/:id/", get(get_expenses))
        .route("/expense/:id/", post(post_expense))
        .route("/total/:id/:id/", get(get_total_owed))
        .route("/add_expense_group/", post(add_expense_group))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .fallback_service(static_files);
    */
    // run it with hyper on localhost:3000
    println!("See example: http://127.0.0.1:3000/example");

    let app = Router::new()
        .route("/protected", get(todo!()))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/login", post(todo!()))
        .route("/login", get(todo!()))
        .layer(auth_service);

    axum::Server::bind(&([127, 0, 0, 1], 3000).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
