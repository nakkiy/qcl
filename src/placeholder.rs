use crate::model::{Function, Snippet};
use dialoguer::{theme::SimpleTheme, Input, Select};
use std::collections::HashMap;
use std::process::Command;

pub fn resolve_placeholders(snippet: Snippet) -> Result<String, Box<dyn std::error::Error>> {
    let mut vars: HashMap<String, String> = HashMap::new();

    if let Some(func) = &snippet.function {
        if func.multi {
            process_function_multi(func, &mut vars)?;
        }
    }

    let mut resolved_command = snippet.command.clone();

    loop {
        let mut updated = false;
        let mut chars = resolved_command.chars().peekable();
        let mut new_command = String::new();

        while let Some(c) = chars.next() {
            if c == '[' && chars.peek() == Some(&'[') {
                chars.next();
                let mut placeholder = String::new();

                while let Some(next_c) = chars.next() {
                    if next_c == ']' && chars.peek() == Some(&']') {
                        chars.next();
                        break;
                    } else {
                        placeholder.push(next_c);
                    }
                }

                let (name, from_command, select_index, default_value, _order) =
                    parse_placeholder(&placeholder);

                if let Some(val) = vars.get(&name) {
                    new_command.push_str(val);
                } else if let Some(cmd) = from_command {
                    let output = Command::new("sh").arg("-c").arg(&cmd).output()?;
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let choices: Vec<String> =
                        stdout.lines().map(|line| line.to_string()).collect();

                    if choices.is_empty() {
                        return Err("No selectable options found.".into());
                    }

                    let selection = Select::with_theme(&SimpleTheme)
                        .with_prompt(format!("Please select {}", name))
                        .items(&choices)
                        .default(0)
                        .interact()?;

                    let selected = if let Some(index) = select_index {
                        let fields: Vec<&str> = choices[selection].split_whitespace().collect();
                        fields.get(index).unwrap_or(&"").to_string()
                    } else {
                        choices[selection].clone()
                    };

                    vars.insert(name.clone(), selected.clone());
                    new_command.push_str(&selected);
                } else {
                    let input = Input::<String>::new()
                        .with_prompt(format!("Please enter {}", name))
                        .default(default_value.unwrap_or_default())
                        .interact_text()?;
                    vars.insert(name.clone(), input.clone());
                    new_command.push_str(&input);
                }

                updated = true;
            } else {
                new_command.push(c);
            }
        }

        resolved_command = new_command;

        if !updated {
            break;
        }
    }

    Ok(resolved_command)
}

fn process_function_multi(
    function: &Function,
    vars: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("sh").arg("-c").arg(&function.from).output()?;

    if !output.status.success() {
        return Err(format!("Function command failed: {}", function.from).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let choices: Vec<String> = stdout.lines().map(|line| line.to_string()).collect();

    if choices.is_empty() {
        return Err("No results from function command".into());
    }

    let selection = Select::with_theme(&SimpleTheme)
        .with_prompt("Please select an option")
        .items(&choices)
        .default(0)
        .interact()?;

    let selected_line = &choices[selection];
    let fields: Vec<&str> = selected_line.split_whitespace().collect();

    for (key, index) in &function.select {
        let val = fields
            .get(*index)
            .ok_or(format!("Index {} not found for key {}", index, key))?
            .to_string();
        vars.insert(key.clone(), val);
    }

    Ok(())
}

fn parse_placeholder(
    placeholder: &str,
) -> (
    String,
    Option<String>,
    Option<usize>,
    Option<String>,
    Option<usize>,
) {
    let name;
    let mut from_command: Option<String> = None;
    let mut select_index: Option<usize> = None;
    let mut default_value: Option<String> = None;
    let mut order: Option<usize> = None;

    let mut parts = placeholder.trim().splitn(2, ' ');
    let first_part = parts.next().unwrap();

    if let Some(eq_pos) = first_part.find('=') {
        name = first_part[..eq_pos].to_string();
        default_value = Some(first_part[eq_pos + 1..].to_string());
    } else {
        name = first_part.to_string();
    }

    if let Some(args_str) = parts.next() {
        let mut rest = args_str.trim();

        while !rest.is_empty() {
            if let Some(from_pos) = rest.find("from:\"") {
                rest = &rest[from_pos + 6..];
                if let Some(end_quote) = rest.find('"') {
                    let cmd = &rest[..end_quote];
                    from_command = Some(cmd.to_string());
                    rest = &rest[end_quote + 1..];
                } else {
                    break;
                }
            } else if let Some(select_pos) = rest.find("select:") {
                rest = &rest[select_pos + 7..];
                let mut split = rest.splitn(2, char::is_whitespace);
                if let Some(idx_str) = split.next() {
                    select_index = idx_str.parse::<usize>().ok();
                }
                rest = split.next().unwrap_or("").trim();
            } else if let Some(order_pos) = rest.find("order:") {
                rest = &rest[order_pos + 6..];
                let mut split = rest.splitn(2, char::is_whitespace);
                if let Some(order_str) = split.next() {
                    order = order_str.parse::<usize>().ok();
                }
                rest = split.next().unwrap_or("").trim();
            } else {
                break;
            }
        }
    }

    (name, from_command, select_index, default_value, order)
}
