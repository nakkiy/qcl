mod model;
mod placeholder;
mod snippet_loader;
mod ui;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short = 'f', long = "file")]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let result = run_qcl(cli);

    if let Ok(final_cmd) = result {
        println!("{}", final_cmd);
        std::process::exit(0);
    } else {
        eprintln!("An error has occurred.");
        std::process::exit(1);
    }
}

fn run_qcl(cli: Cli) -> Result<String, Box<dyn std::error::Error>> {
    let snippets = snippet_loader::load_snippets(cli.file.clone())?;
    let selected = ui::select_snippet_with_dialoguer(&snippets)?;
    let snippet_obj = snippet_loader::load_snippet_object(&selected, cli.file)?;
    let final_cmd = placeholder::resolve_placeholders(snippet_obj)?;
    Ok(final_cmd)
}
