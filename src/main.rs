use std::env;
use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use axum_extra::extract::cookie::Key;
use dotenv::dotenv;
use sqlx::pool::PoolOptions;
use sqlx::{Pool, Sqlite};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use jwt_auth_lib::models::appstate::{Appstate, AppstateWrapper};

#[tokio::main]
async fn main() {
    // setup tracer for nice looks
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Starting...");

    // load environment
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("8000".to_string()); // default as 8000
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let cookie_secret = env::var("COOKIE_SECRET").unwrap();
    let sqlite_url = env::var("DATABASE_URL").unwrap();
    info!("Successfully loaded environment variables ✔");

    // sqlite3 connection
    let pool: Pool<Sqlite> = PoolOptions::new().connect(&sqlite_url).await.unwrap();
    info!("Successfully connected to sqlite ✔");

    let appstate: AppstateWrapper = AppstateWrapper(Arc::new(Appstate::new(
        pool, jwt_secret, Key::try_from(cookie_secret.as_bytes()).unwrap()
    )));

    let app = Router::new()
        .route("/", get(|| async { "Hello World" }));


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port)).await.unwrap();
    info!("Serving on port {}", port);
    axum::serve(listener, app).await.unwrap();
}

