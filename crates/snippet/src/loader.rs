use crate::model::{Function, Snippet};
use crate::parser::parse_placeholders;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct SnippetFile {
    snippets: Vec<SnippetConfig>,
}

#[derive(Debug, Deserialize)]
struct SnippetConfig {
    name: String,
    command: String,
    function: Option<FunctionConfig>,
}

#[derive(Debug, Deserialize)]
struct FunctionConfig {
    from: String,
    select: HashMap<String, usize>,
}

pub fn load_snippets_from_file(file_path: &str) -> Result<Vec<Snippet>> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read snippet file: {}", file_path))?;
    let snippet_file: SnippetFile = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse YAML: {}", file_path))?;

    let mut snippets = Vec::new();

    for config in snippet_file.snippets {
        let placeholders = parse_placeholders(&config.command);
        let function = config.function.map(|f| Function {
            from: f.from,
            select: f.select,
        });

        snippets.push(Snippet {
            name: config.name,
            command: config.command,
            placeholders,
            function,
        });
    }

    Ok(snippets)
}

pub fn load_snippet_configs(files: &[String]) -> Result<Vec<Snippet>> {
    let mut snippet_map = std::collections::HashMap::new();
    let mut ordered_names = Vec::new();

    for file in files {
        let snippets = load_snippets_from_file(file)?;
        for snippet in snippets {
            if !snippet_map.contains_key(&snippet.name) {
                ordered_names.push(snippet.name.clone());
            }
            snippet_map.insert(snippet.name.clone(), snippet);
        }
    }

    let ordered_snippets = ordered_names
        .into_iter()
        .filter_map(|name| snippet_map.remove(&name))
        .collect();

    Ok(ordered_snippets)
}
