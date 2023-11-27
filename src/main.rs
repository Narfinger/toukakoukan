use axum::{
    extract::{FromRef, Path},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_template::{engine::Engine, Key, RenderHtml};
use handlebars::Handlebars;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type AppEngine = Engine<Handlebars<'static>>;
#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

async fn index(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("index", engine, ())
}

#[tokio::main]
async fn main() {
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
    let app = Router::new().route("/", get(index)).with_state(AppState {
        engine: Engine::from(hbs),
    });
    // run it with hyper on localhost:3000
    println!("See example: http://127.0.0.1:3000/example");

    axum::Server::bind(&([127, 0, 0, 1], 3000).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
