use anyhow::Context;
use axum::Router;
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::types::AppState;

mod api;
mod authenticator;
mod routes;
mod services;
mod store;
mod types;
mod users;
mod usersecure;

// SETUP Constants
const SESSION_COOKIE_NAME: &str = "axum_svelte_session";
const FRONT_PUBLIC: &str = "../front_end/dist";
const SERVER_PORT: &str = "3000";
const SERVER_HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // start tracing - level set by either RUST_LOG env variable or defaults to debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "svelte_axum_project=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr: SocketAddr = format!("{}:{}", SERVER_HOST, SERVER_PORT)
        .parse()
        .context("Can not parse address and port")?;

    // create store for backend.  Stores an api_token.
    let shared_state = {
        let pool = Pool::<Sqlite>::connect("test.db")
            .await
            .context("Error in db")?;
        AppState { pool: pool }
    };

    // setup up sessions and store to keep track of session information
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_name(SESSION_COOKIE_NAME);

    // combine the front and backend into server
    let app = Router::new()
        .merge(services::front_public_route())
        .merge(services::backend(session_layer, shared_state));

    tracing::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
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
