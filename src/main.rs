use axum::{
    extract::{self, FromRef, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_template::{engine::Engine, Key, RenderHtml};
use handlebars::Handlebars;
use log::{info, warn};
use sqlx::{Connection, Pool, Sqlite, SqliteConnection};
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

//async fn expenses(state: AppState, expense_group_id: usize) -> Json<types::Expense> {}

async fn add_expense(
    state: AppState,
    extract::Json(payload): extract::Json<types::Expense>,
) -> impl IntoResponse {
    let insert_res =
        sqlx::query("INSERT INTO expense (payed_type, amount, expense_group_id) VALUES (?, ?, ?);")
            .bind(payload.payed_type)
            .bind(payload.amount as i64)
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
    hbs.register_templates_directory(".html.hbs", "./templates/")
        .unwrap();
    hbs.dev_mode();
    println!("templates {:?}", hbs.get_templates());
    // build our application with a single route
    let static_files = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/js", ServeDir::new("json"));
    let app = Router::new()
        .route("/", get(index))
        .with_state(AppState {
            engine: Engine::from(hbs),
            pool: pool,
        })
        .layer(TraceLayer::new_for_http())
        .fallback_service(static_files);
    // run it with hyper on localhost:3000
    println!("See example: http://127.0.0.1:3000/example");

    axum::Server::bind(&([127, 0, 0, 1], 3000).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
