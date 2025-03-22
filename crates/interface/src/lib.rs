pub mod cli;
pub mod tui;

use cli::DialoguerProvider;
use tui::TuiProvider;

/// CLI 用 ValueProvider を生成する
pub fn create_cli_provider() -> DialoguerProvider {
    DialoguerProvider::new()
}

/// TUI 用 ValueProvider を生成する（今後追加予定）
pub fn create_tui_provider() -> TuiProvider {
    TuiProvider::new()
}
