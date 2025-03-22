use crate::AppContext;
use anyhow::Result;

use interface::create_cli_provider;
use resolve::run_qcl;
use snippet::load_snippet_configs;

pub fn run_cli(ctx: AppContext) -> Result<()> {
    tracing::debug!(
        event = "snippet_file_specified",
        file = ?ctx.file
    );

    // デフォルトファイル + オプションファイル読み込み
    let mut files = vec![String::from(
        &dirs::home_dir()
            .unwrap()
            .join(".config/qcl/snippets.yaml")
            .to_string_lossy()
            .to_string(),
    )];
    if let Some(custom_file) = ctx.file {
        files.push(custom_file);
    }

    let snippets = load_snippet_configs(&files)?;

    let provider = create_cli_provider();

    let command = run_qcl(&snippets, provider)?;

    tracing::info!(event = "command_generated", command = command);

    println!("{}", command);
    Ok(())
}
