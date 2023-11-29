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

mod types;

type AppEngine = Engine<Handlebars<'static>>;
#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
    pool: Pool<Sqlite>,
}

async fn index(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("index", engine, ())
}

async fn get_expenses(
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> impl IntoResponse {
    let rows =
        sqlx::query_as::<_, types::Expense>("SELECT * FROM expense WHERE expense_group_id = ?")
            .bind(expense_group_id)
            .fetch_all(&state.pool)
            .await
            .expect("Error in getting expenses");
    Json(rows)
}

async fn post_expense(
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
    Json(payload): extract::Json<types::Expense>,
) -> impl IntoResponse {
    let insert_res =
        sqlx::query("INSERT INTO expense (payed_type, amount, expense_group_id) VALUES (?, ?, ?);")
            .bind(payload.payed_type)
            .bind(payload.amount as i64)
            .bind(expense_group_id as i64)
            .execute(&state.pool)
            .await;

    match insert_res {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            warn!("error {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn add_expense_group(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<types::ExpenseGroup>,
) -> impl IntoResponse {
    let row: SqliteRow = sqlx::query("INSERT INTO expense_group (name) VALUES (?) RETURNING id;")
        .bind(payload.name)
        .fetch_one(&state.pool)
        .await
        .expect("Error in inserting expense group");
    let key: i64 = row.get(0);
    for i in payload.people {
        sqlx::query("INSERT INTO expense_group_people (expense_group_id, name) VALUES (?,?)")
            .bind(key)
            .bind(i)
            .execute(&state.pool)
            .await
            .expect("Could not insert name");
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
