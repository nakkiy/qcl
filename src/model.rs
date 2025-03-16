use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Function {
    pub multi: bool,
    pub from: String,
    pub select: HashMap<String, usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Snippet {
    pub name: String,
    pub command: String,
    pub function: Option<Function>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SnippetConfig {
    pub snippets: Vec<Snippet>,
}
