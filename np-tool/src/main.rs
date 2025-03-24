use candid::Principal;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use thiserror::Error as ThisError;

///
/// Error
///

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Principal error: {0}")]
    Principal(#[from] ic_agent::export::PrincipalError),
}

///
/// Account
///

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Account {
    name: String,
    principal: Option<Principal>,
    account_id: Option<String>,
    ty: AccountType,
}

///
/// AccountType
///

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum AccountType {
    Exchange,
    Individual,
    NodeProvider,
    Sns,
    #[default]
    Unknown,
}

fn main() {
    // open file
    let file = File::open("src/accounts.json").expect("Failed to open accounts.json");
    let reader = BufReader::new(file);

    // deserialize
    let accounts: Vec<Account> =
        serde_json::from_reader(reader).expect("Failed to deserialize accounts.json");

    for account in &accounts {
        println!("{:?}", account);
    }
}
