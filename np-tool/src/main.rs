// Define structures to parse TOML file
#[derive(Debug, Deserialize)]
struct Provider {
    name: Option<String>,
    declaration: Option<String>,
    identity: Option<String>,
    handover: Option<String>,
    #[serde(rename = "wiki-link")]
    wiki_link: Option<String>,
    hardware: Option<String>,
    invoice: Option<String>,
    contract: Option<String>,
    passeport: Option<String>,
    invoice_02: Option<String>,
    registration: Option<String>,
    handover_02: Option<String>,
    decentralization: Option<String>,
    authenticity: Option<String>,
    order: Option<String>,
}

fn main() {
    let toml_content = fs::read_to_string("node-providers.toml");
    let providers: Providers = toml::from_str(&toml_content)?;
}
