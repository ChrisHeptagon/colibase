pub mod server;
use dotenvy::dotenv;
use server::server::main_server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    main_server(addr).await;
}