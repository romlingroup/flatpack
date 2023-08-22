use dockerfile_parser::Dockerfile;
use reqwest;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Deserialize)]
pub struct Config {
    base_image: String,
    cmd: Vec<BTreeMap<String, String>>,
    dataset: Option<Vec<BTreeMap<String, String>>>,
    directories: Option<BTreeMap<String, String>>,
    environment: BTreeMap<String, String>,
    file: Option<Vec<BTreeMap<String, String>>>,
    git: Vec<BTreeMap<String, String>>,
    packages: Option<BTreeMap<String, BTreeMap<String, String>>>,
    port: Vec<BTreeMap<String, u16>>,
    run: Option<Vec<BTreeMap<String, String>>>,
    #[allow(dead_code)]
    version: String,
}

// BEGIN Bash
pub async fn parse_toml_to_pyenv_script(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(format!("Failed to get file from URL: server responded with status code {}", response.status()).into());
    }
    let res = response.text().await?;
    let config: Config = toml::from_str(&res)?;
    let model_name = config.environment.get("model_name").ok_or("Missing model_name in flatpack.toml")?;
    let mut script = String::new();

    script.push_str("#!/bin/bash\n");

    script.push_str("if [[ \"${COLAB_GPU}\" == \"1\" ]]; then\n");
    script.push_str("  echo \"Running in Google Colab environment\"\n");
    script.push_str("  IS_COLAB=1\n");
    script.push_str("else\n");
    script.push_str("  echo \"Not running in Google Colab environment\"\n");
    script.push_str("  IS_COLAB=0\n");
    script.push_str("fi\n");

    script.push_str("if [[ $IS_COLAB -eq 0 ]]; then\n");

    script.push_str(" if ! command -v pyenv >/dev/null; then\n");
    script.push_str("   echo \"pyenv not found. Please install pyenv.\"\n");
    script.push_str("   exit 1\n");
    script.push_str(" fi\n");

    script.push_str(" if ! command -v wget >/dev/null; then\n");
    script.push_str("   echo \"wget not found. Please install wget.\"\n");
    script.push_str("   exit 1\n");
    script.push_str(" fi\n");

    script.push_str(" if ! command -v git >/dev/null; then\n");
    script.push_str("   echo \"git not found. Please install git.\"\n");
    script.push_str("   exit 1\n");
    script.push_str(" fi\n");

    script.push_str(" export PYENV_ROOT=\"$HOME/.pyenv\"\n");
    script.push_str(" export PATH=\"$PYENV_ROOT/bin:$PATH\"\n");
    script.push_str(" if command -v pyenv 1>/dev/null 2>&1; then\n");
    script.push_str("   eval \"$(pyenv init -)\"\n");
    script.push_str("   eval \"$(pyenv virtualenv-init -)\"\n");
    script.push_str(" fi\n");

    script.push_str("fi\n");

    // Create a new project directory
    script.push_str(&format!("mkdir -p ./{}\n", model_name));

    // Create directories
    if let Some(directories_map) = &config.directories {
        for (_directory_name, directory_path) in directories_map {
            let formatted_directory_path = directory_path.trim_start_matches('/');
            let without_home_content = formatted_directory_path.trim_start_matches("home/content/");
            script.push_str(&format!("mkdir -p ./{}/{}\n", model_name, without_home_content));
        }
    } else {
        script.push_str("# Found no directories, proceeding without it.\n");
    }

    // Set environment variables
    for (key, value) in &config.environment {
        script.push_str(&format!("export {}={}\n", key, value.replace("/home/content/", &format!("./{}/", model_name))));
    }

    // Create a new pyenv environment and activate it
    let version = "3.11.3";
    let env_name = "myenv";
    script.push_str(" if [[ $IS_COLAB -eq 0 ]]; then\n");
    script.push_str(&format!(" if ! pyenv versions | grep -q {0}; then\n  pyenv install {0}\nfi\n", version));
    script.push_str(&format!(" if ! pyenv virtualenvs | grep -q {0}; then\n  pyenv virtualenv {1} {0}\nfi\n", env_name, version));
    script.push_str(&format!(" pyenv activate {}\n", env_name));
    script.push_str("fi\n");

    // Install Python packages
    if let Some(packages) = &config.packages {
        if let Some(python_packages) = packages.get("python") {
            let package_list: Vec<String> = python_packages
                .iter()
                .map(|(package, version)| {
                    if version == "*" || version.is_empty() {
                        format!("{}", package)  // If version is not specified or "*", get the latest version
                    } else {
                        format!("{}=={}", package, version)  // If version is specified, get that version
                    }
                })
                .collect();
            script.push_str(&format!(
                "python -m pip install {}\n",
                package_list.join(" ")
            ));
        }
    }

    // Git repositories
    for git in &config.git {
        if let (Some(from_source), Some(to_destination), Some(branch)) = (git.get("from_source"), git.get("to_destination"), git.get("branch")) {
            let repo_path = format!("./{}/{}", model_name, to_destination.replace("/home/content/", ""));
            script.push_str(&format!("echo 'Cloning repository from: {}'\n", from_source));
            script.push_str(&format!("git clone -b {} {} {}\n", branch, from_source, repo_path));
            script.push_str(&format!("if [ -f {}/requirements.txt ]; then\n  echo 'Found requirements.txt, installing dependencies...'\n  cd {} || exit\n  python -m pip install -r requirements.txt\n  cd - || exit\nelse\n  echo 'No requirements.txt found.'\nfi\n", repo_path, repo_path));
        }
    }

    // Download datasets and files
    if let Some(dataset_vec) = &config.dataset {
        for dataset in dataset_vec {
            if let (Some(from_source), Some(to_destination)) = (dataset.get("from_source"), dataset.get("to_destination")) {
                script.push_str(&format!("wget {} -O ./{}/{}\n", from_source, model_name, to_destination.replace("/home/content/", "")));
            }
        }
    } else {
        script.push_str("# Found no datasets, proceeding without them.\n");
    }

    // Download files
    if let Some(file_vec) = &config.file {
        for file in file_vec.iter() {
            if let (Some(from_source), Some(to_destination)) = (file.get("from_source"), file.get("to_destination")) {
                script.push_str(&format!("wget {} -O ./{}/{}\n", from_source, model_name, to_destination.replace("/home/content/", "")));
            }
        }
    } else {
        script.push_str("# Found no files, proceeding without them.\n");
    }

    // RUN commands
    if let Some(run_vec) = &config.run {
        for run in run_vec {
            if let (Some(command), Some(args)) = (run.get("command"), run.get("args")) {
                // replace "/home/content/" with "./{model_name}/"
                let replaced_args = args.replace("/home/content/", &format!("./{}/", model_name));
                script.push_str(&format!("{} {}\n", command, replaced_args));
            }
        }
    } else {
        script.push_str("# Found no run commands, proceeding without them.\n");
    }

    Ok(script)
}
// END Bash

// BEGIN Containerfile
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
    dockerfile.push_str(&format!("FROM {}\n", config.base_image));

    // Create directories
    dockerfile.push_str("\n# Create directories\n");
    if let Some(directories_map) = &config.directories {
        for (_directory_name, directory_path) in directories_map {
            let directories: Vec<&str> = directory_path.split_whitespace().collect();
            dockerfile.push_str(&format!("RUN mkdir -p {}\n", directories.join(" ")));
        }
    } else {
        dockerfile.push_str("# Found no directories, proceeding without it.\n");
    }

    // Environment variables
    for (key, value) in config.environment.iter() {
        dockerfile.push_str(&format!("ENV {}={}\n", key, value));
    }

    // Install packages
    dockerfile.push_str("\n# Install packages\n");

    // Update package list
    dockerfile.push_str("RUN apt-get update && apt-get upgrade -y");

    if let Some(packages) = &config.packages {
        if let Some(unix_packages) = packages.get("unix") {
            let package_list: Vec<String> = unix_packages
                .iter()
                .map(|(package, version)| {
                    if version == "*" || version.is_empty() {
                        format!("{}", package)  // If version is not specified or "*", get the latest version
                    } else {
                        format!("{}={}", package, version)  // If version is specified, get that version
                    }
                })
                .collect();
            dockerfile.push_str(&format!(" && apt-get install -y {}", package_list.join(" ")));
        }

        // Remove unnecessary packages and clear apt cache
        dockerfile.push_str(" && apt-get autoremove -y && apt-get clean && rm -rf /var/lib/apt/lists/*");

        if let Some(python_packages) = packages.get("python") {
            let package_list: Vec<String> = python_packages
                .iter()
                .map(|(package, version)| {
                    if version == "*" || version.is_empty() {
                        format!("{}", package)  // If version is not specified or "*", get the latest version
                    } else {
                        format!("{}=={}", package, version)  // If version is specified, get that version
                    }
                })
                .collect();
            dockerfile.push_str(&format!(
                " && pip install {}",
                package_list.join(" ")
            ));
        }
    } else {
        dockerfile.push_str("# Found no packages, proceeding without them.\n");
    }

    // Add a newline before EXPOSE ports
    dockerfile.push_str("\n");

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

    // Git repositories
    dockerfile.push_str("\n# Clone git repositories\n");
    for git in config.git.iter() {
        if let (Some(from_source), Some(to_destination), Some(branch)) = (git.get("from_source"), git.get("to_destination"), git.get("branch")) {
            dockerfile.push_str(&format!("RUN git clone -b {} {} {}\n", branch, from_source, to_destination));
        } else {
            eprintln!("Warning: Invalid git entry. It should include both 'from_source' and 'to_destination'.");
        }
    }

    // Download datasets and files
    dockerfile.push_str("\n# Download datasets and files\n");

    if let Some(dataset_vec) = &config.dataset {
        for dataset in dataset_vec.iter() {
            if let (Some(from_source), Some(to_destination)) = (dataset.get("from_source"), dataset.get("to_destination")) {
                dockerfile.push_str(&format!("RUN wget {} -O {}\n", from_source, to_destination));
            } else {
                eprintln!("Warning: Invalid dataset entry. It should include both 'from_source' and 'to_destination'.");
            }
        }
    } else {
        dockerfile.push_str("# Found no datasets, proceeding without them.\n");
    }

    if let Some(file_vec) = &config.file {
        for file in file_vec.iter() {
            if let (Some(from_source), Some(to_destination)) = (file.get("from_source"), file.get("to_destination")) {
                dockerfile.push_str(&format!("RUN wget {} -O {}\n", from_source, to_destination));
            } else {
                eprintln!("Warning: Invalid file entry. It should include both 'from_source' and 'to_destination'.");
            }
        }
    } else {
        dockerfile.push_str("# Found no files, proceeding without them.\n");
    }

    // RUN commands
    dockerfile.push_str("\n# RUN commands\n");
    if let Some(run_vec) = &config.run {
        for run in run_vec.iter() {
            if let (Some(command), Some(args)) = (run.get("command"), run.get("args")) {
                dockerfile.push_str(&format!("RUN {} {}\n", command, args));
            } else {
                eprintln!("Warning: Invalid run entry. It should include both 'command' and 'args'.");
            }
        }
    } else {
        dockerfile.push_str("# Found no run commands, proceeding without them.\n");
    }

    // CMD command
    dockerfile.push_str("\n# CMD command\n");
    if config.cmd.len() != 1 {
        return Err("Invalid number of CMD entries. There should be exactly one CMD entry.".into());
    }
    for cmd in config.cmd.iter() {
        if let (Some(command), Some(args)) = (cmd.get("command"), cmd.get("args")) {
            // We need to handle command and args separately
            dockerfile.push_str("CMD [");
            dockerfile.push_str("\"");
            dockerfile.push_str(command);
            dockerfile.push_str("\", ");
            // Splitting the args into separate strings
            let cmd_args: Vec<&str> = args.split(' ').collect();
            for (i, arg) in cmd_args.iter().enumerate() {
                dockerfile.push_str("\"");
                dockerfile.push_str(arg);
                dockerfile.push_str("\"");
                if i != cmd_args.len() - 1 {
                    dockerfile.push_str(", ");
                }
            }
            dockerfile.push_str("]\n");
        } else {
            return Err("Invalid CMD entry. It should include both 'command' and 'args'.".into());
        }
    }

    // Validate Containerfile syntax
    match Dockerfile::parse(&dockerfile) {
        Ok(_) => {
            // Add feedback message when the build is complete
            dockerfile.push_str("\n# Build complete! 🎉\n");
            Ok(dockerfile)
        }
        Err(e) => Err(format!("Error parsing Containerfile: {}", e).into())
    }
}
// END Containerfile