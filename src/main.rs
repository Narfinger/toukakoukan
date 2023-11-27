use std::io;

use axum::{
    extract::{FromRef, Path},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_template::{engine::Engine, Key, RenderHtml};
use handlebars::Handlebars;
use sqlx::{Connection, Pool, Sqlite, SqliteConnection};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type AppEngine = Engine<Handlebars<'static>>;
#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
    pool: Pool<Sqlite>,
}

async fn index(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("index", engine, ())
}

#[derive(Debug)]
struct Expense {
    people: Vec<String>,
    payed: usize,
    amount: u64,
}

#[derive(Debug)]
struct ExpenseGroup {
    expenses: Vec<Expense>,
    name: String,
}

#[tokio::main]
async fn main() {
    let pool = Pool::<Sqlite>::connect("sqlite::memory:")
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
