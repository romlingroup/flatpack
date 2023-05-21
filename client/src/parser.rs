use dockerfile_parser::Dockerfile;
use reqwest;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Deserialize)]
pub struct Config {
    base_image: String,
    dataset: Vec<BTreeMap<String, String>>,
    directories: BTreeMap<String, String>,
    environment: BTreeMap<String, String>,
    file: Vec<BTreeMap<String, String>>,
    git: Vec<BTreeMap<String, String>>,
    #[allow(dead_code)]
    license: String,
    packages: BTreeMap<String, BTreeMap<String, String>>,
    port: Vec<BTreeMap<String, u16>>,
    script: Vec<BTreeMap<String, String>>,
    #[allow(dead_code)]
    version: String,
}

pub async fn parse_toml_to_dockerfile(url: &str) -> Result<String, Box<dyn Error>> {
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
    dockerfile.push_str(&format!("# This file was automatically @generated by flatpack.ai on {}.\n", timestamp));
    dockerfile.push_str("# It is not intended for manual editing.\n\n");

    // Base image
    dockerfile.push_str(&format!("FROM {}\n\n", config.base_image));

    // Environment variables
    for (key, value) in config.environment.iter() {
        dockerfile.push_str(&format!("ENV {}={}\n", key, value));
    }

    // Create directories
    dockerfile.push_str("\n# Create directories\n");
    let directories: Vec<&str> = config.directories.values().map(|v| v.as_str()).collect();
    dockerfile.push_str(&format!("RUN mkdir -p {}\n", directories.join(" ")));

    // Install packages
    dockerfile.push_str("\n# Install packages\n");

    if let Some(unix_packages) = config.packages.get("unix") {
        let package_list: Vec<&str> = unix_packages.keys().map(|k| k.as_str()).collect();
        dockerfile.push_str(&format!("RUN apt-get update && apt-get install -y {}\n", package_list.join(" ")));
    }

    if let Some(python_packages) = config.packages.get("python") {
        let package_list: Vec<&str> = python_packages.keys().map(|k| k.as_str()).collect();
        dockerfile.push_str(&format!("RUN pip install {}\n", package_list.join(" ")));
    }

    // Ports
    dockerfile.push_str("\n# Expose ports\n");
    for port in config.port.iter() {
        if let Some(internal) = port.get("internal") {
            dockerfile.push_str(&format!("EXPOSE {}\n", internal));
        }
        // Note: Dockerfiles can't directly handle external ports. They need to be handled at runtime.
        if let Some(external) = port.get("external") {
            eprintln!("Info: External port {} specified. This needs to be mapped at runtime, e.g. with 'docker run -p {}:...'.", external, external);
        }
    }

    // Dataset and file downloads
    dockerfile.push_str("\n# Download datasets and files\n");
    for dataset in config.dataset.iter() {
        if let (Some(from_source), Some(to_destination)) = (dataset.get("from_source"), dataset.get("to_destination")) {
            dockerfile.push_str(&format!("RUN wget {} -O {}\n", from_source, to_destination));
        } else {
            eprintln!("Warning: Invalid dataset entry. It should include both 'from_source' and 'to_destination'.");
        }
    }
    for file in config.file.iter() {
        if let (Some(from_source), Some(to_destination)) = (file.get("from_source"), file.get("to_destination")) {
            dockerfile.push_str(&format!("RUN wget {} -O {}\n", from_source, to_destination));
        } else {
            eprintln!("Warning: Invalid file entry. It should include both 'from_source' and 'to_destination'.");
        }
    }

    // Git repositories
    dockerfile.push_str("\n# Clone git repositories\n");
    for git in config.git.iter() {
        if let (Some(from_source), Some(to_destination)) = (git.get("from_source"), git.get("to_destination")) {
            dockerfile.push_str(&format!("RUN git clone {} {}\n", from_source, to_destination));
        } else {
            eprintln!("Warning: Invalid git entry. It should include both 'from_source' and 'to_destination'.");
        }
    }

    // Scripts
    dockerfile.push_str("\n# Scripts\n");
    for script in config.script.iter() {
        if let (Some(command), Some(file)) = (script.get("command"), script.get("file")) {
            dockerfile.push_str(&format!("RUN {} {}\n", command, file));
        } else {
            eprintln!("Warning: Invalid script entry. It should include both 'command' and 'file'.");
        }
    }
    // Note: Only the last script will be used as CMD.
    if let Some(last_script) = config.script.last() {
        if let (Some(command), Some(file)) = (last_script.get("command"), last_script.get("file")) {
            dockerfile.push_str(&format!("CMD [\"{}\", \"{}\"]\n", command, file));
        } else {
            eprintln!("Warning: Invalid script entry. It should include both 'command' and 'file'.");
        }
    }

    // Validate Dockerfile syntax
    match Dockerfile::parse(&dockerfile) {
        Ok(_) => Ok(dockerfile),
        Err(e) => Err(format!("Error parsing Dockerfile: {}", e).into())
    }
}