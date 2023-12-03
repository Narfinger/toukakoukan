use axum::{
    async_trait,
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    BoxError, Form, Router,
};
use axum_login::{
    login_required,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder, AuthUser, AuthnBackend, UserId,
};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tower::ServiceBuilder;

use crate::AppState;

type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub(crate) struct User {
    id: i64,
    password: String,
    name: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password.as_bytes()
    }
}
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Backend {
    db: SqlitePool,
}

impl Backend {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("select * from users where username = ? ")
            .bind(creds.username)
            .fetch_optional(&self.db)
            .await?;

        Ok(user.filter(|user| {
            verify_password(creds.password, &user.password)
                .ok()
                .is_some() // We're using password-based authentication--this
                           // works by comparing our form input with an argon2
                           // password hash.
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("select * from users where id = ?")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}

async fn login(mut auth_session: AuthSession, Form(creds): Form<Credentials>) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/protected").into_response()
}

fn protected_routes() -> Router {
    Router::new()
        .route(
            "/protected",
            get(|| async { "Gotta be logged in to see me!" }),
        )
        .route_layer(login_required!(Backend, login_url = "/login"))
}

async fn get_login() -> impl IntoResponse {
    String::from("PLEASE LOGIN").into_response()
}

async fn post_login(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return String::from("LOGIN ERROR").into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/").into_response()
}

async fn protected(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => format!("You are a user {:?}", user).into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub(crate) fn login_auth_routes(pool: SqlitePool) -> Router<()> {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = Backend::new(pool);
    let auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayerBuilder::new(backend, session_layer).build());

    let auth_routes: Router<()> = Router::new()
        .route("/login", post(post_login))
        .route("/login", get(get_login));

    Router::new()
        .route("/", get(protected))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .merge(auth_routes)
        .layer(auth_service)
}
