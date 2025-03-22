use anyhow::Result;
use resolve::ValueProvider;

/// TUI 用 Provider 実装（未実装）
pub struct TuiProvider;

impl TuiProvider {
    pub fn new() -> Self {
        TuiProvider
    }
}

impl Default for TuiProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueProvider for TuiProvider {
    fn prompt_input(
        &mut self,
        _var_name: &str,
        _prompt: &str,
        _default: Option<String>,
    ) -> Result<String> {
        unimplemented!("TUI input is not implemented yet")
    }

    fn prompt_select(
        &mut self,
        _var_name: &str,
        _prompt: &str,
        _records: Vec<Vec<String>>,
        _default_index: Option<usize>,
    ) -> Result<usize> {
        unimplemented!("TUI select is not implemented yet")
    }
}
