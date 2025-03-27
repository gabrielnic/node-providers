use std::collections::{HashMap, HashSet};

use crate::{
    helper::{is_valid_account_id, principal_to_account_id},
    AccountData,
};
use candid::{CandidType, Decode, Encode};
use ic_agent::{export::Principal, Agent};
use icp_ledger::AccountIdentifier;
use serde::{Deserialize, Serialize};

const INDEX_CANISTER_ID: &str = "qhbym-qaaaa-aaaaa-aaafq-cai";
const GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

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
    pub ty: String,
    extra_accounts: Vec<String>,
    pub transactions: Vec<TransactionWithId>,
    pub balance: u64,
    pub oldest_tx_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
struct GovAccountIdentifierentifier {
    hash: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct NodeProviderReward {
    id: Option<Principal>,
    reward_account: Option<GovAccountIdentifierentifier>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct RewardToNeuron {
    dissolve_delay_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct RewardToAccount {
    to_account: Option<GovAccountIdentifierentifier>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
enum RewardMode {
    RewardToNeuron(RewardToNeuron),
    RewardToAccount(RewardToAccount),
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct RewardNodeProvider {
    node_provider: Option<NodeProviderReward>,
    reward_mode: Option<RewardMode>,
    amount_e8s: u64,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct XdrConversionRate {
    xdr_permyriad_per_icp: Option<u64>,
    timestamp_seconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct DateRangeFilter {
    start_timestamp_seconds: Option<u64>,
    end_timestamp_seconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct ListNodeProviderRewardsRequest {
    date_filter: Option<DateRangeFilter>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct MonthlyNodeProviderRewards {
    timestamp: u64,
    rewards: Vec<RewardNodeProvider>,
    xdr_conversion_rate: Option<XdrConversionRate>,
    #[serde(default)]
    pub node_providers: Vec<NodeProviderReward>,
    #[serde(default)]
    pub registry_version: Option<u64>,
    #[serde(default)]
    pub minimum_xdr_permyriad_per_icp: Option<u64>,
    #[serde(default)]
    pub maximum_node_provider_rewards_e8s: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct ListNodeProviderRewardsResponse {
    pub rewards: Vec<MonthlyNodeProviderRewards>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ChecksumError {
    input: [u8; 32],
    expected_checksum: [u8; 4],
    found_checksum: [u8; 4],
}

#[derive(Debug, Clone, Serialize)]
pub struct ProviderRewardInfo {
    reward_account_hex: Option<String>,
    pub reward_account_formatted: Option<String>,
    reward_account_dashboard_link: Option<String>,
    most_recent_reward_e8s: Option<u64>,
    most_recent_reward_xdr: Option<f64>,
    most_recent_timestamp: Option<u64>,
    total_mint_rewards_e8s: Option<u64>,
    total_mint_rewards_icp: Option<f64>,
    mint_transaction_count: Option<u32>,
    first_mint_timestamp: Option<u64>,
    last_mint_timestamp: Option<u64>,
}

pub fn process_account_hex(hex: &str) -> (Option<String>, Option<String>, Option<String>) {
    // Original hex
    let orig_hex = Some(hex.to_string());

    // Try to convert to proper AccountIdentifier format
    if let Ok(account) = AccountIdentifier::from_hex(hex) {
        let formatted = account.to_hex();
        let dashboard_link = format!("https://dashboard.internetcomputer.org/account/{}", formatted);
        return (orig_hex, Some(formatted), Some(dashboard_link));
    }

    // If conversion fails, return only the original hex
    (orig_hex, None, None)
}

pub fn process_rewards_data(rewards_response: ListNodeProviderRewardsResponse) -> HashMap<String, ProviderRewardInfo> {
    let mut result = HashMap::new();

    for monthly_reward in rewards_response.rewards {
        // Get XDR conversion rate
        let xdr_rate = monthly_reward.xdr_conversion_rate.and_then(|rate| rate.xdr_permyriad_per_icp).unwrap_or(0)
            as f64
            / 10000.0; // Convert from permyriad to ratio

        for reward in monthly_reward.rewards {
            if let Some(node_provider) = reward.node_provider {
                if let Some(id) = node_provider.id {
                    let principal_id = id.to_text();

                    // Get reward account hash
                    let (reward_account_hex, reward_account_formatted, reward_account_dashboard_link) =
                        if let Some(account) = &node_provider.reward_account {
                            let hex = hex::encode(&account.hash);
                            process_account_hex(&hex)
                        } else if let Some(RewardMode::RewardToAccount(account)) = &reward.reward_mode {
                            if let Some(acc) = &account.to_account {
                                let hex = hex::encode(&acc.hash);
                                process_account_hex(&hex)
                            } else {
                                (None, None, None)
                            }
                        } else {
                            (None, None, None)
                        };

                    // Convert E8s to ICP then to XDR
                    let reward_xdr =
                        if xdr_rate > 0.0 { (reward.amount_e8s as f64 / 100_000_000.0) * xdr_rate } else { 0.0 };

                    // Add or update reward info in the map
                    result
                        .entry(principal_id)
                        .and_modify(|info: &mut ProviderRewardInfo| {
                            // Update most recent info if this reward is newer
                            if let Some(current_ts) = info.most_recent_timestamp {
                                if monthly_reward.timestamp > current_ts {
                                    info.most_recent_timestamp = Some(monthly_reward.timestamp);
                                    info.most_recent_reward_e8s = Some(reward.amount_e8s);
                                    info.most_recent_reward_xdr = Some(reward_xdr);

                                    // Update reward account only if we have a new one
                                    if reward_account_hex.is_some() {
                                        info.reward_account_hex = reward_account_hex.clone();
                                        info.reward_account_formatted = reward_account_formatted.clone();
                                        info.reward_account_dashboard_link = reward_account_dashboard_link.clone();
                                    }
                                }
                            }
                        })
                        .or_insert_with(|| ProviderRewardInfo {
                            reward_account_hex: reward_account_hex.clone(),
                            reward_account_formatted: reward_account_formatted.clone(),
                            reward_account_dashboard_link: reward_account_dashboard_link.clone(),
                            most_recent_reward_e8s: Some(reward.amount_e8s),
                            most_recent_reward_xdr: Some(reward_xdr),
                            most_recent_timestamp: Some(monthly_reward.timestamp),
                            first_mint_timestamp: None,
                            last_mint_timestamp: None,
                            mint_transaction_count: None,
                            total_mint_rewards_e8s: None,
                            total_mint_rewards_icp: None,
                        });
                }
            }
        }
    }

    result
}

pub async fn fetch_nodes_rewards(agent: &Agent) -> Result<ListNodeProviderRewardsResponse, Box<dyn std::error::Error>> {
    let request = ListNodeProviderRewardsRequest { date_filter: None };

    // Encode the request using Candid
    let args = Encode!(&request)?;

    // Call the governance canister
    let principal = Principal::from_text(GOVERNANCE_CANISTER_ID)?;
    let response = agent.query(&principal, "list_node_provider_rewards").with_arg(args).call().await?;

    // Decode the response
    let result = Decode!(response.as_slice(), ListNodeProviderRewardsResponse)?;

    Ok(result)
}

pub async fn get_accounts_from_rewards(principal: Principal, rewards: ListNodeProviderRewardsResponse) -> Vec<String> {
    // Compute the default account identifier for the given principal (with default subaccount)
    let default_account: [u8; 32] = principal_to_account_id(&principal, None);
    let default_vec = default_account.to_vec();

    let mut extra_accounts: HashSet<String> = HashSet::new();

    for monthly in &rewards.rewards {
        for reward in &monthly.rewards {
            // Check if the reward mode is RewardToAccount.
            if let Some(RewardMode::RewardToAccount(ref reward_to_account)) = reward.reward_mode {
                if let Some(ref account) = reward_to_account.to_account {
                    // If the reward account's hash is different from the default, record it.
                    if account.hash != default_vec {
                        let hex = account.hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
                        extra_accounts.insert(hex);
                    }
                }
            }
        }
    }

    // Convert the HashSet into a Vec. Order is not guaranteed; use a BTreeSet if you need sorting.
    extra_accounts.into_iter().collect()
}

pub async fn fetch_account_transactions(
    account_data: AccountData,
    agent: &Agent,
) -> Result<AccountTransactionsJson, Box<dyn std::error::Error>> {
    let account_identifier = if let Some(principal) = account_data.principal {
        let account_id = principal_to_account_id(&principal, None);
        hex::encode(account_id)
    } else if let Some(account_hex) = account_data.account.clone() {
        account_hex
    } else {
        return Err("No principal or account id provided".into());
    };

    if !is_valid_account_id(&account_identifier)? {
        return Err("Invalid account ID".into());
    }
    println!("Fetching txs data for account {}", account_identifier);

    let request =
        GetAccountTransactionsArgs { max_results: 10000, start: None, account_identifier: account_identifier.clone() };
    let args = Encode!(&request)?;
    let principal = Principal::from_text(INDEX_CANISTER_ID)?;

    let response_bytes = agent.query(&principal, "get_account_identifier_transactions").with_arg(args).call().await?;
    let result = Decode!(response_bytes.as_slice(), GetAccountIdentifierTransactionsResult)?;
    let (balance, mut transactions, oldest_tx_id) = match result {
        GetAccountIdentifierTransactionsResult::Ok(resp) => (resp.balance, resp.transactions, resp.oldest_tx_id),
        GetAccountIdentifierTransactionsResult::Err(err) => return Err(err.message.into()),
    };

    let rewards = fetch_nodes_rewards(&agent).await?;
    let rewards_by_principal = process_rewards_data(rewards);
    let extra_account: Option<String> = if let Some(principal) = account_data.principal {
        rewards_by_principal.get(&Principal::to_string(&principal)).and_then(|rd| rd.clone().reward_account_formatted)
    } else {
        None
    };

    let mut extra_accounts = Vec::new();
    if let Some(extra_acc) = extra_account.clone() {
        extra_accounts.push(extra_acc.clone());
        let extra_request =
            GetAccountTransactionsArgs { max_results: 10000, start: None, account_identifier: extra_acc };
        let extra_args = Encode!(&extra_request)?;
        let extra_response_bytes =
            agent.query(&principal, "get_account_identifier_transactions").with_arg(extra_args).call().await?;
        let extra_result = Decode!(extra_response_bytes.as_slice(), GetAccountIdentifierTransactionsResult)?;
        match extra_result {
            GetAccountIdentifierTransactionsResult::Ok(extra_resp) => {
                transactions.extend(extra_resp.transactions);
                // Optionally, combine balances if needed.
            }
            GetAccountIdentifierTransactionsResult::Err(err) => {
                return Err(err.message.into());
            }
        }
    }

    // 5. Build the final JSON output.
    let output = AccountTransactionsJson {
        name: account_data.name,
        principal: account_data.principal.map(|p| p.to_text()),
        account: Some(account_identifier),
        ty: format!("{:?}", account_data.ty),
        transactions,
        balance,
        extra_accounts,
        oldest_tx_id,
    };

    Ok(output)
}
