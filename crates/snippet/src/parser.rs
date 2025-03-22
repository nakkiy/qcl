use crate::model::Placeholder;
use regex::Regex;

pub fn parse_placeholders(command: &str) -> Vec<Placeholder> {
    let re = Regex::new(r"\[\[(?P<content>[^\]]+)\]\]").unwrap();
    let mut placeholders = Vec::new();

    for caps in re.captures_iter(command) {
        let content = &caps["content"];
        let parts: Vec<&str> = content.split_whitespace().collect();

        let mut name = String::new();
        let mut default = None;
        let mut from = None;
        let mut select = None;
        let mut order = None;

        for part in parts {
            if part.contains('=') && name.is_empty() {
                let pair: Vec<&str> = part.splitn(2, '=').collect();
                name = pair[0].to_string();
                default = Some(pair[1].to_string());
            } else if let Some(stripped) = part.strip_prefix("from:") {
                from = Some(stripped.trim_matches('"').to_string());
            } else if let Some(stripped) = part.strip_prefix("select:") {
                select = stripped.parse::<usize>().ok();
            } else if let Some(stripped) = part.strip_prefix("order:") {
                order = stripped.parse::<usize>().ok();
            } else if name.is_empty() {
                name = part.to_string();
            }
        }

        placeholders.push(Placeholder {
            name,
            default,
            from,
            select,
            order,
        });
    }

    placeholders
}
