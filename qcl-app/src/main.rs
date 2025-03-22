mod cli;
mod logger;
mod tui;

use anyhow::Result;
use clap::{Parser, Subcommand};

pub struct AppContext {
    pub file: Option<String>,
    pub log_level: String,
    pub log_file: Option<String>,
}

#[derive(Parser)]
#[command(name = "qcl", version, about = "QCL CLI / TUI")]
struct AppArgs {
    #[arg(short, long)]
    file: Option<String>,

    #[arg(long, default_value = "info")]
    log_level: String,

    #[arg(long)]
    log_file: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Cli,
    Tui,
}

fn main() -> Result<()> {
    let args = AppArgs::parse();

    let ctx = AppContext {
        file: args.file,
        log_level: args.log_level,
        log_file: args.log_file,
    };

    logger::init_logger(&ctx.log_level, ctx.log_file.clone());

    tracing::info!(event = "app_start", command = format!("{:?}", args.command));

    match args.command.unwrap_or(Commands::Cli) {
        Commands::Cli => {
            tracing::info!(event = "app_mode_selected", mode = "cli");
            cli::run_cli(ctx)
        }
        Commands::Tui => {
            tracing::info!(event = "app_mode_selected", mode = "tui");
            tui::run_tui(ctx)
        }
    }
}
