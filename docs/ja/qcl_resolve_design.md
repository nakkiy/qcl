# ✅ resolve クレート設計書（engine層・最新版／orderロジック修正版）

---

## ✅ 1. 概要  
`resolve` クレートは、qcl の **エンジン層**。  
スニペット内に含まれるプレースホルダを動的に解決し、最終的なコマンド文字列を生成する。  
ユーザーとのインタラクションは `interface` クレートが担当し、  
`ValueProvider` トレイトを通じて抽象化することで UI 非依存の実装を実現する。

---

## ✅ 2. クレートの責務

| カテゴリ              | 内容                                                                                 |
|-----------------------|--------------------------------------------------------------------------------------|
| ✅ プレースホルダ解決 | プレースホルダ（Placeholder）の値を決定し、コマンドテンプレートに埋め込む処理         |
| ✅ Function実行       | Function 定義がある場合に外部コマンドを実行し、結果を解釈して候補データを作成する     |
| ✅ UIへの問い合わせ  | ValueProvider トレイトによる、ユーザーインタラクションの抽象化                     |
| ✅ 解決状態の保持    | プレースホルダ名 → 値の解決済みマップを管理し、再利用/二重入力を防ぐ               |
| ✅ 最終コマンド生成  | 解決後のコマンド文字列を返却（実行はしない）                                      |

---

## ✅ 3. クレートの非責務

| 項目                | 説明                                                                 |
|---------------------|----------------------------------------------------------------------|
| ❌ UI表示／入出力    | `interface` クレートの責務                                           |
| ❌ コマンド実行      | ユーザーにコマンドを提示するだけで、実行はしない（qcl-appの責務）   |
| ❌ スニペット管理    | `snippet` クレートの責務                                            |
| ❌ サブコマンド実装  | CLI/TUI切替やアプリエントリーポイントは `qcl-app` クレートが行う     |

---

## ✅ 4. モジュール構成

```
crates/resolve/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── resolver.rs    // プレースホルダ解決エンジン
    └── provider.rs    // UI抽象レイヤー (ValueProvider トレイト)
```

---

## ✅ 5. モジュールと構造体詳細

### 📁 resolver.rs

#### ✅ 公開関数

##### 【1】run_qcl
```rust
pub fn run_qcl<P: ValueProvider>(
    snippets: &[Snippet],
    mut provider: P
) -> anyhow::Result<String>;
```

##### 【2】resolve_placeholders
```rust
pub fn resolve_placeholders<P: ValueProvider>(
    snippet: &Snippet,
    provider: &mut P
) -> anyhow::Result<String>;
```

#### ✅ 非公開関数
```rust
fn process_function(function: &Function) -> anyhow::Result<Vec<Vec<String>>>;

fn resolve_placeholder<P: ValueProvider>(
    ph: &Placeholder,
    function: &Option<Function>,
    vars: &mut HashMap<String, String>,
    provider: &mut P
) -> anyhow::Result<()>;

fn render_command(template: &str, vars: &HashMap<String, String>) -> String;
```

---

### 📁 provider.rs
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
        display_columns: Vec<usize>,
        default_index: Option<usize>
    ) -> anyhow::Result<usize>;
}
```

---

## ✅ 6. ValueProvider による interface とのやり取り仕様

### 🎯 基本方針
- `resolve` クレートは UI を持たない  
- `ValueProvider` トレイトを通じてユーザーインタラクションを外部に委譲する  
- `interface` クレートは `ValueProvider` を実装し、具体的な UI を提供する

### 🎯 役割の分担

| resolve クレート                   | interface クレート                       |
|-----------------------------------|-----------------------------------------|
| `ValueProvider` トレイトを定義     | `ValueProvider` トレイトを実装          |
| ユーザーから値を取得したい場合に、`ValueProvider` を呼び出す | 実際に入力フォームや選択肢 UI を提供    |
| 戻り値を使ってプレースホルダ解決を進める | 戻り値（入力値・行番号）を返却する       |

---

### 🎯 トレイト定義
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
        display_columns: Vec<usize>,
        default_index: Option<usize>
    ) -> anyhow::Result<usize>;
}
```

### 🎯 やり取りフロー
1. `resolve_placeholder()` 内で `prompt_input()` または `prompt_select()` を呼ぶ  
2. `interface` クレートがダイアログや TUI を表示  
3. ユーザー入力値 or 行番号が返る  
4. resolve 側で値をマップに登録してプレースホルダを解決する

### 🎯 データの流れ図
```
resolve::resolve_placeholder()
    └── provider.prompt_input() / prompt_select()
        └── interface::DialoguerProvider or TuiProvider
            └── ユーザーが入力/選択
        ←── 値（String or usize）を返す
    ←── プレースホルダに値をセット
```

## ✅ 7. プレースホルダ解決フロー

```
run_qcl():
  └─ snippet選択（interface::ValueProvider）
  └─ resolve_placeholders():
      ├─ 各プレースホルダを順次処理
      ├─ from/command or function 実行→records生成
      ├─ prompt_select()/prompt_input()でユーザー入力
      └─ 最終的な文字列を組み立て
```

---

## ✅ 7.1 プレースホルダ解決の順序ルール【追加】

プレースホルダの解決順序は、以下のルールに従う。

### ✅ 処理順序ルール
| 状況                          | 処理順序                                      |
|-------------------------------|-----------------------------------------------|
| **order が指定されたプレースホルダ** | `order` の値で昇順にソートし、順番に解決      |
| **order が無いプレースホルダ**      | パース順（スニペット記述順）で順番に解決      |
| **order 指定と未指定が混在**        | 1. `order` 指定ありを昇順で解決<br>2. 指定なしをパース順で解決 |

### ✅ 詳細ルール
- `order` の値は整数（正の数、0ベース可）  
- `order` が飛び飛びでも問題なし（昇順で処理）  
- `order` が重複している場合、パース順に解決する  
- `order` が一切指定されていない場合、すべてパース順で解決される  
- 無効な `order` 値（負数やパース不能など）は `snippet` クレートでバリデーション済みとする  
（resolve 側ではチェックしない）

### ✅ 実装ポイント
- `resolve_placeholders()` 内で  
  - `order` が `Some` なものと `None` なものに分けて処理  
  - `Some(order)` は昇順ソート後に解決  
  - `None` は順次解決  
- このルールは**プレースホルダの依存関係とは無関係**に適用される  
  - 依存関係は将来的な機能（並列処理や依存解決）で拡張予定

---

## ✅ 8. tracing ログ設計

### 🎯 基本ポリシー
- `tracing` で構造化ログを出力  
- `qcl-app` がフォーマット・出力を管理

### 🎯 ログイベント一覧  
| レベル  | イベント名                 | 内容                                           |
|--------|----------------------------|------------------------------------------------|
| TRACE  | placeholder_resolve_start   | snippet_name, placeholder_name                |
| TRACE  | placeholder_resolve_end     | snippet_name, placeholder_name, resolved_value|
| DEBUG  | placeholder_value_resolved  | var_name, value                               |
| DEBUG  | external_command_executed   | command, stdout_lines                         |
| ERROR  | external_command_failed     | command, error                                |
| ERROR  | no_snippet_found            | snippet_name                                  |

---

### 🎯 実装例  
```rust
trace!(
    event = "placeholder_resolve_start",
    snippet_name = %snippet.name,
    placeholder_name = %ph.name
);

debug!(
    event = "placeholder_value_resolved",
    var_name = %ph.name,
    value = %resolved_value
);

error!(
    event = "external_command_failed",
    command = %cmd,
    error = %err
);
```

## ✅ 9. 依存ライブラリ（Cargo.toml）
```toml
[dependencies]
anyhow = "1.0"
regex = "1.5"
tracing = "0.1"
snippet = { path = "../snippet" }
```

---

## ✅ 10. 将来の拡張

| 項目                     | 内容                                       |
|--------------------------|--------------------------------------------|
| 並列プレースホルダ解決    | 依存関係のないプレースホルダを並列処理     |
| 外部プラグインのサポート | Function を外部プロセス/プラグインに委譲 |
| 型バリデーション強化     | int/float/bool など型情報による検証       |

---

## ✅ 11. resolve クレートの責務まとめ

### ✅ やること  
- プレースホルダ解決処理  
  - `order` を考慮した解決順制御【追記】  
- Function 実行 → 候補取得とマッピング  
- UI 抽象インターフェースの提供  
- コマンド文字列の組み立て  
- ログイベントの発行

### ❌ やらないこと  
- UI 実装  
- コマンドの実行  
- スニペットの読み込み  
- サブコマンドやアプリ制御

---

## ✅ コードへの反映（補足）

#### `resolve_placeholders()` に追加される処理イメージ
```rust
// 1. プレースホルダを分類
let (mut ordered, mut unordered): (Vec<_>, Vec<_>) = snippet.placeholders
    .iter()
    .partition(|ph| ph.order.is_some());

// 2. ordered を昇順ソート
ordered.sort_by_key(|ph| ph.order.unwrap());

// 3. 解決処理（ordered → unordered の順）
for ph in ordered.iter().chain(unordered.iter()) {
    resolve_placeholder(ph, ...)?;
}
```

