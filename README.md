# qcl - Quick Command Launcher

[🇯🇵 日本語版README](docs/ja/README.md)  

## 🚀 Overview
**`qcl` (Quick Command Launcher)** is a CLI tool that helps you select and execute command snippets efficiently in your terminal.  
Snippets are managed in YAML files, and you can dynamically embed values through interactive prompts and selections.  
Our goal is to enable **error-free and effortless CLI operations** without the need to memorize commands.

---

## ✅ Key Features
- 🔖 Define command snippets in YAML
- ✏️ Interactive input and selection for placeholders
- 🏗️ Use `function` to batch select and split multiple values automatically
- 📂 Load additional snippet files with the `-f` option (with priority handling)
- 🛠️ Automatically generate a default snippet file at `~/.config/qcl/snippets.yaml`

---

## 🎬 Sample: Connect via SSH config
![demo](docs/demo.gif)

---

## 🛠️ Installation
```bash
git clone --depth 1 https://github.com/nakkiy/qcl ~/.qcl
cd ~/.qcl
cargo install --path .
```

After installation, make sure `$HOME/.cargo/bin` is included in your `PATH`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Copy the sample `snippets.yaml`:
```bash
mkdir ~/.config/qcl/
cp ~/.qcl/sample/snippets.yaml ~/.config/qcl/
```

If you're using bash, you can bind `ctrl + /` to launch `qcl` by running:
```bash
echo "[ -f ~/.qcl/shell/keybinding.bash ] && source ~/.qcl/shell/keybinding.bash" >> ~/.bashrc
```

---

## 🖥️ How to Use
### Select from a list of snippets
```bash
qcl
```

1. Choose a snippet
2. Enter/select values for the placeholders
3. View the final command

---

## ⚙️ Options
| Option          | Description                                                        |
|-----------------|--------------------------------------------------------------------|
| `-f, --file`    | Load an additional YAML file. Snippets with duplicate keys are overridden by this file. |

Example:
```bash
qcl -f ./my_snippets.yaml
```

---

## 🗂️ YAML Snippet Structure
### Default snippet file
On the first run, the following file is automatically generated:
```
~/.config/qcl/snippets.yaml
```

---

## 🔡 Placeholder Syntax
```
[[name=default]]
[[name from:"command" select:1 order:2]]
[[name from:function]]
```

| Parameter   | Description                                                    |
|-------------|----------------------------------------------------------------|
| `name`      | Placeholder name                                               |
| `=default`  | Default value                                                  |
| `from`      | Use command output as selectable choices                      |
| `select`    | Field index of the choice (starting from 0)                   |
| `order`     | Specify the input/selection order (executed by ascending order)|
| `function`  | See below                                                     |

---

## 🏗️ What is `function`?
With `function`, you can **input multiple fields at once by a single selection**.  
It's handy when you want to retrieve and split multiple values together!

### Example use cases
- Build an `ssh` connection command by fetching `Hostname` and `User` from `.ssh/config`
- Populate multiple fields from a single item in a list

### YAML Example
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

#### How it works
1. Executes the command defined in `function`
2. Select one item from the result list
3. Extract the specified fields with `select`, and embed them into the placeholders inside `command`

---

## Future Plans
- Add search functionality in each selection list
- Organize/search snippets by tags

---

## 🤝 Contributing
Pull requests for improvements, suggestions, or new features are welcome!  
Even sharing your own useful snippets is great too!  
Feel free to open an Issue or PR 🙌

---

## 📝 License
- [MIT License](LICENSE-MIT) or https://opensource.org/licenses/MIT
- [Apache License 2.0](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0

