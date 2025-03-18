use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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

// Combined Data Structure
#[derive(Debug, Clone, Serialize)]
struct CombinedNodeProvider {
    // Core identifying information
    name: String,            // The canonical name
    toml_id: Option<String>, // ID in the TOML file
    principal_id: String,    // Canonical ID on the IC

    // From API
    location_count: i32,
    total_nodes: i32,
    total_rewardable_nodes: i32,
    total_subnets: i32,
    total_unassigned_nodes: i32,
    website: Option<String>,

    // From Wiki
    declaration: Option<String>,
    identity: Option<String>,
    wiki_link: Option<String>,

    // Additional data
    additional_documents: HashMap<String, String>,
}

fn parse_json(content: &str) -> Result<ApiResponse, serde_json::Error> {
    serde_json::from_str(content)
}

fn parse_toml_content(content: &str) -> HashMap<String, NodeProviderWikiInfo> {
    let mut result = HashMap::new();

    // Parse the TOML content
    let parsed_toml: toml::Table = match content.parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing TOML: {}", err);
            return result;
        }
    };

    // Iterate through each section (table)
    for (section_name, section_value) in parsed_toml {
        if let toml::Value::Table(table) = section_value {
            let mut provider = NodeProviderWikiInfo {
                name: String::new(),
                declaration: None,
                identity: None,
                wiki_link: None,
                additional_documents: HashMap::new(),
            };

            // Process each key-value pair in the section
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

            result.insert(section_name, provider);
        }
    }

    result
}

fn merge_node_provider_data(
    api_data: &ApiResponse,
    wiki_data: &HashMap<String, NodeProviderWikiInfo>,
) -> Vec<CombinedNodeProvider> {
    let mut combined_providers = Vec::new();

    // Create a case-insensitive lookup map for wiki entries by name
    let mut name_to_wiki: HashMap<String, (String, &NodeProviderWikiInfo)> = HashMap::new();
    for (id, info) in wiki_data {
        name_to_wiki.insert(info.name.to_lowercase(), (id.clone(), info));
    }

    // Process each API provider
    for provider in &api_data.node_providers {
        let normalized_api_name = provider.display_name.to_lowercase();

        let mut combined = CombinedNodeProvider {
            name: provider.display_name.clone(),
            toml_id: None,
            principal_id: provider.principal_id.clone(),
            location_count: provider.location_count,
            total_nodes: provider.total_nodes,
            total_rewardable_nodes: provider.total_rewardable_nodes,
            total_subnets: provider.total_subnets,
            total_unassigned_nodes: provider.total_unassigned_nodes,
            website: provider.website.clone(),
            declaration: None,
            identity: None,
            wiki_link: None,
            additional_documents: HashMap::new(),
        };

        // Try to match with wiki data based on name
        if let Some((id, wiki_info)) = name_to_wiki.get(&normalized_api_name) {
            combined.toml_id = Some(id.clone());
            combined.declaration = wiki_info.declaration.clone();
            combined.identity = wiki_info.identity.clone();
            combined.wiki_link = wiki_info.wiki_link.clone();
            combined.additional_documents = wiki_info.additional_documents.clone();
        }

        combined_providers.push(combined);
    }

    let api_names: std::collections::HashSet<String> = api_data
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
                principal_id: String::new(), // No API data
                location_count: 0,
                total_nodes: 0,
                total_rewardable_nodes: 0,
                total_subnets: 0,
                total_unassigned_nodes: 0,
                website: None,
                declaration: info.declaration.clone(),
                identity: info.identity.clone(),
                wiki_link: info.wiki_link.clone(),
                additional_documents: info.additional_documents.clone(),
            };
            combined_providers.push(combined);
        }
    }

    combined_providers
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the JSON API response
    let json_data = include_str!("../node_providers.json");
    let api_data = parse_json(json_data)?;

    // Read the TOML wiki data
    let toml_data = include_str!("../node_providers-wiki.toml");
    let wiki_data = parse_toml_content(toml_data);

    // Merge the data
    let combined_data = merge_node_provider_data(&api_data, &wiki_data);

    // Print statistics
    println!("API providers: {}", api_data.node_providers.len());
    println!("Wiki providers: {}", wiki_data.len());
    println!("Combined providers: {}", combined_data.len());

    // Print sample of combined data
    for provider in combined_data.iter().take(5) {
        println!("\nProvider: {}", provider.name);
        println!("  Principal ID: {}", provider.principal_id);
        println!("  TOML ID: {:?}", provider.toml_id);
        println!("  Nodes: {}", provider.total_nodes);
        println!("  Wiki Link: {:?}", provider.wiki_link);
        if let Some(decl) = &provider.declaration {
            println!("  Declaration: {}", decl);
        }
    }

    // Find providers without wiki data
    let missing_wiki = combined_data
        .iter()
        .filter(|p| p.toml_id.is_none())
        .collect::<Vec<_>>();

    println!("\nProviders without wiki entries: {}", missing_wiki.len());
    for provider in missing_wiki.iter().take(5) {
        println!(
            "  {} (Principal ID: {})",
            provider.name, provider.principal_id
        );
    }

    // Option to write to a JSON file
    let combined_json = serde_json::to_string_pretty(&combined_data)?;
    std::fs::write("combined_providers.json", combined_json)?;
    println!("\nCombined data written to combined_providers.json");

    Ok(())
}
