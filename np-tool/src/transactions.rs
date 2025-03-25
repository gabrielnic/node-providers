use candid::{CandidType, Decode, Encode};
use crc32fast::Hasher as Crc32Hasher;
use ic_agent::{export::Principal, Agent};
use serde::{Deserialize, Serialize};

use crate::{account_id_to_hex, principal_to_account_id, AccountData};

const GOVERNANCE_CANISTER_ID: &str = "qhbym-qaaaa-aaaaa-aaafq-cai";

#[derive(CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct GetAccountTransactionsArgs {
    pub max_results: u64,
    pub start: Option<u64>,
    pub account_identifier: String,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct Tokens {
    pub e8s: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct TimeStamp {
    pub timestamp_nanos: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub enum Operation {
    Approve {
        fee: Tokens,
        from: String,
        allowance: Tokens,
        expected_allowance: Option<Tokens>,
        expires_at: Option<TimeStamp>,
        spender: String,
    },
    Burn {
        from: String,
        amount: Tokens,
        spender: Option<String>,
    },
    Mint {
        to: String,
        amount: Tokens,
    },
    Transfer {
        to: String,
        fee: Tokens,
        from: String,
        amount: Tokens,
        spender: Option<String>,
    },
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct Transaction {
    pub memo: u64,
    pub icrc1_memo: Option<serde_bytes::ByteBuf>,
    pub operation: Operation,
    pub timestamp: Option<TimeStamp>,
    pub created_at_time: Option<TimeStamp>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct TransactionWithId {
    pub id: u64,
    pub transaction: Transaction,
}

#[derive(CandidType, Deserialize)]
pub struct GetAccountIdentifierTransactionsResponse {
    pub balance: u64,
    pub transactions: Vec<TransactionWithId>,
    pub oldest_tx_id: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct GetAccountIdentifierTransactionsError {
    pub message: String,
}

#[derive(CandidType, Deserialize)]
pub enum GetAccountIdentifierTransactionsResult {
    Ok(GetAccountIdentifierTransactionsResponse),
    Err(GetAccountIdentifierTransactionsError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountTransactionsJson {
    pub name: String,
    pub principal: Option<String>, // we'll convert the Principal to a text string if present
    pub account: Option<String>,
    pub ty: String, // as a string representation
    pub transactions: Vec<TransactionWithId>,
    pub balance: u64,
    pub oldest_tx_id: Option<u64>,
}

fn is_valid_account_id(account_id_hex: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if account_id_hex.len() != 64 {
        return Ok(false);
    }

    let account_bytes = hex::decode(account_id_hex)?;
    if account_bytes.len() != 32 {
        return Ok(false);
    }

    let mut hasher = Crc32Hasher::new();
    hasher.update(&account_bytes[4..]); // bytes[4..32]
    let computed_checksum = hasher.finalize();

    let provided_checksum =
        u32::from_be_bytes([account_bytes[0], account_bytes[1], account_bytes[2], account_bytes[3]]);

    Ok(computed_checksum == provided_checksum)
}

pub async fn fetch_account_transactions(
    account_data: AccountData,
    agent: &Agent,
) -> Result<AccountTransactionsJson, Box<dyn std::error::Error>> {
    let account_identifier = if let Some(principal) = account_data.principal {
        let account_id = principal_to_account_id(&principal, None);
        account_id_to_hex(account_id) // Convert the byte array to a hex string
    } else if let Some(account_hex) = account_data.account.clone() {
        account_hex
    } else {
        return Err("No principal or account id provided".into());
    };

    if !is_valid_account_id(&account_identifier)? {
        return Err("Invalid account ID".into());
    }
    println!("Fetching txs data for account {} ", account_identifier);
    // Create the query arguments.
    let request =
        GetAccountTransactionsArgs { max_results: 10000, start: None, account_identifier: account_identifier.clone() };
    let args = Encode!(&request)?;

    // Call the governance canister
    let principal = Principal::from_text(GOVERNANCE_CANISTER_ID)?;

    let response_bytes = agent.query(&principal, "get_account_identifier_transactions").with_arg(args).call().await?;

    let result = Decode!(response_bytes.as_slice(), GetAccountIdentifierTransactionsResult)?;

    let (balance, transactions, oldest_tx_id) = match result {
        GetAccountIdentifierTransactionsResult::Ok(resp) => (resp.balance, resp.transactions, resp.oldest_tx_id),
        GetAccountIdentifierTransactionsResult::Err(err) => return Err(err.message.into()),
    };

    // Build our final JSON output. We convert the principal (if any) to a text string.
    let output = AccountTransactionsJson {
        name: account_data.name,
        principal: account_data.principal.map(|p| p.to_text()),
        account: Some(account_identifier),
        ty: format!("{:?}", account_data.ty),
        transactions,
        balance,
        oldest_tx_id,
    };

    Ok(output)
}
