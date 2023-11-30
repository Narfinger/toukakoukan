use axum::{
    extract::{self, FromRef, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use axum_template::{engine::Engine, Key, RenderHtml};
use handlebars::Handlebars;
use log::{info, warn};
use sqlx::{sqlite::SqliteRow, Connection, Pool, Row, Sqlite, SqliteConnection};
use std::io;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use types::PayedType;

mod types;

const EXPENSE_LIMIT: i64 = 25;

type AppEngine = Engine<Handlebars<'static>>;
#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
    pool: Pool<Sqlite>,
}

async fn index(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("index", engine, ())
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
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let mut hbs = Handlebars::new();
    // hbs.register_templates_directory(".html.hbs", "./templates/")
    //.unwrap();
    hbs.dev_mode();
    println!("templates {:?}", hbs.get_templates());
    // build our application with a single route
    let state = AppState {
        engine: Engine::from(hbs),
        pool,
    };
    let static_files = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/js", ServeDir::new("json"));
    let app = Router::new()
        .route("/", get(index))
        .route("/expense/:id/", get(get_expenses))
        .route("/expense/:id/", post(post_expense))
        .route("/total/:id/:id/", get(get_total_owed))
        .route("/add_expense_group/", post(add_expense_group))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .fallback_service(static_files);
    // run it with hyper on localhost:3000
    println!("See example: http://127.0.0.1:3000/example");

    axum::Server::bind(&([127, 0, 0, 1], 3000).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
