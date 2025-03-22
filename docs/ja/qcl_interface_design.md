# ✅ interface クレート設計書（adapter層・完全版／display_columns削除・ファクトリ追加対応版）

---

## ✅ 1. 概要
`interface` クレートは、`resolve` クレートが提供する `ValueProvider` トレイトを実装し、  
ユーザーとの対話インターフェースを提供するレイヤー。  
CLI・TUI・Web など複数の UI モードをサポートし、UI の実装と `resolve` クレートの処理を分離する。

---

## ✅ 2. クレートの責務

| カテゴリ                  | 内容                                                                                   |
|---------------------------|----------------------------------------------------------------------------------------|
| ✅ ユーザー入力の収集      | `prompt_input()` でユーザーの文字列入力を受け取る                                     |
| ✅ ユーザー選択肢の提示    | `prompt_select()` でリストからの選択を UI で提示する                                  |
| ✅ UI 実装の切り替え       | CLI（dialoguer）と TUI（ratatui）の実装を提供し、モードに応じた ValueProvider を提供   |
| ✅ resolve へのデータ提供  | resolve クレートへ、入力値または選択結果（行番号）を返却する                         |

---

## ✅ 3. クレートの非責務

| 項目                | 説明                                                                |
|---------------------|---------------------------------------------------------------------|
| ❌ プレースホルダ解決 | プレースホルダロジックは `resolve` クレートの責務                  |
| ❌ スニペット管理    | スニペットの読み込みとパースは `snippet` クレートの責務            |
| ❌ コマンド実行      | コマンドの出力と実行は `qcl-app` クレートの責務                   |

---

## ✅ 4. モジュール構成

```
crates/interface/
├── Cargo.toml
└── src/
    ├── lib.rs               // ファクトリ関数と公開API
    ├── cli.rs               // dialoguer 実装
    └── tui.rs               // ratatui 実装（今後追加予定）
```

---

## ✅ 5. モジュールと構造体・関数シグネチャ

### 📁 lib.rs（ファクトリ関数追加）
```rust
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
```

---

### 📁 cli.rs
#### ✅ 構造体
```rust
pub struct DialoguerProvider;
```

#### ✅ 関数シグネチャ
```rust
impl DialoguerProvider {
    pub fn new() -> Self;
}

impl ValueProvider for DialoguerProvider {
    fn prompt_input(
        &mut self,
        var_name: &str,
        prompt: &str,
        default: Option<String>
    ) -> anyhow::Result<String>;

    fn prompt_select(
        &mut self,
        var_name: &str,
        prompt: &str,
        records: Vec<Vec<String>>,
        default_index: Option<usize>
    ) -> anyhow::Result<usize>;
}
```

---

### 📁 tui.rs（実装予定）
```rust
pub struct TuiProvider;

impl TuiProvider {
    pub fn new() -> Self;
}

impl ValueProvider for TuiProvider {
    fn prompt_input(
        &mut self,
        var_name: &str,
        prompt: &str,
        default: Option<String>
    ) -> anyhow::Result<String>;

    fn prompt_select(
        &mut self,
        var_name: &str,
        prompt: &str,
        records: Vec<Vec<String>>,
        default_index: Option<usize>
    ) -> anyhow::Result<usize>;
}
```

---

## ✅ 6. ValueProvider トレイト定義
```rust
pub trait ValueProvider {
    fn prompt_input(
        &mut self,
        var_name: &str,
        prompt: &str,
        default: Option<String>
    ) -> anyhow::Result<String>;

    fn prompt_select(
        &mut self,
        var_name: &str,
        prompt: &str,
        records: Vec<Vec<String>>,
        default_index: Option<usize>
    ) -> anyhow::Result<usize>;
}
```

---

## ✅ 7. UI 処理フロー

```
qcl-app
└── run_cli() / run_tui()
    └── interface::create_cli_provider() / create_tui_provider()
        └── DialoguerProvider / TuiProvider
            └── resolve::run_qcl()
                └── ValueProvider::prompt_input / prompt_select
                    └── ユーザー入力・選択処理
```

---

## ✅ 8. CLI 実装仕様（DialoguerProvider）

### 🎯 prompt_input
```rust
let input: String = Input::new()
    .with_prompt(prompt)
    .default(default.unwrap_or_default())
    .interact_text()?;
Ok(input)
```

### 🎯 prompt_select（display_columns 削除・全カラム表示）
```rust
let items: Vec<String> = records.iter()
    .map(|record| record.join(" | "))
    .collect();

let selection = Select::new()
    .with_prompt(prompt)
    .items(&items)
    .default(default_index.unwrap_or(0))
    .interact()?;

Ok(selection)
```

---

## ✅ 9. tracing ログ設計

### 🎯 ログイベント一覧
| レベル  | イベント名            | 内容                                       |
|--------|-----------------------|--------------------------------------------|
| TRACE  | user_input_start      | var_name, prompt                           |
| TRACE  | user_input_end        | var_name, input_value                      |
| TRACE  | user_select_start     | var_name, prompt                           |
| TRACE  | user_select_end       | var_name, selected_index                   |

### 🎯 実装例
```rust
trace!(
    event = "user_input_start",
    var_name = var_name,
    prompt = prompt
);

let result = Input::new()
    .with_prompt(prompt)
    .default(default.unwrap_or_default())
    .interact_text()?;

trace!(
    event = "user_input_end",
    var_name = var_name,
    input_value = result
);
```

```rust
trace!(
    event = "user_select_start",
    var_name = var_name,
    prompt = prompt
);

let items: Vec<String> = records.iter()
    .map(|record| record.join(" | "))
    .collect();

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
```

---

## ✅ 10. 依存ライブラリ（Cargo.toml）

```toml
[dependencies]
anyhow = "1.0"
dialoguer = "0.10"
ratatui = { version = "0.20", optional = true }
tracing = "0.1"
resolve = { path = "../resolve" }
```

---

## ✅ 11. 将来の拡張

| 項目                    | 内容                              |
|-------------------------|-----------------------------------|
| TuiProvider             | ratatui 実装                     |
| WebProvider             | REST API / Web UI                |
| バリデーションの追加    | 型情報による入力制御とエラー表示 |
| 外部ツールの統合        | fzf や peco のバックエンド追加    |

---

## ✅ 12. interface クレートの責務まとめ

### ✅ やること
- CLI / TUI / 他インターフェースの ValueProvider 実装
- resolve に対する UI 提供
- ユーザー入力 / 選択の制御とデータ収集

### ❌ やらないこと
- プレースホルダ解決
- コマンド生成
- スニペット管理
- サブコマンドの制御
