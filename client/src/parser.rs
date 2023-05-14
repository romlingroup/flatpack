use dockerfile_parser::Dockerfile;
use reqwest;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct Config {
    #[allow(dead_code)]
    version: String,
    #[allow(dead_code)]
    license: String,
    base_image: String,
    environment: BTreeMap<String, String>,
    port: Vec<BTreeMap<String, u16>>,
    directories: BTreeMap<String, String>,
    packages: BTreeMap<String, BTreeMap<String, String>>,
    dataset: Vec<BTreeMap<String, String>>,
    file: Vec<BTreeMap<String, String>>,
}

pub async fn parse_toml_to_dockerfile(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(format!("Failed to get file from URL: server responded with status code {}", response.status()).into());
    }

    let res = response.text().await?;
    let config: Config = toml::from_str(&res)?;

    // Start building the Dockerfile string
    let mut dockerfile = String::new();

    // Add comment indicating it was generated with flatpack.ai and the timestamp
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    dockerfile.push_str(&format!("# Generated with flatpack.ai on {}\n", timestamp));
    dockerfile.push_str("# Please do not manually modify this file\n\n");

    // Base Image
    dockerfile.push_str(&format!("FROM {}\n\n", config.base_image));

    // Environment Variables
    for (key, value) in config.environment.iter() {
        dockerfile.push_str(&format!("ENV {}={}\n", key, value));
    }

    // Directories
    dockerfile.push_str("\n# Create directories\n");
    let directories: Vec<&str> = config.directories.values().map(|v| v.as_str()).collect();
    dockerfile.push_str(&format!("RUN mkdir -p {}\n", directories.join(" ")));

    // Packages
    dockerfile.push_str("\n# Install packages\n");
    for (package_type, packages) in config.packages.iter() {
        match package_type.as_str() {
            "unix" => {
                let package_list: Vec<&str> = packages.keys().map(|k| k.as_str()).collect();
                dockerfile.push_str(&format!("RUN apt-get update && apt-get install -y {}\n", package_list.join(" ")));
            }
            "python" => {
                let package_list: Vec<&str> = packages.keys().map(|k| k.as_str()).collect();
                dockerfile.push_str(&format!("RUN pip install {}\n", package_list.join(" ")));
            }
            _ => { /* Ignore unsupported package types */ }
        }
    }

    // Ports
    for port in config.port.iter() {
        if let Some(internal) = port.get("internal") {
            dockerfile.push_str(&format!("EXPOSE {}\n", internal));
        }
    }

    // Dataset and file downloads
    dockerfile.push_str("\n# Download datasets and files\n");
    for dataset in config.dataset.iter() {
        if let (Some(from_source), Some(to_destination)) = (dataset.get("from_source"), dataset.get("to_destination")) {
            dockerfile.push_str(&format!("RUN wget {} -O {}\n", from_source, to_destination));
        }
    }
    for file in config.file.iter() {
        if let (Some(from_source), Some(to_destination)) = (file.get("from_source"), file.get("to_destination")) {
            dockerfile.push_str(&format!("RUN wget {} -O {}\n", from_source, to_destination));
        }
    }

    // Validate Dockerfile syntax
    let _ = Dockerfile::parse(&dockerfile)?;

    Ok(dockerfile)
}