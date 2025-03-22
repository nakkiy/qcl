use anyhow::Result;

/// ユーザー入力や選択肢を提供する抽象トレイト
pub trait ValueProvider {
    fn prompt_input(
        &mut self,
        var_name: &str,
        prompt: &str,
        default: Option<String>,
    ) -> Result<String>;

    fn prompt_select(
        &mut self,
        var_name: &str,
        prompt: &str,
        records: Vec<Vec<String>>,
        default_index: Option<usize>,
    ) -> Result<usize>;
}
