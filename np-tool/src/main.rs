use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};

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

    // Document file paths
    document_paths: HashMap<String, String>,

    // Document validations
    document_validations: Vec<DocumentValidation>,
}

// Parse JSON API data
fn parse_json(content: &str) -> Result<ApiResponse, serde_json::Error> {
    serde_json::from_str(content)
}

// Parse TOML wiki data
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

// Process document files in the specified directory
fn process_document_files(base_path: &str) -> HashMap<String, HashMap<String, String>> {
    let mut result = HashMap::new();

    // Define allowed document types
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

    // Read directory entries
    match fs::read_dir(base_path) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    continue;
                }

                let toml_id = entry.file_name().to_string_lossy().to_string();
                let provider_dir = entry.path();
                let mut doc_paths = HashMap::new();

                // Process each file in the provider directory
                if let Ok(files) = fs::read_dir(&provider_dir) {
                    for file in files.filter_map(Result::ok) {
                        if !file.file_type().map(|t| t.is_file()).unwrap_or(false) {
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
                }

                if !doc_paths.is_empty() {
                    result.insert(toml_id, doc_paths);
                }
            }
        }
        Err(e) => eprintln!("Error reading directory {}: {}", base_path, e),
    }

    result
}

// Calculate SHA-256 hash of a file
fn calculate_file_hash(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Use a different crate for SHA-256
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
            document_paths: HashMap::new(),
            document_validations: Vec::new(),
        };

        // Try to match with wiki data based on name
        if let Some((id, wiki_info)) = name_to_wiki.get(&normalized_api_name) {
            combined.toml_id = Some(id.clone());
            combined.declaration = wiki_info.declaration.clone();
            combined.identity = wiki_info.identity.clone();
            combined.wiki_link = wiki_info.wiki_link.clone();
            combined.additional_documents = wiki_info.additional_documents.clone();

            // Add document file paths if available
            if let Some(doc_paths) = file_data.get(id) {
                combined.document_paths = doc_paths.clone();
            }
        }

        combined_providers.push(combined);
    }

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
                website: None,
                declaration: info.declaration.clone(),
                identity: info.identity.clone(),
                wiki_link: info.wiki_link.clone(),
                additional_documents: info.additional_documents.clone(),
                document_paths: file_data.get(id).cloned().unwrap_or_default(),
                document_validations: Vec::new(),
            };
            combined_providers.push(combined);
        }
    }

    combined_providers
}

// Validate document hashes against actual files
fn validate_document_hashes(combined_data: &mut [CombinedNodeProvider]) {
    for provider in combined_data {
        // Skip providers without wiki data or document paths
        if provider.toml_id.is_none() || provider.document_paths.is_empty() {
            continue;
        }

        let mut validations = Vec::new();

        // Check declaration hash if available
        if let (Some(expected_hash), Some(file_path)) = (
            &provider.declaration,
            provider.document_paths.get("declaration"),
        ) {
            match calculate_file_hash(file_path) {
                Ok(actual_hash) => {
                    let expected = expected_hash.to_lowercase();
                    let actual = actual_hash.clone();
                    validations.push(DocumentValidation {
                        document_type: "declaration".to_string(),
                        file_path: file_path.clone(),
                        expected_hash: expected_hash.clone(),
                        actual_hash,
                        matches: expected == actual.to_lowercase(),
                    });
                }
                Err(e) => {
                    eprintln!("Error calculating hash for {}: {}", file_path, e);
                }
            }
        }

        // Check identity hash if available
        if let (Some(expected_hash), Some(file_path)) =
            (&provider.identity, provider.document_paths.get("identity"))
        {
            match calculate_file_hash(file_path) {
                Ok(actual_hash) => {
                    let expected = expected_hash.to_lowercase();
                    let actual = actual_hash.clone();
                    validations.push(DocumentValidation {
                        document_type: "identity".to_string(),
                        file_path: file_path.clone(),
                        expected_hash: expected_hash.clone(),
                        actual_hash,
                        matches: expected == actual.to_lowercase(),
                    });
                }
                Err(e) => {
                    eprintln!("Error calculating hash for {}: {}", file_path, e);
                }
            }
        }

        // Check other document hashes as needed
        for (doc_type, expected_hash) in &provider.additional_documents {
            if let Some(file_path) = provider.document_paths.get(doc_type) {
                match calculate_file_hash(file_path) {
                    Ok(actual_hash) => {
                        let expected = expected_hash.to_lowercase();
                        let actual = actual_hash.clone();
                        validations.push(DocumentValidation {
                            document_type: doc_type.clone(),
                            file_path: file_path.clone(),
                            expected_hash: expected_hash.clone(),
                            actual_hash,
                            matches: expected == actual.to_lowercase(),
                        });
                    }
                    Err(e) => {
                        eprintln!("Error calculating hash for {}: {}", file_path, e);
                    }
                }
            }
        }

        provider.document_validations = validations;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the JSON API response
    let json_data = include_str!("../node_providers.json");
    let api_data = parse_json(json_data)?;

    // Read the TOML wiki data
    let toml_data = include_str!("../node_providers-wiki.toml");
    let wiki_data = parse_toml_content(toml_data);

    // Process document files
    let file_data = process_document_files("../np-list");
    println!("Document directories processed: {}", file_data.len());

    // Merge the data
    let mut combined_data = merge_node_provider_data(&api_data, &wiki_data, &file_data);

    // Validate document hashes
    validate_document_hashes(&mut combined_data);

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
        if !provider.document_paths.is_empty() {
            println!("  Document Files:");
            for (doc_type, path) in &provider.document_paths {
                println!("    {}: {}", doc_type, path);
            }
        }
        if !provider.document_validations.is_empty() {
            println!("  Document Validations:");
            for validation in &provider.document_validations {
                println!(
                    "    {}: Expected {} | Actual {} | Matches: {}",
                    validation.document_type,
                    &validation.expected_hash[..8], // Show just first 8 chars
                    &validation.actual_hash[..8],
                    validation.matches
                );
            }
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

    // Find providers without document files
    let missing_docs = combined_data
        .iter()
        .filter(|p| p.toml_id.is_some() && p.document_paths.is_empty())
        .collect::<Vec<_>>();

    println!(
        "\nProviders with wiki entries but missing document files: {}",
        missing_docs.len()
    );
    for provider in missing_docs.iter().take(5) {
        if let Some(id) = &provider.toml_id {
            println!("  {} (TOML ID: {})", provider.name, id);
        }
    }

    // Count hash validation results
    let valid_hashes = combined_data
        .iter()
        .flat_map(|p| &p.document_validations)
        .filter(|v| v.matches)
        .count();

    let invalid_hashes = combined_data
        .iter()
        .flat_map(|p| &p.document_validations)
        .filter(|v| !v.matches)
        .count();

    println!(
        "\nDocument hash validations: {} valid, {} invalid",
        valid_hashes, invalid_hashes
    );

    // List some invalid hashes for inspection
    let invalid_examples: Vec<_> = combined_data
        .iter()
        .flat_map(|p| {
            p.document_validations
                .iter()
                .map(move |v| (p.name.clone(), v))
        })
        .filter(|(_, v)| !v.matches)
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
    fs::write("combined_providers.json", combined_json)?;
    println!("\nCombined data written to combined_providers.json");

    Ok(())
}
