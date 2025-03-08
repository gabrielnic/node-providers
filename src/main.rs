use serde_derive::Deserialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use toml;

// Define the structure for each entry
#[derive(Debug, Deserialize)]
struct Entry {
    name: String,
    path: Option<String>,
    paths: Option<String>, // For cases where "paths" is used instead of "path"
    #[serde(rename = "company-registration")]
    company_registration: Option<String>,
    #[serde(rename = "self-declaration")]
    self_declaration: Option<String>,
    #[serde(rename = "proof-of-identity")]
    proof_of_identity: Option<String>,
    handover: Option<String>,
    #[serde(rename = "node-handover")]
    node_handover: Option<String>,
}

// Calculate SHA-256 hash of a file
fn calculate_file_hash(file_path: &Path) -> Result<String, String> {
    if !file_path.exists() {
        return Err(format!("File not found: {}", file_path.display()));
    }

    let content = fs::read(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

// Process a single hash field
fn process_hash_field(
    field_name: &str,
    expected_hash: &Option<String>,
    base_path: &str,
    file_name: &str,
) {
    if let Some(expected) = expected_hash {
        let file_path = PathBuf::from(base_path).join(file_name);
        match calculate_file_hash(&file_path) {
            Ok(actual_hash) => {
                println!("{} for {}:", field_name, file_path.display());
                println!("  Expected: {}", expected);
                println!("  Actual:   {}", actual_hash);
                if expected != &actual_hash {
                    println!("  ❌ HASH MISMATCH!");
                } else {
                    println!("  ✅ Hash matches");
                }
            }
            Err(err) => println!(
                "Error calculating hash for {}: {}",
                file_path.display(),
                err
            ),
        }
    }
}

fn main() {
    // Read the config from node-providers.toml
    let config_file = "node-providers.toml";
    let input = match fs::read_to_string(config_file) {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read {}: {}", config_file, e);
            return;
        }
    };

    // Parse the TOML input
    let entries: Result<std::collections::HashMap<String, Entry>, toml::de::Error> =
        toml::from_str(&input);

    match entries {
        Ok(entries_map) => {
            for (key, entry) in entries_map.iter() {
                println!("\nProcessing entry: {} ({})", key, entry.name);

                // Determine the path (handle both "path" and "paths")
                let file_path = entry.path.as_ref().or(entry.paths.as_ref());

                if let Some(path) = file_path {
                    println!("Path: {}", path);

                    // Check for missing required fields
                    if entry.self_declaration.is_none() {
                        println!("⚠️ WARNING: Self-declaration is missing!");
                    } else {
                        process_hash_field(
                            "Self-declaration",
                            &entry.self_declaration,
                            path,
                            "declaration.pdf",
                        );
                    }

                    if entry.proof_of_identity.is_none() {
                        println!("⚠️ WARNING: Proof-of-identity is missing!");
                    } else {
                        process_hash_field(
                            "Proof-of-identity",
                            &entry.proof_of_identity,
                            path,
                            "identity.pdf",
                        );
                    }

                    // Process optional hash fields
                    process_hash_field("Handover", &entry.handover, path, "handover.pdf");
                    process_hash_field("Node-handover", &entry.node_handover, path, "handover.pdf");
                } else {
                    println!("No path specified for this entry");

                    // Check for missing required fields even if no path
                    if entry.self_declaration.is_none() {
                        println!("⚠️ WARNING: Self-declaration is missing!");
                    }
                    if entry.proof_of_identity.is_none() {
                        println!("⚠️ WARNING: Proof-of-identity is missing!");
                    }
                }
            }
        }
        Err(e) => println!("Failed to parse {}: {}", config_file, e),
    }
}
