# ✅ snippet クレート設計書（core層・完全版）

## ✅ 1. 概要
`snippet` クレートは、qcl 全体の「スニペットデータ構造」と「スニペットファイル操作」を担当する **coreレイヤー**。  
YAML ファイルで定義されたスニペット群を読み込み、構造体として提供し、`resolve` クレートに渡す。  
また、コマンド文字列中のプレースホルダ（`[[name=default ...]]`）を解析する。

---

## ✅ 2. クレートの責務

| カテゴリ        | 内容                                                                                 |
|-----------------|--------------------------------------------------------------------------------------|
| ✅ データ定義   | `Snippet`, `Function`, `Placeholder` などのドメインモデルを提供                     |
| ✅ データ取得   | スニペットファイル（YAML形式）を読み込み、構造体にデシリアライズ                   |
| ✅ ファイル操作 | 複数ファイルのマージ、同名スニペットの後勝ち処理                                    |
| ✅ 構文解析     | コマンド文字列内のプレースホルダ記法（`[[...]]`）をパースし、`Placeholder` として提供 |
| ✅ バリデーション | プレースホルダ構文や Function 定義の整合性を検証                                     |

---

## ✅ 3. クレートの非責務

| 項目                      | 説明                                             |
|---------------------------|--------------------------------------------------|
| ❌ プレースホルダの値解決  | ユーザー入力・外部コマンド実行などは `resolve` の責務 |
| ❌ UI 表示                | CLI/TUI/Web UI の表示・入力は `interface` の責務  |
| ❌ コマンド実行           | コマンドを実行するのは `resolve` もしくは `qcl-app` |
| ❌ 実行順制御（orderの適用） | プレースホルダの解決順は `resolve` が担当する     |

---

## ✅ 4. モジュール構成

```
crates/snippet/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── model.rs
    ├── loader.rs
    └── parser.rs
```

---

## ✅ 5. モジュールと構造体・関数シグネチャ

### 📁 model.rs
```rust
/// スニペット本体
pub struct Snippet {
    pub name: String,
    pub command: String,
    pub function: Option<Function>,
}

/// Function: コマンド実行と select マッピング
pub struct Function {
    pub from: String,                        // コマンド実行内容
    pub select: HashMap<String, usize>,     // プレースホルダ名 -> カラム番号
}

/// 1ファイル分のスニペット群
pub struct SnippetConfig {
    pub snippets: Vec<Snippet>,
}

/// コマンド内のプレースホルダ情報
pub struct Placeholder {
    pub name: String,                       // プレースホルダ名
    pub default: Option<String>,            // デフォルト値
    pub from: Option<String>,               // from:"コマンド"
    pub select: Option<usize>,              // select:N
    pub order: Option<usize>,               // order:N
}
```

---

### 📁 loader.rs
```rust
/// 任意のファイルをロードし、スニペット一覧を返す
pub fn load_snippet_configs(
    override_file: Option<String>
) -> anyhow::Result<Vec<Snippet>>;

/// デフォルトファイル＋任意ファイルをマージして返す
pub fn load_snippets_from_file<P: AsRef<Path>>(
    path: P
) -> anyhow::Result<Vec<Snippet>>;

/// 任意のスニペット名に一致する Snippet を返す
pub fn load_snippet_object(
    name: &str,
    override_file: Option<String>
) -> anyhow::Result<Snippet>;

/// 複数ファイルのマージ
fn merge_snippet_configs(
    configs: Vec<SnippetConfig>
) -> Vec<Snippet>;
```

---

### 📁 parser.rs
```rust
/// コマンド文字列中のすべてのプレースホルダをパース
pub fn parse_placeholders(
    command: &str
) -> Vec<Placeholder>;

/// プレースホルダ構文の検証
pub fn validate_placeholder(
    placeholder: &Placeholder
) -> anyhow::Result<()>;

/// Function 定義のバリデーション
pub fn validate_function(
    function: &Function
) -> anyhow::Result<()>;
```

---

## ✅ 6. プレースホルダ詳細（構造と流れ）

- コマンド内の `[[name=default from:"command" select:N order:N]]` を  
  → `Placeholder` に変換  
- `resolve` に渡すときはこの `Placeholder` を `HashMap<String, Placeholder>` にしておく  
- 必須チェックや `from/select/order` の整合性を `validate_placeholder()` でチェック

---

## ✅ 7. tracing ログ設計

### 🎯 基本方針
- `tracing` クレートを使用し、イベントログを発行する  
- フォーマットや出力先は `qcl-app` 側で制御  
- ログは構造化（イベント名＋変数情報）

---

### 🎯 ログイベント一覧

| レベル  | イベント名                   | 出力する情報                          | 説明                                                  |
|--------|------------------------------|--------------------------------------|-------------------------------------------------------|
| TRACE  | snippet_load_from_file_start | file                                | ファイルロード処理の開始                             |
| TRACE  | snippet_load_from_file_end   | file, snippet_count                 | ロード完了（読み込み件数）                           |
| DEBUG  | snippet_merged               | total_files, merged_snippet_count   | 複数ファイルマージ時                                 |
| DEBUG  | placeholder_parsed           | name, default, from, select, order  | プレースホルダ解析成功時                             |
| ERROR  | snippet_file_not_found       | file, error                         | ファイルが存在しない                                 |
| ERROR  | snippet_parse_error          | file, error                         | YAMLパース失敗                                       |

---

### 🎯 実装例  
#### ファイル読み込み開始
```rust
trace!(
    event = "snippet_load_from_file_start",
    file = ?path.as_ref()
);
```

#### ファイル読み込み成功
```rust
trace!(
    event = "snippet_load_from_file_end",
    file = ?path.as_ref(),
    snippet_count = config.snippets.len()
);
```

#### プレースホルダ解析成功
```rust
debug!(
    event = "placeholder_parsed",
    name = %ph.name,
    default = ?ph.default,
    from = ?ph.from,
    select = ?ph.select,
    order = ?ph.order
);
```

---

## ✅ 8. 依存ライブラリ（Cargo.toml）
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
regex = "1.5"
anyhow = "1.0"
tracing = "0.1"
```

---

## ✅ 9. 将来の拡張

| 項目                          | 内容                                       |
|-------------------------------|--------------------------------------------|
| プレースホルダ型サポート      | int/bool/date 型などを宣言してバリデーション |
| 依存関係の定義               | 他のプレースホルダに依存する構造の定義      |
| 外部スニペットソース          | Git/HTTP からスニペットファイルをロード     |

---

## ✅ 10. snippet クレートの責務まとめ

### ✅ やること  
- スニペットモデルの定義 (`Snippet`, `Function`, `Placeholder`)  
- ファイルロードとマージ（YAML 読み込み＋後勝ちルール）  
- プレースホルダ構文解析とバリデーション  
- ログイベントの発行

### ❌ やらないこと  
- プレースホルダ値の解決  
- UI 表示・選択処理  
- コマンド実行・出力  
- サブコマンド・アプリケーションフローの制御

