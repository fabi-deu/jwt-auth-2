use std::env;
use dotenv::dotenv;

#[tokio::main]
fn main() {
    println!("Starting...");

    // load environment
    dotenv().ok();

    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let cookie_secret = env::var("COOKIE_SECRET").unwrap();

    // postgres conn

}

