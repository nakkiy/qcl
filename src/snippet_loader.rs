use crate::model::{Snippet, SnippetConfig};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load_snippets(
    override_file: Option<String>,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let configs = load_snippet_configs(override_file)?;
    let snippets = configs
        .iter()
        .map(|s| (s.name.clone(), s.command.clone()))
        .collect();
    Ok(snippets)
}

pub fn load_snippet_configs(
    override_file: Option<String>,
) -> Result<Vec<Snippet>, Box<dyn std::error::Error>> {
    let mut snippets: Vec<Snippet> = Vec::new();
    let mut seen_names: HashMap<String, usize> = HashMap::new();

    let default_path = dirs::home_dir()
        .map(|d| d.join(".config/qcl/snippets.yaml"))
        .ok_or("Failed to locate the home directory")?;

    if !default_path.exists() {
        println!(
            "Default snippet file not found.\nCreating an initial file: {:?}",
            default_path
        );
        if let Some(parent) = default_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let default_snippet = SnippetConfig {
            snippets: vec![Snippet {
                name: "example".to_string(),
                command: "echo Hello, [[name=world]]!".to_string(),
                function: None,
            }],
        };

        let yaml = serde_yaml::to_string(&default_snippet)?;
        fs::write(&default_path, yaml)?;
    }

    if default_path.exists() {
        let defaults = load_snippets_from_file(&default_path)?;
        for s in defaults {
            if !seen_names.contains_key(&s.name) {
                seen_names.insert(s.name.clone(), snippets.len());
                snippets.push(s);
            }
        }
    }

    if let Some(override_path) = override_file {
        if Path::new(&override_path).exists() {
            let overrides = load_snippets_from_file(&override_path)?;
            for s in overrides {
                if let Some(index) = seen_names.get(&s.name) {
                    snippets[*index] = s;
                } else {
                    seen_names.insert(s.name.clone(), snippets.len());
                    snippets.push(s);
                }
            }
        } else {
            eprintln!("Specified file does not exist: {}", override_path);
            std::process::exit(1);
        }
    }

    Ok(snippets)
}

pub fn load_snippets_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<Snippet>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let config: SnippetConfig = serde_yaml::from_str(&data)?;
    Ok(config.snippets)
}

pub fn load_snippet_object(
    name: &str,
    override_file: Option<String>,
) -> Result<crate::model::Snippet, Box<dyn std::error::Error>> {
    let configs = load_snippet_configs(override_file)?;
    configs
        .into_iter()
        .find(|s| s.name == name)
        .ok_or_else(|| "Snippet not found".into())
}
