use ansi_term::Colour::{Green, Red};
use anyhow::{anyhow, Context};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;

use sqlx::{migrate::Migrator, sqlite::SqliteConnectOptions, ConnectOptions, Pool};
use std::{net::SocketAddr, path::PathBuf, str::FromStr};
use time::Duration;
use tower_http::trace::TraceLayer;
use tower_sessions::{CachingSessionStore, Expiry, SessionManagerLayer};
use tower_sessions_moka_store::MokaStore;
use tower_sessions_sqlx_store::SqliteStore;

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
const SESSION_COOKIE_NAME: &str = "betsubetsu";
static MIGRATOR: Migrator = sqlx::migrate!();
const SERVER_HOST: &str = "127.0.0.1";

/// setup the whole app
async fn app(args: &Args) -> anyhow::Result<Router> {
    // create store for backend.  Stores an api_token.
    let state = {
        let pool = Pool::connect_with(
            SqliteConnectOptions::from_str("splittinger.db")
                .context("Could not parse db location")?
                .log_statements(log::LevelFilter::Error)
                .create_if_missing(true),
        )
        .await
        .context("Error in DB")?;
        MIGRATOR.run(&pool).await?;

        //sqlx::migrate!().run(&pool).await?;
        AppState {
            pool,
            args: args.clone(),
        }
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

    let moka_store = MokaStore::new(Some(10_000));
    let caching_store = CachingSessionStore::new(moka_store, session_store);

    let session_service = SessionManagerLayer::new(caching_store)
        .with_secure(false)
        .with_name(SESSION_COOKIE_NAME)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));
    let backend = Router::new()
        .merge(services::back_public_route(state))
        //.merge(back_token_route(state))
        .layer(session_service);

    // combine the front and backend into server
    Ok(Router::new()
        .merge(services::front_public_route(args.clone()))
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
                "tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let host = if cli.listen_global {
        "0.0.0.0"
    } else {
        SERVER_HOST
    };
    let addr: SocketAddr = format!("{}:{}", host, cli.port)
        .parse()
        .context("Can not parse address and port")?;

    let app = app(&cli).await?;
    println!("listening on http://{}", addr);

    let cert_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("certs")
        .join("cert.pem");
    let key_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("certs")
        .join("key.pem");
    if cli.serve_tls && (!cert_file.exists() || !key_file.exists()) {
        return Err(anyhow!(
            "Please create cert and keyfile in certs subdirectory"
        ));
    }

    if cli.serve_tls {
        let config = RustlsConfig::from_pem_file(cert_file, key_file)
            .await
            .unwrap();
        //let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .context("Could not bind server")?;
    } else {
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await
            .context("Could not bind server")?;
    }
    Ok(())
}
