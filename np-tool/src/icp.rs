use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

// Base URL for the ledger API
const LEDGER_API_URL: &str = "https://ledger-api.internetcomputer.org";

// Custom error type
#[derive(Debug)]
pub enum FetchError {
    Request(reqwest::Error),
    Parse(serde_json::Error),
    Other(String),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchError::Request(e) => write!(f, "Request error: {}", e),
            FetchError::Parse(e) => write!(f, "JSON parsing error: {}", e),
            FetchError::Other(s) => write!(f, "Error: {}", s),
        }
    }
}

impl Error for FetchError {}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> Self {
        FetchError::Request(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> Self {
        FetchError::Parse(err)
    }
}

// Ledger transaction structure
#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    block_height: String,
    transaction_hash: String,
    from_account_identifier: Option<String>,
    to_account_identifier: String,
    transfer_type: String,
    amount: String,
    fee: String,
    memo: String,
    created_at: u64,
}

// API response structure
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    total: u32,
    blocks: Vec<Transaction>,
}

// Reward summary structure
#[derive(Debug, Serialize)]
pub struct RewardSummary {
    pub total_transactions: usize,
    pub total_e8s: u64,
    pub total_icp: f64,
    pub first_transaction_timestamp: Option<u64>,
    pub last_transaction_timestamp: Option<u64>,
}

// Function to fetch mint transactions for an account
pub async fn fetch_account_mint_transactions(account_id: &str) -> Result<ApiResponse, FetchError> {
    println!("Fetching mint transactions for account: {}", account_id);

    let url = format!(
        "{}/accounts/{}/transactions?limit=1000&offset=0&transfer_type=mint",
        LEDGER_API_URL, account_id
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("accept", "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(FetchError::Other(format!(
            "API returned status code: {}",
            response.status()
        )));
    }

    let json_data = response.text().await?;
    let transactions: ApiResponse = serde_json::from_str(&json_data)?;

    println!(
        "Successfully fetched {} mint transactions for account",
        transactions.blocks.len()
    );

    Ok(transactions)
}

// Calculate total rewards from transactions
pub fn calculate_rewards(transactions: &ApiResponse) -> RewardSummary {
    let mut total_e8s: u64 = 0;
    let mut first_timestamp: Option<u64> = None;
    let mut last_timestamp: Option<u64> = None;

    for tx in &transactions.blocks {
        if tx.transfer_type == "mint" {
            // Parse amount string to u64
            if let Ok(amount) = tx.amount.parse::<u64>() {
                total_e8s += amount;
            }

            // Track timestamps
            match (first_timestamp, last_timestamp) {
                (None, _) => {
                    first_timestamp = Some(tx.created_at);
                    last_timestamp = Some(tx.created_at);
                }
                (Some(first), None) => {
                    if tx.created_at < first {
                        first_timestamp = Some(tx.created_at);
                    }
                    last_timestamp = Some(tx.created_at);
                }
                (Some(first), Some(last)) => {
                    if tx.created_at < first {
                        first_timestamp = Some(tx.created_at);
                    }
                    if tx.created_at > last {
                        last_timestamp = Some(tx.created_at);
                    }
                }
            }
        }
    }

    // Convert total E8s to ICP (1 ICP = 10^8 E8s)
    let total_icp = total_e8s as f64 / 100_000_000.0;

    RewardSummary {
        total_transactions: transactions.blocks.len(),
        total_e8s,
        total_icp,
        first_transaction_timestamp: first_timestamp,
        last_transaction_timestamp: last_timestamp,
    }
}

// Main function to fetch and calculate rewards
pub async fn get_account_rewards(account_id: &str) -> Result<RewardSummary, FetchError> {
    let transactions = fetch_account_mint_transactions(account_id).await?;
    let summary = calculate_rewards(&transactions);
    Ok(summary)
}
