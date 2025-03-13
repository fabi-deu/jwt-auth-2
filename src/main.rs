use std::env;
use std::sync::Arc;
use axum::{middleware, Extension, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum_extra::extract::cookie::Key;
use axum_extra::extract::PrivateCookieJar;
use dotenv::dotenv;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceBuilder;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use jwt_auth_lib::handlers::user::auth_test::auth_test;
use jwt_auth_lib::handlers::user::login::login;
use jwt_auth_lib::handlers::user::new::create_new_user;
use jwt_auth_lib::handlers::user::refresh::access_token::refresh_access_token;
use jwt_auth_lib::handlers::user::refresh::refresh_token::refresh_refresh_token;
use jwt_auth_lib::middleware::user::auth::auth_middleware;
use jwt_auth_lib::middleware::user::refresh_auth::refresh_token_auth_middleware;
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
    let pool: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(1024)
        .connect(&sqlite_url).await.unwrap();
    info!("Successfully connected to sqlite ✔");

    // appstate
    let appstate: AppstateWrapper = AppstateWrapper(Arc::new(Appstate::new(
        pool, jwt_secret, Key::try_from(cookie_secret.as_bytes()).unwrap()
    )));


    // set up routes
    let pub_routes = Router::new()
        .route("/new", post(create_new_user))
        .route("/login", post(login))
        .route("/refresh/refresh_token", post(refresh_refresh_token))
        .with_state(appstate.clone());


    let protected_routes = Router::new()
        .route("/auth_test", get(auth_test))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(auth_middleware))
                .layer(Extension(appstate.clone()))
        );

    let refresh_token_protected_routes = Router::new()
        .route("/refresh/access_token", get(refresh_access_token))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(refresh_token_auth_middleware))
                .layer(Extension(appstate.clone()))
        );


    let app = Router::new()
        .nest("/v1/user/", protected_routes)
        .nest("/v1/user", refresh_token_protected_routes)
        .layer(Extension(appstate.clone()))
        .nest("/v1/user/", pub_routes)
        .route("/cookie_test", get(test))
        .with_state(appstate);


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port)).await.unwrap();
    info!("Serving on port {}", port);
    axum::serve(listener, app).await.unwrap();
}


async fn test(
    State(wrapped_appstate): State<AppstateWrapper>,
    jar: PrivateCookieJar,
) -> StatusCode {
    let appstate = wrapped_appstate.0;

    println!("{:#?}", jar);

    let a = jar.get("access_token").unwrap();
    let r = jar.get("refresh_token").unwrap();

    println!("decrypted: {}, {}", a.value(), r.value());

    StatusCode::OK
}

