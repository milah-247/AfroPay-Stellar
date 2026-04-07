use anyhow::Result;
use redis::AsyncCommands;
use tracing::{error, info};
use crate::models::TransactionJob;
use crate::stellar::submit_transaction;

pub async fn listen() -> Result<()> {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".into());
    let client = redis::Client::open(redis_url)?;
    let mut conn = client.get_async_connection().await?;

    info!("Listening on Redis queue: stellar_jobs");

    loop {
        let result: Option<(String, String)> = conn
            .blpop("stellar_jobs", 0.0)
            .await
            .unwrap_or(None);

        if let Some((_, payload)) = result {
            match serde_json::from_str::<TransactionJob>(&payload) {
                Ok(job) => {
                    info!("Processing job: {}", job.tx_id);
                    match submit_transaction(&job).await {
                        Ok(hash) => info!("Job {} succeeded: {}", job.tx_id, hash),
                        Err(e) => error!("Job {} failed: {}", job.tx_id, e),
                    }
                }
                Err(e) => error!("Failed to parse job: {}", e),
            }
        }
    }
}
