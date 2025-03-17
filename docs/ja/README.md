# qcl - Quick Command Launcher

## 🚀 概要
**`qcl` (Quick Command Launcher)** は、ターミナルで使うコマンドスニペットの選択＆実行をサポートする CLI ツールです。  
スニペットは YAML ファイルで管理し、インタラクティブな選択・入力で動的に値を埋め込みます。  
**覚えない・間違えない CLI 操作** を目指します。

---

## ✅ 主な機能
- 🔖 YAML でコマンドスニペットを定義
- ✏️ プレースホルダーへのインタラクティブ入力と選択
- 🏗️ `function` で複数の値を一括選択・フィールド自動分割
- 📂 `-f` オプションで追加スニペットファイルを読み込み可能（優先順位あり）
- 🛠️ デフォルトファイル（`~/.config/qcl/snippets.yaml`）の自動生成

---

## 🎬 ssh config から接続先を選ぶサンプル
![demo](../demo.gif)

---

## 🛠️ インストール
```bash
git clone --depth 1 https://github.com/nakkiy/qcl ~/.qcl
cd ~/.qcl
cargo install --path .
```

インストール後、`$HOME/.cargo/bin` が `PATH` に含まれていることを確認：
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

[snippets.yaml](sample/snippets.yaml)をコピー
```bash
mkdir ~/.config/qcl/
cp ~/.qcl/sample/snippets.yaml ~/.config/qcl/
```

bashの場合、下記を実行することで `ctrl + /` で実行することができます。
```bash
echo "[ -f ~/.qcl/shell/keybinding.bash ] && source ~/.qcl/shell/keybinding.bash" >> ~/.bashrc
```

---

## 🖥️ 使い方
### スニペット一覧から選択
```bash
qcl
```

1. スニペットを選ぶ
1. プレースホルダーに値を入力・選択
1. 最終的なコマンドを表示

---

## ⚙️ オプション
| オプション      | 説明                                               |
|-----------------|----------------------------------------------------|
| `-f, --file`    | 追加の YAML ファイルを読み込む。重複キーはこのファイルが優先 |

例：
```bash
qcl -f ./my_snippets.yaml
```

---

## 🗂️ YAMLスニペットの構成
### デフォルトスニペットファイル
初回実行時、以下のファイルが自動生成されます：
```
~/.config/qcl/snippets.yaml
```

---

## 🔡 プレースホルダー構文
```
[[name=default]]
[[name from:"コマンド" select:1 order:2]]
[[name from:function]]
```

| パラメータ   | 説明                                                        |
|--------------|-------------------------------------------------------------|
| `name`       | プレースホルダー名                                          |
| `=default`   | デフォルト値                                                |
| `from`       | コマンド出力を選択肢にする                                  |
| `select`     | 選択肢のフィールドインデックス（0始まり）                   |
| `order`      | 入力・選択順を指定（数字が小さい順に実行）                  |
| `function`   | 下記参照                                                    |

---

## 🏗️ `function` とは？
`function` を使うと、**1回の選択操作で複数のフィールドを一括入力**できます。  
複数項目をまとめて取得・分解したい場合に便利！

### 使いどころ
- `.ssh/config` の `Hostname / User` を一度に取得して `ssh` 接続を組み立てる
- 一覧から選んで複数フィールドを埋め込むとき

### YAML例
```yaml
- name: ssh-login-function
command: ssh [[user from:function]]@[[host from:function]]
function:
    multi: true
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

#### 処理の流れ
1. `function` で定義したコマンドを実行
1. 結果リストから1つ選択
1. `select` で指定したフィールドを変数化し、`command` 内のプレースホルダーに埋め込む

---

## 今後
- 各選択リストで検索機能
- スニペットをtagでの分類、検索

---

## 🤝 コントリビュート
改善・提案・機能追加のPR大歓迎！  
「こんなスニペット作ってみた」でもOK！  
気軽にIssue・PRお願いします 🙌

---

## 📝 ライセンス
- [MIT License](../../LICENSE-MIT) または https://opensource.org/licenses/MIT
- [Apache License 2.0](../../LICENSE-APACHE) または https://www.apache.org/licenses/LICENSE-2.0

