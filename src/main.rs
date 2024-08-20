mod models;
mod server;
mod utils;

use server::server::server_router;

#[tokio::main]
async fn main() {
    let _ = server_router().await;
}
