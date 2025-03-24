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
    account: Option<String>,
    ty: Type,
}

impl Account {
    pub fn new(name: &str, address: &str, ty: Type) -> Self {
        let (principal, account) = match Principal::from_text(address) {
            Ok(p) => (Some(p), None),
            Err(_) => (None, Some(address.to_string())),
        };

        Self {
            name: name.to_string(),
            principal,
            account,
            ty,
        }
    }
}

///
/// AccountType
///

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Type {
    Exchange,
    Individual,
    NodeProvider,
    Sns,
    #[default]
    Unknown,
}

fn main() {
    // open file

    for account in get_accounts() {
        println!("{:?}", account);
    }
}

fn get_accounts() -> Vec<Account> {
    vec![
        // Individuals
        Account::new(
            "Austin Fatheree",
            "jrnhz-6ekxv-2fffs-wfcgt-l3pe7-456id-heznf-xyf64-nykjq-4jyso-zae",
            Type::Individual,
        ),
        // Node Providers
        Account::new(
            "Staking Facilities",
            "niw4y-easue-l3qvz-sozsi-tfkvb-cxcx6-pzslg-5dqld-ooudp-hsuui-xae",
            Type::NodeProvider,
        ),
        // SNS
        Account::new(
            "Boom DAO",
            "38af024c2f3a8681e661b67065e5d83d692d16252a7cdd96bae94452b65f498d",
            Type::Sns,
        ),
    ]
}
