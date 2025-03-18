use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{self, File};
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

// Document validation structure
#[derive(Debug, Clone, Serialize)]
struct DocumentValidation {
    document_type: String,
    file_path: String,
    expected_hash: String,
    actual_hash: String,
    matches: bool,
}

// Combined Data Structure with regions
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
    country: Option<String>,
    town: Option<String>,
    dashboard_link: String,

    // From Wiki
    wiki_link: Option<String>,

    // Document validations
    document_validations: Vec<DocumentValidation>,
}

// Parse JSON API data
fn parse_json(content: &str) -> Result<ApiResponse, serde_json::Error> {
    serde_json::from_str(content)
}

// Extract location info from location data
fn extract_location_info(locations: &[Location]) -> (Vec<String>, Option<String>, Option<String>) {
    let mut regions = HashSet::new();
    let mut country = None;
    let mut town = None;

    if let Some(first_location) = locations.first() {
        let parts: Vec<&str> = first_location.region.split(',').collect();

        // Extract region (first part)
        if let Some(region_part) = parts.first() {
            regions.insert(region_part.trim().to_string());
        }

        // Extract country (second part if available)
        if parts.len() > 1 {
            country = Some(parts[1].trim().to_string());
        }

        // Extract town (third part if available)
        if parts.len() > 2 {
            town = Some(parts[2].trim().to_string());
        }
    }

    // Add regions from other locations
    for location in locations.iter().skip(1) {
        if let Some(region_part) = location.region.split(',').next() {
            regions.insert(region_part.trim().to_string());
        }
    }

    (regions.into_iter().collect(), country, town)
}

// Parse TOML wiki data
fn parse_toml_content(content: &str) -> Result<HashMap<String, NodeProviderWikiInfo>, String> {
    let parsed_toml: toml::Table = match content.parse() {
        Ok(value) => value,
        Err(err) => return Err(format!("Error parsing TOML: {}", err)),
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
fn process_document_files<P: AsRef<Path>>(
    base_path: P,
) -> io::Result<HashMap<String, HashMap<String, String>>> {
    let mut result = HashMap::new();
    let allowed_docs: HashSet<&str> = [
        "declaration",
        "identity",
        "handover",
        "invoice",
        "contract",
        "passeport",
        "order",
        "registration",
        "authenticity",
        "decentralization",
    ]
    .iter()
    .copied()
    .collect();

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
fn calculate_file_hash(file_path: &str) -> Result<String, io::Error> {
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
        let (regions, country, town) = extract_location_info(&provider.locations);
        let dashboard_link = format!(
            "https://dashboard.internetcomputer.org/provider/{}",
            provider.principal_id
        );

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
            country,
            town,
            dashboard_link,
            wiki_link: None,
            document_validations: Vec::new(),
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
                            let matches =
                                expected_hash.to_lowercase() == actual_hash.to_lowercase();
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
                            let matches =
                                expected_hash.to_lowercase() == actual_hash.to_lowercase();
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
                            let matches =
                                expected_hash.to_lowercase() == actual_hash.to_lowercase();
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
                country: None,
                town: None,
                dashboard_link: String::new(),
                wiki_link: info.wiki_link.clone(),
                document_validations: Vec::new(),
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

fn main() -> Result<(), Box<dyn Error>> {
    // Configuration
    let json_path = "node_providers.json";
    let toml_path = "node_providers-wiki.toml";
    let docs_dir = "../np-list";
    let output_path = "combined_providers.json";

    // Read the JSON API response
    let json_data = fs::read_to_string(json_path)?;
    let api_data = parse_json(&json_data)?;

    // Read the TOML wiki data
    let toml_data = fs::read_to_string(toml_path)?;
    let wiki_data =
        parse_toml_content(&toml_data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Process document files
    let file_data = process_document_files(docs_dir)?;
    println!("Document directories processed: {}", file_data.len());

    // Merge the data
    let combined_data = merge_node_provider_data(&api_data, &wiki_data, &file_data);

    // Count document validations
    let (valid_hashes, invalid_hashes) = count_document_validations(&combined_data);

    // Print statistics
    println!("API providers: {}", api_data.node_providers.len());
    println!("Wiki providers: {}", wiki_data.len());
    println!("Combined providers: {}", combined_data.len());
    println!(
        "Document hash validations: {} valid, {} invalid",
        valid_hashes, invalid_hashes
    );

    // Find providers without wiki data
    let missing_wiki_count = combined_data.iter().filter(|p| p.toml_id.is_none()).count();
    println!("\nProviders without wiki entries: {}", missing_wiki_count);

    // Find providers with missing documents
    let missing_docs_count = combined_data
        .iter()
        .filter(|p| p.toml_id.is_some() && p.document_validations.is_empty())
        .count();
    println!(
        "\nProviders with wiki entries but missing documents: {}",
        missing_docs_count
    );

    // Print sample of combined data
    println!("\nSample provider data:");
    for provider in combined_data.iter().take(3) {
        println!("\nProvider: {}", provider.name);
        println!("  Principal ID: {}", provider.principal_id);
        println!("  Dashboard: {}", provider.dashboard_link);
        println!("  Regions: {:?}", provider.regions);
        if let Some(country) = &provider.country {
            println!("  Country: {}", country);
        }
        if let Some(town) = &provider.town {
            println!("  Town: {}", town);
        }
        println!(
            "  Nodes: {}/{} rewardable",
            provider.total_nodes, provider.total_rewardable_nodes
        );

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

    // List some invalid hashes for inspection
    let invalid_examples: Vec<_> = combined_data
        .iter()
        .flat_map(|p| {
            p.document_validations
                .iter()
                .filter(|v| !v.matches)
                .map(move |v| (p.name.clone(), v))
        })
        .take(5)
        .collect();

    if !invalid_examples.is_empty() {
        println!("\nSample invalid hashes:");
        for (provider_name, validation) in invalid_examples {
            println!(
                "  {} - {}: Expected {} | Actual {}",
                provider_name,
                validation.document_type,
                &validation.expected_hash[..16],
                &validation.actual_hash[..16]
            );
        }
    }

    // Write to JSON file
    let combined_json = serde_json::to_string_pretty(&combined_data)?;
    fs::write(output_path, combined_json)?;
    println!("\nCombined data written to {}", output_path);

    Ok(())
}
