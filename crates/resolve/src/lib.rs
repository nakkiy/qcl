pub mod provider;
pub mod resolver;

pub use provider::ValueProvider;
pub use resolver::{resolve_placeholders, run_qcl};
