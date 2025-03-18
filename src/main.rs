use std::env;
use std::sync::Arc;
use axum_extra::extract::cookie::Key;
use dotenv::dotenv;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use clap::Parser;
use jwt_auth_lib::models::appstate::{Appstate, AppstateWrapper};
use jwt_auth_lib::route::get_default_router;

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
    let args = Args::parse();

    let port = args.port;
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let cookie_secret = env::var("COOKIE_SECRET").unwrap();
    let sqlite_url = env::var("DATABASE_URL").unwrap();
    info!("Successfully loaded environment variables ✔");

    // sqlite3 connection
    let pool: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(1024)
        .connect(&sqlite_url).await.unwrap();
    info!("Successfully connected to sqlite ✔");

    // appstate
    let appstate: AppstateWrapper = AppstateWrapper(Arc::new(Appstate::new(
        pool, jwt_secret, Key::try_from(cookie_secret.as_bytes()).unwrap()
    )));


    // get routes
    let app = get_default_router(appstate, "v1");


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port)).await.unwrap();
    info!("Serving on port {}", port);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t=8000)]
    port: usize,
}

