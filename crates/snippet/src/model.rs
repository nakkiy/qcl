use std::collections::HashMap;

/// スニペットの定義
#[derive(Debug, Clone)]
pub struct Snippet {
    pub name: String,
    pub command: String,
    pub placeholders: Vec<Placeholder>,
    pub function: Option<Function>,
}

/// プレースホルダ構造
#[derive(Debug, Clone)]
pub struct Placeholder {
    pub name: String,
    pub default: Option<String>,
    pub from: Option<String>,
    pub select: Option<usize>,
    pub order: Option<usize>,
}

/// Function 定義
#[derive(Debug, Clone)]
pub struct Function {
    pub from: String,
    pub select: HashMap<String, usize>,
}
