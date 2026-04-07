mod stellar;
mod queue;
mod models;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("RemitX Rust Worker starting...");
    queue::listen().await
}
