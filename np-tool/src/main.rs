pub mod addresses;
pub mod helper;
pub mod transactions;

use addresses::{CEXES, DEFI, FOUNDATION, IDENTIFIED, NODE_PROVIDERS, SNSES, SNS_PARTICIPANTS, SPAMMERS, SUSPECTS};
use candid::Principal;
use ic_agent::Agent;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error as ThisError;
use transactions::fetch_account_transactions;

const IC_URL: &str = "https://ic0.app";
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
/// AccountData
///

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountData {
    name: String,
    principal: Option<Principal>,
    account: Option<String>,
    ty: Type,
}

impl AccountData {
    pub fn new(name: &str, address: &str, ty: Type) -> Self {
        let (principal, account) = if address.contains("-") {
            (Some(Principal::from_text(address).unwrap()), None)
        } else {
            (None, Some(address.to_string()))
        };

        Self { name: name.to_string(), principal, account, ty }
    }
}

///
/// AccountType
///

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Cex,
    Defi,
    Foundation,
    Identified,
    NodeProvider,
    Spammer,
    Sns,
    SnsParticipant,
    Suspect,
}

//
// main
//

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = Agent::builder().with_url(IC_URL).build()?;

    // Initialize the agent (fetch root key in development)
    agent.fetch_root_key().await?;

    let mut results = Vec::new();
    for entry in get_entries() {
        match fetch_account_transactions(entry, &agent).await {
            Ok(account_tx) => results.push(account_tx),
            Err(e) => eprintln!("Error fetching account transactions: {}", e),
        }
    }

    let json_string = serde_json::to_string_pretty(&results)?;
    std::fs::write("./../frontend/public/account_transactions.json", json_string)?;
    println!("Saved combined account transactions to account_transactions.json");

    Ok(())
}

// get_entries
fn get_entries() -> Vec<AccountData> {
    let mut entries = Vec::new();

    // named
    entries.extend(CEXES.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Cex)));
    entries.extend(DEFI.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Defi)));
    entries.extend(IDENTIFIED.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Identified)));
    entries.extend(NODE_PROVIDERS.iter().map(|(name, addr)| AccountData::new(name, addr, Type::NodeProvider)));
    entries.extend(SNSES.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Sns)));
    entries.extend(SNS_PARTICIPANTS.iter().map(|(name, addr)| AccountData::new(name, addr, Type::SnsParticipant)));
    entries.extend(SUSPECTS.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Suspect)));
    entries.extend(FOUNDATION.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Foundation)));
    entries.extend(SPAMMERS.iter().map(|addr| AccountData::new(&addr[..5], addr, Type::Spammer)));

    // no name

    // check for dupes
    let mut seen_account_ids = HashSet::new();
    let mut seen_principals = HashSet::new();
    print!("Validating {} addresses...", entries.len());
    for entry in &entries {
        if let Some(acc) = &entry.account {
            if !seen_account_ids.insert(acc) {
                panic!("duplicate account found: {acc}");
            }
        }

        if let Some(pid) = &entry.principal {
            if !seen_principals.insert(pid) {
                panic!("duplicate principal found: {pid}");
            }
        }
    }
    println!(" ok");

    entries
}
