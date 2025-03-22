
# ✅ スニペットのパターン設計書

---

## ✅ 1. 概要

スニペットは、YAMLファイルに記述されたコマンドテンプレートであり、`qcl` のコアとなるデータ構造です。  
その中に定義される **プレースホルダ** により、動的な値の埋め込みや外部情報の取得を行うことができます。

---

## ✅ 2. プレースホルダ記法の基本構文

```
[[name=default from:"command" select:N order:N]]
```

| 要素         | 説明                                                                       |
|--------------|----------------------------------------------------------------------------|
| `name`       | プレースホルダ名（必須）                                                   |
| `=default`   | デフォルト値（省略可能）                                                  |
| `from:"command"` | 値を取得するための外部コマンド、または function（省略可能）             |
| `select:N`   | 外部コマンドの出力が複数列ある場合、何列目を値として取得するか（0-based） |
| `order:N`    | ユーザー入力/選択を促す順番を制御（省略可能、自動順も可）                |

---

## ✅ 3. プレースホルダパターン一覧

### 🎯 3.1 単純入力プレースホルダ
#### ✅ 構文
```
[[name]]
```
#### ✅ 例
```yaml
command: "echo Hello, [[name]]!"
```
#### ✅ 解説
- ユーザーに `name` の値を入力させる。
- デフォルト値なし。

---

### 🎯 3.2 デフォルト値付き入力
#### ✅ 構文
```
[[name=default]]
```
#### ✅ 例
```yaml
command: "echo Hello, [[name=world]]!"
```
#### ✅ 解説
- デフォルト値 `world` を表示し、ユーザーが入力できる。

---

### 🎯 3.3 外部コマンドから選択
#### ✅ 構文
```
[[name from:"command" select:N]]
```
#### ✅ 例
```yaml
command: >
  docker exec -it [[container_id from:"docker ps --format '{{.ID}} {{.Names}}'" select:0]] /bin/bash
```
#### ✅ 詳細な解説
- `docker ps --format '{{.ID}} {{.Names}}'` を実行し、結果の各行を候補として表示する。
- ユーザーは一覧から行を選択する。
  ```
  [0] 3e1a57c1234 nginx-container
  [1] a9bf21d9876 redis-server
  ```
- 選んだ行の **select:N** に指定したカラム番号の値が、`container_id` に埋め込まれる。  
  → たとえば `select:0` なら `a9bf21d9876` がセットされる。

#### ✅ 典型的な用途
- Dockerコンテナ一覧から選択
- Kubernetes Pod一覧から選択
- SSH先ホスト一覧から選択

---

### 🎯 3.4 order 指定付き
#### ✅ 構文
```
[[name order:N]]
```
#### ✅ 例
```yaml
command: "ssh [[user order:2]]@[[host order:1]]"
```
#### ✅ 解説
- `host` が先に、`user` が後に解決される（表示順制御）。
- 実行時の入力順番を意図的にコントロールできる。

---

### 🎯 3.5 function による複数プレースホルダ解決
#### ✅ 構文
```
[[var from:function]]
```
#### ✅ 例
```yaml
command: |
  ssh [[user from:function]]@[[host from:function]]
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
#### ✅ 解説
- `function` に定義されたコマンドが実行され、その結果を複数のプレースホルダにマッピングする。
- `select` で各カラムのどの値を `user`、`host` にマッピングするかを定義。

---

### 🎯 3.6 複数の選択肢と function 混在パターン
```yaml
snippets:
  - name: multi-source
    command: "ssh [[user=ubuntu]]@[[host from:"gcloud compute instances list --format 'value(NAME)'" select:0]]"
```
#### ✅ 解説
- `user` はデフォルト `ubuntu`（ユーザー入力可）  
- `host` は `gcloud compute instances list` の結果から選択

---

## ✅ 4. プレースホルダ構文ルールまとめ

| ルール                | 説明                                                 |
|-----------------------|------------------------------------------------------|
| name は必須           | 変数名を必ず指定する（省略不可）                     |
| デフォルト値は省略可   | `=` がなければデフォルト値なし（空文字）             |
| from があれば選択式    | `from:"command"` が指定された場合は選択式UIが呼ばれる |
| select があればカラム選択 | 0-based インデックスで列指定                       |
| order は任意          | 明示しない場合はプレースホルダの出現順で解決される   |

---

## ✅ 5. スニペット YAML フォーマットのパターン例

```yaml
snippets:
  - name: example
    command: "echo Hello, [[name=world]]!"

  - name: ssh-login-host
    command: "ssh [[user order:2]]@[[host order:1]]"

  - name: docker-exec
    command: |
      docker exec -it [[container_id from:"docker ps --format '{{.ID}} {{.Names}}'" select:0]] /bin/bash

  - name: ssh-login-user@hostname
    command: |
      ssh [[user from:function]]@[[host from:function]]
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
