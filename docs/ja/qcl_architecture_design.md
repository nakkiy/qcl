
# qcl 全体設計書（ライブラリクレート＋サブコマンド対応版）

---

## 1. 概要

`qcl` は、YAML で管理されたコマンドスニペットを対話的に選択・編集し、最終的なコマンド文字列を生成するツールです。
CLI と TUI インターフェースを1つのバイナリで提供し、将来的に Web/REST API などの他のインターフェースも追加可能なアーキテクチャを目指します。

---

## 2. クレート構成と依存関係

```
qcl/
├── crates/
│   ├── snippet/      # スニペット管理（core層）
│   ├── resolve/      # プレースホルダ解決ロジック（engine層）
│   └── interface/    # UIインターフェース（adapter層）
└── qcl-app/          # 実行用バイナリクレート（サブコマンドによるモード切替）
```

---

## 3. クレートの責務と概要

### 3.1 snippet クレート（core層）

#### 🎯 目的
- スニペットのモデル定義
- YAMLファイルの読み込み・マージ・パース
- プレースホルダ文字列のパース（[[name=default...]]）

#### 🎯 依存ライブラリ
- serde / serde_yaml

### 3.2 resolve クレート（engine層）

#### 🎯 目的
- プレースホルダの解決ロジック
- ValueProvider によるユーザーインタラクション抽象化
- 最終的なコマンド文字列の生成

#### 🎯 依存ライブラリ
- snippet クレート
- anyhow（エラー管理）

### 3.3 interface クレート（adapter層）

#### 🎯 目的
- ユーザーインターフェースの実装
- ValueProvider トレイトを使ったCLI/TUIプロバイダの提供

#### 🎯 初期実装
- DialoguerProvider（CLI用）
- 将来的に TuiProvider（TUI用）を追加予定

#### 🎯 依存ライブラリ
- resolve クレート
- dialoguer（CLI）
- ratatui（TUI時に導入）

### 3.4 qcl-app クレート（実行バイナリ）

#### 🎯 目的
- 実行エントリポイント（サブコマンド管理）
- ログ・設定初期化
- CLI/TUIの起動と resolve への処理委譲

#### 🎯 依存ライブラリ
- interface クレート
- clap（サブコマンド）
- tracing / tracing-subscriber

---

## 4. 実行モードとサブコマンド設計

### 🎯 サブコマンド一覧
| サブコマンド  | 説明                  |
|---------------|-----------------------|
| cli           | 対話的なCLI実行モード |
| tui           | CUIベースのTUIモード  |

```bash
qcl cli --file snippets.yaml
qcl tui
```

### 🎯 clap を用いたコマンドライン設計

```rust
#[derive(Parser)]
#[command(name = "qcl", version, about)]
pub struct AppArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Cli {
        #[arg(short, long)]
        file: Option<String>,
    },
    Tui,
}
```

---

## 5. データフロー（全体）

```
qcl-app (main.rs)
└── AppArgs → Commands::Cli
    ├── ロガー初期化 (tracing)
    ├── snippet::loader::load_snippets(file)
    ├── interface::DialoguerProvider インスタンス生成
    └── resolve::resolver::run_qcl(snippets, provider)
        └── provider.prompt(ResolvePrompt) でユーザーインターフェースを実行
```

---

## 6. データ構造とプロンプト設計

### 🎯 ResolvePrompt（resolve → interface）
```rust
pub enum ResolvePrompt {
    Input {
        var_name: String,
        prompt: String,
        default: Option<String>,
    },
    Select {
        var_name: String,
        prompt: String,
        records: Vec<SelectRecord>,
        display_columns: Vec<usize>,
        default_index: Option<usize>,
    },
}
```

### 🎯 ResolveAnswer（interface → resolve）
```rust
pub enum ResolveAnswer {
    Input(String),
    Selection(usize),
}
```

---

## 7. プレースホルダ解決仕様

```
[[name=default from:"command" select:N order:N]]
```

### 🎯 解決方法
| 種類      | 説明                                                        |
|-----------|-------------------------------------------------------------|
|入力       |from が無い → ユーザー入力                                  |
|選択       |from がある → コマンド実行 or function の結果からリスト選択 |

| 記法                                         | 処理内容                                    |
|----------------------------------------------|---------------------------------------------|
| [[name=default]]                             | 入力プロンプト表示                          |
| [[var from:"command" select:N]]              | コマンド実行後リスト選択 → N列目を返す     |
| [[var from:function]]                        | function 定義に従い実行・リスト選択 → 値返却 |

### 🎯 function 定義
```
function:
  from: "<コマンド or スクリプト>"
  select:
    user: 2
    host: 1
```

### サンプル
```
snippets:
  - name: example
    command: "echo Hello, [[name=world]]!"

  - name: ssh-login-host
    command: ssh [[user order:2]]@[[host order:1]]

  - name: docker-exec
    command: docker exec -it [[container_id from:"docker ps --format '{{.ID}} {{.Names}}'" select:1]] /bin/bash

  - name: ssh-login-user@hostname
    command: ssh [[user from:function]]@[[host from:function]]
    function:
      from: >
        awk '$1 == "Host" {
                if (host != "" && hostname != "" && user != "")
                    print host, hostname, user
                host=$2; hostname=""; user=""
              }
              $1 == "Hostname" {hostname=$2}
              $1 == "User" {user=$2}
              END {
                if (host != "" && hostname != "" && user != "")
                    print host, hostname, user
              }' ~/.ssh/config
      select:
        user: 2
        host: 1
```

---

## 9. クレートフォルダ構成
```
qcl/
├── crates/
│   ├── snippet/
│   ├── resolve/
│   └── interface/
└── qcl-app/
```

---

## 12. 将来的な拡張

- TUI モード（ratatui）
- WebAPI モード
- スニペット共有機能（Gitなど）
- Function 外部プラグイン対応
