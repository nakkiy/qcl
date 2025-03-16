use dialoguer::{theme::SimpleTheme, Select};

pub fn select_snippet_with_dialoguer(
    snippets: &Vec<(String, String)>,
) -> Result<String, Box<dyn std::error::Error>> {
    let max_len = snippets
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0);
    let snippet_names: Vec<String> = snippets
        .iter()
        .map(|(name, _)| format!("{:<width$}", name, width = max_len))
        .collect();

    let snippet_refs: Vec<&str> = snippet_names.iter().map(|s| s.as_str()).collect();

    let theme = SimpleTheme;

    let selection = Select::with_theme(&theme)
        .with_prompt("Please select a snippet")
        .items(&snippet_refs)
        .default(0)
        .interact()?;

    Ok(snippets[selection].0.clone())
}
