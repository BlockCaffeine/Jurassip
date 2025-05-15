mod api;
mod serial;
mod protocol;
mod server;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file

    if let Err(e) = server::start_server().await {
        eprintln!("Server error: {}", e);
    }
}
