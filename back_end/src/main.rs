use anyhow::Context;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Router};
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer, SqliteStore};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::types::AppState;

mod api;
mod group;
mod routes;
mod services;
mod types;
mod users;

// SETUP Constants
const SESSION_COOKIE_NAME: &str = "splittinger";
const FRONT_PUBLIC: &str = "../front_end/dist";
const SERVER_PORT: &str = "3000";
const SERVER_HOST: &str = "127.0.0.1";

/// setup the whole app
async fn app() -> anyhow::Result<Router> {
    // create store for backend.  Stores an api_token.
    let state = {
        let pool = Pool::<Sqlite>::connect("test.db")
            .await
            .context("Error in db")?;
        //sqlx::migrate!().run(&pool).await?;
        AppState { pool: pool }
    };

    // setup up sessions and store to keep track of session information
    let session_store = MemoryStore::default();
    let session_store = SqliteStore::new(state.pool.clone())
        .with_table_name("sessions")
        .expect("error in store");
    session_store
        .migrate()
        .await
        .expect("Could not do session store");

    let session_service = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_name(SESSION_COOKIE_NAME);
    let backend = Router::new()
        .merge(services::back_public_route(state))
        //.merge(back_token_route(state))
        .layer(session_service);

    // combine the front and backend into server
    Ok(Router::new()
        .merge(services::front_public_route())
        .merge(backend)
        .layer(TraceLayer::new_for_http()))
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // start tracing - level set by either RUST_LOG env variable or defaults to debug
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
    let addr: SocketAddr = format!("{}:{}", SERVER_HOST, SERVER_PORT)
        .parse()
        .context("Can not parse address and port")?;

    let app = app().await?;
    tracing::info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .context("Could not bind server")?;
    Ok(())
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}
