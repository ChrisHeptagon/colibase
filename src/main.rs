pub mod server;
use crate::server::server::main_server;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");
    main_server().await;
}