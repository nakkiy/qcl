use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use tracing::{debug, error, trace};

use crate::provider::ValueProvider;
use regex::Regex;
use snippet::{Placeholder, Snippet};

/// スニペット選択とプレースホルダ解決フロー
pub fn run_qcl<P: ValueProvider>(snippets: &[Snippet], mut provider: P) -> Result<String> {
    if snippets.is_empty() {
        return Err(anyhow!("No snippets available"));
    }

    // === スニペット選択 ===
    let snippet_names: Vec<Vec<String>> = snippets.iter().map(|s| vec![s.name.clone()]).collect();

    let selected_index =
        provider.prompt_select("snippet", "Select a snippet", snippet_names, Some(0))?;

    let snippet = &snippets[selected_index];

    trace!(event = "snippet_selected", snippet_name = snippet.name);

    let resolved_command = resolve_placeholders(snippet, &mut provider)?;

    trace!(
        event = "placeholder_resolve_end",
        snippet_name = snippet.name,
        resolved_command = resolved_command
    );

    Ok(resolved_command)
}

/// プレースホルダ解決フロー
pub fn resolve_placeholders<P: ValueProvider>(
    snippet: &Snippet,
    provider: &mut P,
) -> Result<String> {
    let mut vars: HashMap<String, String> = HashMap::new();

    // === function 実行時、selectがあれば一括解決 ===
    if let Some(function) = &snippet.function {
        let records = run_external_command(&function.from)?;

        if records.is_empty() {
            return Err(anyhow!("No records from function: {}", function.from));
        }

        let display_items: Vec<Vec<String>> = records.clone();

        let index =
            provider.prompt_select("function_select", "Select a record", display_items, Some(0))?;

        let selected_row = records
            .get(index)
            .ok_or_else(|| anyhow!("Invalid selection index"))?;

        for (var_name, col_index) in &function.select {
            let value = selected_row
                .get(*col_index)
                .ok_or_else(|| anyhow!("Column {} out of bounds", col_index))?
                .to_string();

            vars.insert(var_name.clone(), value.clone());

            tracing::debug!(
                event = "function_placeholder_resolved",
                var_name = var_name,
                value = value
            );
        }
    }

    // === order 付きと無しで分ける ===
    let (mut ordered, unordered): (Vec<_>, Vec<_>) = snippet
        .placeholders
        .iter()
        .partition(|ph| ph.order.is_some());

    ordered.sort_by_key(|ph| ph.order.unwrap());

    for ph in ordered.iter().chain(unordered.iter()) {
        // 既に function で解決済みのプレースホルダはスキップ
        if vars.contains_key(&ph.name) {
            continue;
        }

        resolve_placeholder(snippet, ph, &mut vars, provider)?;
    }

    let command = render_command(&snippet.command, &vars);

    debug!(event = "command_generated", command = command);

    Ok(command)
}

/// プレースホルダ単体解決
fn resolve_placeholder<P: ValueProvider>(
    snippet: &Snippet,
    ph: &Placeholder,
    vars: &mut HashMap<String, String>,
    provider: &mut P,
) -> Result<()> {
    if vars.contains_key(&ph.name) {
        return Ok(());
    }

    trace!(
        event = "placeholder_resolve_start",
        snippet_name = snippet.name,
        placeholder_name = ph.name
    );

    let value = if let Some(from_cmd) = &ph.from {
        let records = run_external_command(from_cmd)?;

        if records.is_empty() {
            return Err(anyhow!("No records found for from: {}", from_cmd));
        }

        let index = provider.prompt_select(
            &ph.name,
            &format!("Select value for {}", &ph.name),
            records.clone(),
            Some(0),
        )?;

        let select_col = ph.select.unwrap_or(0);

        records
            .get(index)
            .and_then(|row| row.get(select_col).cloned())
            .ok_or_else(|| anyhow!("Invalid selection index or column"))?
    } else {
        provider.prompt_input(
            &ph.name,
            &format!("Enter value for {}", &ph.name),
            ph.default.clone(),
        )?
    };

    trace!(
        event = "placeholder_resolve_end",
        snippet_name = snippet.name,
        placeholder_name = ph.name,
        resolved_value = value
    );

    vars.insert(ph.name.clone(), value);

    Ok(())
}

/// function の from コマンド実行 → records 取得
fn run_external_command(cmd: &str) -> Result<Vec<Vec<String>>> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .with_context(|| format!("Failed to run external command: {}", cmd))?;

    if !output.status.success() {
        error!(
            event = "external_command_failed",
            command = cmd,
            code = ?output.status.code()
        );
        return Err(anyhow!("Command failed: {}", cmd));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    let records = stdout
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect::<Vec<Vec<String>>>();

    debug!(
        event = "external_command_executed",
        command = cmd,
        stdout_lines = stdout
    );

    Ok(records)
}

/// コマンドテンプレートのプレースホルダ置換
fn render_command(template: &str, vars: &HashMap<String, String>) -> String {
    let mut result = template.to_string();

    for (key, value) in vars {
        let re = Regex::new(&format!(r"\[\[\s*{}\s*([^\]]*)\]\]", regex::escape(key))).unwrap();

        result = re.replace_all(&result, value.as_str()).to_string();
    }

    result
}
