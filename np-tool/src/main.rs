use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use thiserror::Error;
use ic_agent::{Agent, export::Principal};
use candid::{CandidType, Decode, Encode};
use icp_ledger::{AccountIdentifier, Operation};

// Configuration constants
const JSON_PATH: &str = "node_providers.json";
const TOML_PATH: &str = "node_providers-wiki.toml";
const DOCS_DIR: &str = "../np-list";
const OUTPUT_PATH: &str = "combined_providers.json";
const GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";
const IC_URL: &str = "https://ic0.app";

// Custom error type
#[derive(Error, Debug)]
enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML parsing error: {0}")]
    Toml(String),

    #[error("File hash calculation error: {0}")]
    HashCalculation(String),

    #[error("IC Agent error: {0}")]
    IcAgent(#[from] ic_agent::AgentError),

    #[error("Candid error: {0}")]
    Candid(#[from] candid::Error),

    #[error("Principal error: {0}")]
    Principal(#[from] ic_agent::export::PrincipalError),
}

// Custom result type
type Result<T> = std::result::Result<T, MyError>;

// API Data Structures
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    node_providers: Vec<NodeProvider>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NodeProvider {
    display_name: String,
    location_count: i32,
    locations: Vec<Location>,
    logo_url: Option<String>,
    principal_id: String,
    total_node_allowance: i32,
    total_nodes: i32,
    total_rewardable_nodes: i32,
    total_subnets: i32,
    total_unassigned_nodes: i32,
    website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    dc_key: String,
    display_name: String,
    latitude: f64,
    longitude: f64,
    owner: String,
    region: String,
}

// Wiki Data Structures
#[derive(Debug, Clone)]
struct NodeProviderWikiInfo {
    name: String,
    declaration: Option<String>,
    identity: Option<String>,
    wiki_link: Option<String>,
    additional_documents: HashMap<String, String>,
}

// Document validation structure
#[derive(Debug, Clone, Serialize)]
struct DocumentValidation {
    document_type: String,
    file_path: String,
    expected_hash: String,
    actual_hash: String,
    matches: bool,
}

// Governance API structures for rewards
#[derive(Debug, Serialize, Deserialize, CandidType)]
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
struct MonthlyNodeProviderRewards {
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
struct DateRangeFilter {
    start_timestamp_seconds: Option<u64>,
    end_timestamp_seconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct ListNodeProviderRewardsRequest {
    date_filter: Option<DateRangeFilter>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct ListNodeProviderRewardsResponse {
    rewards: Vec<MonthlyNodeProviderRewards>,
}

// Provider reward info
#[derive(Debug, Clone, Serialize)]
struct ProviderRewardInfo {
    reward_account_hex: Option<String>,
    reward_account_formatted: Option<String>,
    reward_account_dashboard_link: Option<String>,
    most_recent_reward_e8s: Option<u64>,
    most_recent_reward_xdr: Option<f64>,
    most_recent_timestamp: Option<u64>,
    total_rewards_e8s: u64,
    total_rewards_xdr: f64,
}

// Combined Data Structure with rewards
#[derive(Debug, Clone, Serialize)]
struct CombinedNodeProvider {
    // Core identifying information
    name: String,
    toml_id: Option<String>,
    principal_id: String,

    // From API
    location_count: i32,
    total_nodes: i32,
    total_rewardable_nodes: i32,
    total_subnets: i32,
    total_unassigned_nodes: i32,

    // Location info
    regions: Vec<String>,
    countries: Vec<String>,
    towns: Vec<String>,
    dashboard_link: String,

    // From Wiki
    wiki_link: Option<String>,

    // Document validations
    document_validations: Vec<DocumentValidation>,

    // Reward information
    rewards: Option<ProviderRewardInfo>,
}

// Parse JSON API data
fn parse_json(content: &str) -> Result<ApiResponse> {
    Ok(serde_json::from_str(content)?)
}

// Extract location info from location data
fn extract_location_info(locations: &[Location]) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut regions = HashSet::new();
    let mut countries = HashSet::new();
    let mut towns = HashSet::new();

    for location in locations {
        let parts: Vec<&str> = location.region.split(',').collect();

        // Extract region (first part)
        if let Some(region_part) = parts.first() {
            if !region_part.trim().is_empty() {
                regions.insert(region_part.trim().to_string());
            }
        }

        // Extract country (second part if available)
        if parts.len() > 1 {
            if !parts[1].trim().is_empty() {
                countries.insert(parts[1].trim().to_string());
            }
        }

        // Extract town (third part if available)
        if parts.len() > 2 {
            if !parts[2].trim().is_empty() {
                towns.insert(parts[2].trim().to_string());
            }
        }
    }

    (
        regions.into_iter().collect(),
        countries.into_iter().collect(),
        towns.into_iter().collect()
    )
}

// Parse TOML wiki data
fn parse_toml_content(content: &str) -> Result<HashMap<String, NodeProviderWikiInfo>> {
    let parsed_toml: toml::Table = match content.parse() {
        Ok(value) => value,
        Err(err) => return Err(MyError::Toml(format!("Error parsing TOML: {}", err))),
    };

    let mut result = HashMap::new();

    for (section_name, section_value) in parsed_toml {
        if let toml::Value::Table(table) = section_value {
            let mut provider = NodeProviderWikiInfo {
                name: String::new(),
                declaration: None,
                identity: None,
                wiki_link: None,
                additional_documents: HashMap::new(),
            };

            for (key, value) in table {
                match key.as_str() {
                    "name" => {
                        if let toml::Value::String(name) = value {
                            provider.name = name;
                        }
                    }
                    "declaration" => {
                        if let toml::Value::String(decl) = value {
                            if !decl.is_empty() {
                                provider.declaration = Some(decl);
                            }
                        }
                    }
                    "identity" => {
                        if let toml::Value::String(id) = value {
                            if !id.is_empty() {
                                provider.identity = Some(id);
                            }
                        }
                    }
                    "wiki-link" => {
                        if let toml::Value::String(link) = value {
                            if !link.is_empty() {
                                provider.wiki_link = Some(link);
                            }
                        }
                    }
                    // Any other fields go into additional_documents
                    _ => {
                        if let toml::Value::String(doc) = value {
                            if !doc.is_empty() {
                                provider.additional_documents.insert(key, doc);
                            }
                        }
                    }
                }
            }

            // Only add providers with a name
            if !provider.name.is_empty() {
                result.insert(section_name, provider);
            }
        }
    }

    Ok(result)
}

// Process document files in the specified directory
fn process_document_files<P: AsRef<Path>>(base_path: P) -> Result<HashMap<String, HashMap<String, String>>> {
    let mut result = HashMap::new();
    let allowed_docs: HashSet<&str> = [
        "declaration", "identity", "handover", "invoice", "contract",
        "passeport", "order", "registration", "authenticity", "decentralization",
    ].iter().copied().collect();

    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if !file_type.is_dir() {
            continue;
        }

        let toml_id = entry.file_name().to_string_lossy().to_string();
        let provider_dir = entry.path();
        let mut doc_paths = HashMap::new();

        // Process each file in the provider directory
        for file in fs::read_dir(&provider_dir)? {
            let file = file?;
            if !file.file_type()?.is_file() {
                continue;
            }

            let filename = file.file_name().to_string_lossy().to_string();
            // Extract document type (removing file extension)
            if let Some(doc_type) = filename.split('.').next() {
                // Check if it's an allowed document type
                if allowed_docs.contains(doc_type) {
                    let path = file.path().to_string_lossy().to_string();
                    doc_paths.insert(doc_type.to_string(), path);
                }
            }
        }

        if !doc_paths.is_empty() {
            result.insert(toml_id, doc_paths);
        }
    }

    Ok(result)
}

// Calculate SHA-256 hash of a file
fn calculate_file_hash(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    use ring::digest::{Context, SHA256};
    let mut context = Context::new(&SHA256);
    context.update(&buffer);
    let digest = context.finish();

    Ok(hex::encode(digest.as_ref()))
}

// Merge data from all sources
fn merge_node_provider_data(
    api_data: &ApiResponse,
    wiki_data: &HashMap<String, NodeProviderWikiInfo>,
    file_data: &HashMap<String, HashMap<String, String>>,
) -> Vec<CombinedNodeProvider> {
    let mut combined_providers = Vec::new();

    // Create normalized lookup maps
    let mut name_to_wiki: HashMap<String, (String, &NodeProviderWikiInfo)> = HashMap::new();
    for (id, info) in wiki_data {
        name_to_wiki.insert(info.name.to_lowercase(), (id.clone(), info));
    }

    // Process each API provider
    for provider in &api_data.node_providers {
        let normalized_api_name = provider.display_name.to_lowercase();
        let (regions, countries, towns) = extract_location_info(&provider.locations);
        let dashboard_link = format!("https://dashboard.internetcomputer.org/provider/{}", provider.principal_id);

        let mut combined = CombinedNodeProvider {
            name: provider.display_name.clone(),
            toml_id: None,
            principal_id: provider.principal_id.clone(),
            location_count: provider.location_count,
            total_nodes: provider.total_nodes,
            total_rewardable_nodes: provider.total_rewardable_nodes,
            total_subnets: provider.total_subnets,
            total_unassigned_nodes: provider.total_unassigned_nodes,
            regions,
            countries,
            towns,
            dashboard_link,
            wiki_link: None,
            document_validations: Vec::new(),
            rewards: None,
        };

        // Try to match with wiki data based on name
        if let Some((id, wiki_info)) = name_to_wiki.get(&normalized_api_name) {
            combined.toml_id = Some(id.clone());
            combined.wiki_link = wiki_info.wiki_link.clone();

            // Get document paths if available
            if let Some(doc_paths) = file_data.get(id) {
                // Validate documents directly
                let mut validations = Vec::new();

                // Check declaration
                if let Some(expected_hash) = &wiki_info.declaration {
                    if let Some(file_path) = doc_paths.get("declaration") {
                        if let Ok(actual_hash) = calculate_file_hash(file_path) {
                            let matches = expected_hash.to_lowercase() == actual_hash.to_lowercase();
                            validations.push(DocumentValidation {
                                document_type: "declaration".to_string(),
                                file_path: file_path.clone().to_string(),
                                expected_hash: expected_hash.clone(),
                                actual_hash,
                                matches,
                            });
                        }
                    }
                }

                // Check identity
                if let Some(expected_hash) = &wiki_info.identity {
                    if let Some(file_path) = doc_paths.get("identity") {
                        if let Ok(actual_hash) = calculate_file_hash(file_path) {
                            let matches = expected_hash.to_lowercase() == actual_hash.to_lowercase();
                            validations.push(DocumentValidation {
                                document_type: "identity".to_string(),
                                file_path: file_path.clone().to_string(),
                                expected_hash: expected_hash.clone(),
                                actual_hash,
                                matches,
                            });
                        }
                    }
                }

                // Check additional documents
                for (doc_type, expected_hash) in &wiki_info.additional_documents {
                    if let Some(file_path) = doc_paths.get(doc_type) {
                        if let Ok(actual_hash) = calculate_file_hash(file_path) {
                            let matches = expected_hash.to_lowercase() == actual_hash.to_lowercase();
                            validations.push(DocumentValidation {
                                document_type: doc_type.clone(),
                                file_path: file_path.clone().to_string(),
                                expected_hash: expected_hash.clone(),
                                actual_hash,
                                matches,
                            });
                        }
                    }
                }

                combined.document_validations = validations;
            }
        }

        combined_providers.push(combined);
    }

    // Add wiki-only providers (ones that don't have API data)
    let api_names: HashSet<String> = api_data
        .node_providers
        .iter()
        .map(|p| p.display_name.to_lowercase())
        .collect();

    for (id, info) in wiki_data {
        let normalized_name = info.name.to_lowercase();
        if !api_names.contains(&normalized_name) {
            // This wiki entry has no matching API entry
            let combined = CombinedNodeProvider {
                name: info.name.clone(),
                toml_id: Some(id.clone()),
                principal_id: String::new(),
                location_count: 0,
                total_nodes: 0,
                total_rewardable_nodes: 0,
                total_subnets: 0,
                total_unassigned_nodes: 0,
                regions: Vec::new(),
                countries: Vec::new(),
                towns: Vec::new(),
                dashboard_link: String::new(),
                wiki_link: info.wiki_link.clone(),
                document_validations: Vec::new(),
                rewards: None,
            };
            combined_providers.push(combined);
        }
    }

    combined_providers
}

// Count document validation results
fn count_document_validations(combined_data: &[CombinedNodeProvider]) -> (usize, usize) {
    let mut valid_count = 0;
    let mut invalid_count = 0;

    for provider in combined_data {
        for validation in &provider.document_validations {
            if validation.matches {
                valid_count += 1;
            } else {
                invalid_count += 1;
            }
        }
    }

    (valid_count, invalid_count)
}

// Fetch rewards data from the governance canister
async fn fetch_node_provider_rewards(agent: &Agent) -> Result<ListNodeProviderRewardsResponse> {
    // Create request with no date filter to get all rewards
    let request = ListNodeProviderRewardsRequest {
        date_filter: None,
    };

    // Encode the request using Candid
    let args = Encode!(&request)?;

    // Call the governance canister
    let principal = Principal::from_text(GOVERNANCE_CANISTER_ID)?;
    let response = agent
        .query(&principal, "list_node_provider_rewards")
        .with_arg(args)
        .call()
        .await?;

    // Decode the response
    let result = Decode!(response.as_slice(), ListNodeProviderRewardsResponse)?;

    Ok(result)
}

// Process rewards data into a map keyed by principal ID
fn process_rewards_data(rewards_response: ListNodeProviderRewardsResponse) -> HashMap<String, ProviderRewardInfo> {
    let mut result = HashMap::new();

    for monthly_reward in rewards_response.rewards {
        // Get XDR conversion rate
        let xdr_rate = monthly_reward.xdr_conversion_rate
            .and_then(|rate| rate.xdr_permyriad_per_icp)
            .unwrap_or(0) as f64 / 10000.0; // Convert from permyriad to ratio

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
                    let reward_xdr = if xdr_rate > 0.0 {
                        (reward.amount_e8s as f64 / 100_000_000.0) * xdr_rate
                    } else {
                        0.0
                    };

                    // Add or update reward info in the map
                    result
                        .entry(principal_id)
                        .and_modify(|info: &mut ProviderRewardInfo| {
                            // Update total rewards
                            info.total_rewards_e8s += reward.amount_e8s;
                            info.total_rewards_xdr += reward_xdr;

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
                            total_rewards_e8s: reward.amount_e8s,
                            total_rewards_xdr: reward_xdr,
                        });
                }
            }
        }
    }

    result
}

// Process account hex to get formatted account and dashboard link
fn process_account_hex(hex: &str) -> (Option<String>, Option<String>, Option<String>) {
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

// Add rewards data to combined providers
fn add_rewards_to_providers(
    combined_data: &mut [CombinedNodeProvider],
    rewards_data: &HashMap<String, ProviderRewardInfo>
) {
    for provider in combined_data {
        if !provider.principal_id.is_empty() {
            if let Some(rewards) = rewards_data.get(&provider.principal_id) {
                provider.rewards = Some(rewards.clone());
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Read the JSON API response
    let json_data = fs::read_to_string(JSON_PATH)?;
    let api_data = parse_json(&json_data)?;

    // Read the TOML wiki data
    let toml_data = fs::read_to_string(TOML_PATH)?;
    let wiki_data = parse_toml_content(&toml_data)?;

    // Process document files
    let file_data = process_document_files(DOCS_DIR)?;
    println!("Document directories processed: {}", file_data.len());

    // Merge the data
    let mut combined_data = merge_node_provider_data(&api_data, &wiki_data, &file_data);

    // Count document validations
    let (valid_hashes, invalid_hashes) = count_document_validations(&combined_data);

    // Create an agent to communicate with the IC
    println!("Connecting to IC to fetch rewards data...");
    let agent = Agent::builder()
        .with_url(IC_URL)
        .build()?;

    // Initialize the agent (fetch root key in development)
    agent.fetch_root_key().await?;

    // Fetch rewards data
    let rewards_response = fetch_node_provider_rewards(&agent).await?;
    println!("Successfully fetched rewards data from governance canister");

    // Process rewards data
    let rewards_by_principal = process_rewards_data(rewards_response);
    println!("Processed reward data for {} principals", rewards_by_principal.len());

    // Add rewards to the combined provider data
    add_rewards_to_providers(&mut combined_data, &rewards_by_principal);

    // Print statistics
    println!("API providers: {}", api_data.node_providers.len());
    println!("Wiki providers: {}", wiki_data.len());
    println!("Combined providers: {}", combined_data.len());
    println!("Document hash validations: {} valid, {} invalid", valid_hashes, invalid_hashes);

    // Print reward statistics
    let providers_with_rewards = combined_data.iter().filter(|p| p.rewards.is_some()).count();
    println!("Providers with rewards: {}", providers_with_rewards);

    let total_rewards_xdr: f64 = rewards_by_principal.values().map(|r| r.total_rewards_xdr).sum();
    println!("Total rewards distributed: {:.2} XDR", total_rewards_xdr);

    // Find providers without wiki data
    let missing_wiki_count = combined_data.iter().filter(|p| p.toml_id.is_none()).count();
    println!("\nProviders without wiki entries: {}", missing_wiki_count);

    // Find providers with missing documents
    let missing_docs_count = combined_data
        .iter()
        .filter(|p| p.toml_id.is_some() && p.document_validations.is_empty())
        .count();
    println!("\nProviders with wiki entries but missing documents: {}", missing_docs_count);

    // Print sample of combined data
    println!("\nSample provider data:");
    for provider in combined_data.iter().take(3) {
        println!("\nProvider: {}", provider.name);
        println!("  Principal ID: {}", provider.principal_id);
        println!("  Dashboard: {}", provider.dashboard_link);
        println!("  Regions: {:?}", provider.regions);
        println!("  Countries: {:?}", provider.countries);
        println!("  Towns: {:?}", provider.towns);
        println!("  Nodes: {}/{} rewardable", provider.total_nodes, provider.total_rewardable_nodes);

        if let Some(rewards) = &provider.rewards {
            println!("  Rewards:");
            if let Some(account) = &rewards.reward_account_hex {
                println!("    Account (raw): {}", account);
                if let Some(formatted) = &rewards.reward_account_formatted {
                    println!("    Account (formatted): {}", formatted);
                }
                if let Some(link) = &rewards.reward_account_dashboard_link {
                    println!("    Dashboard Account Link: {}", link);
                }
            }
            println!("    Total rewards: {:.2} XDR ({} E8s)", rewards.total_rewards_xdr, rewards.total_rewards_e8s);
            if let Some(recent_xdr) = rewards.most_recent_reward_xdr {
                println!("    Most recent reward: {:.2} XDR", recent_xdr);
            }
        }

        if !provider.document_validations.is_empty() {
            println!("  Document Validations:");
            for validation in &provider.document_validations {
                println!(
                    "    {}: {} | Matches: {}",
                    validation.document_type,
                    &validation.expected_hash[..8],
                    validation.matches
                );
            }
        }
    }

    // Write to JSON file
    let combined_json = serde_json::to_string_pretty(&combined_data)?;
    fs::write(OUTPUT_PATH, combined_json)?;
    println!("\nCombined data written to {}", OUTPUT_PATH);

    Ok(())
}
