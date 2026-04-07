use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::{json, Value};
use crate::models::TransactionJob;

pub async fn submit_transaction(job: &TransactionJob) -> Result<String> {
    let horizon_url = std::env::var("STELLAR_HORIZON_URL")
        .unwrap_or_else(|_| "https://horizon-testnet.stellar.org".into());

    let client = Client::new();

    // Fetch source account sequence
    let account_url = format!("{}/accounts/{}", horizon_url, derive_public_key(&job.source_secret)?);
    let account: Value = client.get(&account_url).send().await?.json().await?;
    let sequence: i64 = account["sequence"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing sequence"))?
        .parse()?;

    // Build and sign transaction envelope via Stellar XDR
    // NOTE: In production, use stellar-xdr crate for full XDR building.
    // This stub delegates to the Horizon fee-bump or a pre-built XDR.
    let envelope_xdr = build_payment_xdr(job, sequence + 1)?;

    let params = [("tx", envelope_xdr.as_str())];
    let resp: Value = client
        .post(format!("{}/transactions", horizon_url))
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    if let Some(hash) = resp["hash"].as_str() {
        Ok(hash.to_string())
    } else {
        Err(anyhow!("Submission failed: {}", resp))
    }
}

fn derive_public_key(secret: &str) -> Result<String> {
    // Placeholder: use stellar-base keypair derivation in production
    Ok(secret[..56].to_string())
}

fn build_payment_xdr(job: &TransactionJob, _sequence: i64) -> Result<String> {
    // Placeholder: integrate stellar-xdr crate for real XDR construction
    // Returns a stub envelope for compilation purposes
    Ok(format!(
        "STUB_XDR_{}_{}_{}",
        job.tx_id, job.amount, job.asset_code
    ))
}
