use anyhow::Result;
use dialoguer::{Input, Select};
use resolve::ValueProvider;
use tracing::trace;

/// CLI 用 Provider 実装
pub struct DialoguerProvider;

impl DialoguerProvider {
    pub fn new() -> Self {
        DialoguerProvider
    }
}

impl Default for DialoguerProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueProvider for DialoguerProvider {
    fn prompt_input(
        &mut self,
        var_name: &str,
        prompt: &str,
        default: Option<String>,
    ) -> Result<String> {
        trace!(
            event = "user_input_start",
            var_name = var_name,
            prompt = prompt
        );

        let input: String = Input::new()
            .with_prompt(prompt)
            .default(default.unwrap_or_default())
            .interact_text()?;

        trace!(
            event = "user_input_end",
            var_name = var_name,
            input_value = input
        );

        Ok(input)
    }

    fn prompt_select(
        &mut self,
        var_name: &str,
        prompt: &str,
        records: Vec<Vec<String>>,
        default_index: Option<usize>,
    ) -> Result<usize> {
        trace!(
            event = "user_select_start",
            var_name = var_name,
            prompt = prompt
        );

        // 表示用のリスト作成
        let items: Vec<String> = records.iter().map(|record| record.join(" | ")).collect();

        let selection = Select::new()
            .with_prompt(prompt)
            .items(&items)
            .default(default_index.unwrap_or(0))
            .interact()?;

        trace!(
            event = "user_select_end",
            var_name = var_name,
            selected_index = selection
        );

        Ok(selection)
    }
}
