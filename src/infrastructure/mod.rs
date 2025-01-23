use std::env;
use dotenv::dotenv;

pub fn get_server_address() -> String {
    dotenv().ok();
    let port = env::var("SERVICE_PORT").unwrap_or_else(|_| "8080".to_string());
    format!("0.0.0.0:{}", port)
}
