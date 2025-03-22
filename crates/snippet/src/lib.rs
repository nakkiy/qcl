pub mod loader;
pub mod model;
pub mod parser;

pub use loader::{load_snippet_configs, load_snippets_from_file};
pub use model::{Function, Placeholder, Snippet};
pub use parser::parse_placeholders;
