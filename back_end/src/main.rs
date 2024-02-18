use ansi_term::Colour::{Green, Red};
use anyhow::Context;
use axum::{http::Method, Router};
use clap::Parser;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Pool, Sqlite};
use std::{net::SocketAddr, str::FromStr};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::SqliteStore;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use types::Args;

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
async fn app(args: Args) -> anyhow::Result<Router> {
    // create store for backend.  Stores an api_token.
    let state = {
        let pool = Pool::connect_with(
            SqliteConnectOptions::from_str("test.db")
                .context("Could not parse db location")?
                .log_statements(log::LevelFilter::Error),
        )
        .await
        .context("Error in DB")?;

        /*
        let pool = Pool::<Sqlite>::connect("test.db")
        .await
        .context("Error in db")?;
        */

        //sqlx::migrate!().run(&pool).await?;
        AppState { pool: pool, args }
    };

    // setup up sessions and store to keep track of session information
    //let session_store = MemoryStore::default();
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
    let cors_layer = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    let backend = Router::new()
        .merge(services::back_public_route(state))
        //.merge(back_token_route(state))
        .layer(session_service)
        .layer(cors_layer);

    // combine the front and backend into server
    Ok(Router::new()
        .merge(services::front_public_route())
        .merge(backend)
        .layer(TraceLayer::new_for_http()))
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Args::parse();

    if cli.release {
        println!("{}", Green.paint("In release mode!"));
    } else {
        println!(
            "{}",
            Red.paint("WARNING: IN DEBUG MODE, EVERYTHING IS INSECURE!")
        );
    }
    if cli.user_creation {
        println!("{}", Red.paint("User creation is enabled!"));
    }

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

    let app = app(cli).await?;
    println!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .context("Could not bind server")?;
    Ok(())
}
