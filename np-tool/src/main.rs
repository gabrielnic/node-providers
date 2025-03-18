use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use toml::{Table, Value};

/// Node Provider data from the TOML file
#[derive(Debug, Clone)]
struct NodeProviderWikiInfo {
    /// The name of the node provider
    name: String,
    /// Hash of the declaration document (optional)
    declaration: Option<String>,
    /// Identity hash (optional)
    identity: Option<String>,
    /// Link to the wiki page (optional)
    wiki_link: Option<String>,
    /// Additional document hashes keyed by document name
    additional_documents: HashMap<String, String>,
}

fn parse_toml_content(content: &str) -> HashMap<String, NodeProviderWikiInfo> {
    let mut result = HashMap::new();

    // Parse the TOML content
    let parsed_toml: Table = match content.parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing TOML: {}", err);
            return result;
        }
    };

    // Iterate through each section (table)
    for (section_name, section_value) in parsed_toml {
        if let Value::Table(table) = section_value {
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
                        if let Value::String(name) = value {
                            provider.name = name;
                        }
                    }
                    "declaration" => {
                        if let Value::String(decl) = value {
                            if !decl.is_empty() {
                                provider.declaration = Some(decl);
                            }
                        }
                    }
                    "identity" => {
                        if let Value::String(id) = value {
                            if !id.is_empty() {
                                provider.identity = Some(id);
                            }
                        }
                    }
                    "wiki-link" => {
                        if let Value::String(link) = value {
                            if !link.is_empty() {
                                provider.wiki_link = Some(link);
                            }
                        }
                    }
                    // Any other fields go into additional_documents
                    _ => {
                        if let Value::String(doc) = value {
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

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let content = include_str!("../node_providers-wiki.toml");

    // Parse the providers
    let providers = parse_toml_content(content);
    println!("Parsed {} providers", providers.len());

    // Print the parsed providers (first few as example)
    for (id, provider) in providers.iter() {
        println!("Provider ID: {}", id);
        println!("  Name: {}", provider.name);
        println!("  Declaration: {:?}", provider.declaration);
        println!("  Identity: {:?}", provider.identity);
        println!("  Wiki Link: {:?}", provider.wiki_link);
        println!(
            "  Additional Documents: {}",
            provider.additional_documents.len()
        );
        for (key, value) in &provider.additional_documents {
            println!("    {}: {}", key, value);
        }
        println!();
    }

    Ok(())
}

// Example of how to use with a file passed as argument
fn parse_from_file(path: &str) -> io::Result<HashMap<String, NodeProviderWikiInfo>> {
    let content = read_file(path)?;
    Ok(parse_toml_content(&content))
}
