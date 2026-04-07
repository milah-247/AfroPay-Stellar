use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionJob {
    pub tx_id: String,
    pub user_id: String,
    pub source_secret: String,
    pub destination_public_key: String,
    pub amount: String,
    pub asset_code: String,
    pub asset_issuer: Option<String>,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobResult {
    pub tx_id: String,
    pub success: bool,
    pub stellar_hash: Option<String>,
    pub error: Option<String>,
}
