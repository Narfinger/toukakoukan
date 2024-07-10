use anyhow::{anyhow, Context};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use yansi::Paint;

use sqlx::{migrate::Migrator, sqlite::SqliteConnectOptions, ConnectOptions, Pool};
use std::{env, net::SocketAddr, path::PathBuf, str::FromStr};
use time::Duration;
use tower_http::trace::TraceLayer;
use tower_sessions::{CachingSessionStore, Expiry, SessionManagerLayer};
use tower_sessions_moka_store::MokaStore;
use tower_sessions_sqlx_store::SqliteStore;
use tracing::info;

use tracing_subscriber::{
    filter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};
use types::Args;

use crate::types::AppState;

mod api;
mod group;
mod routes;
mod types;
mod users;

// SETUP Constants
const SESSION_COOKIE_NAME: &str = "toukakoukan";
static MIGRATOR: Migrator = sqlx::migrate!();
const SERVER_HOST: &str = "127.0.0.1";
const FRONT_PUBLIC: &str = "../front_end/dist";
const DEFAULT_DATABASE: &str = "toukakoukan.db";

/// setup the whole app
async fn app(args: &Args) -> anyhow::Result<Router> {
    // create store for backend.  Stores an api_token.
    let state = {
        let pool_path = args
            .database
            .clone()
            .unwrap_or(PathBuf::from(DEFAULT_DATABASE));

        if !pool_path.exists() {
            info!("Creating new database at {:?}", pool_path);
        }
        let p = pool_path.as_os_str().to_str().unwrap();
        let pool = Pool::connect_with(
            SqliteConnectOptions::from_str(p)
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

    let dir = args.js.to_owned().unwrap_or(PathBuf::from(FRONT_PUBLIC));
    if !dir.exists() {
        return Err(anyhow!("The javascript dir does not exist ({:?})", dir));
    }

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
        .with_expiry(Expiry::OnInactivity(Duration::seconds(60 * 60 * 24)));
    let backend = Router::new()
        .merge(routes::back_public_route(state))
        //.merge(back_token_route(state))
        .layer(session_service);

    // combine the front and backend into server
    Ok(Router::new()
        .merge(routes::front_public_route(dir))
        .merge(backend)
        .layer(TraceLayer::new_for_http()))
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Args::parse();

    if cli.release {
        println!("{}", "In release mode!".green());
    } else {
        println!(
            "{}",
            "WARNING: IN DEBUG MODE, EVERYTHING IS INSECURE!".red()
        );
    }
    if cli.user_creation {
        println!("{}", "User creation is enabled!").red();
    }

    let mut filter =
        EnvFilter::try_from_default_env()?.add_directive("sqlx::migrations=error".parse()?);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
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

    let cwd = env::current_dir()?;
    let cert_file = cwd.join("certs").join("cert.pem");
    let key_file = cwd.join("certs").join("key.pem");
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
