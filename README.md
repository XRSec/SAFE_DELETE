# SAFE DELETE

安全的删除文件/文件夹

Rust 实现会把目标文件或文件夹移动到系统垃圾篓，而不是直接永久删除。

旧版 Go 实现已移动到 `deprecated/`。

## Support

底层使用 Rust [`trash`](https://crates.io/crates/trash) 库，支持：

- Windows Recycle Bin
- macOS Trash
- Linux / BSD 等 FreeDesktop Trash 兼容环境，例如 GNOME、KDE、XFCE

在 Linux / FreeDesktop 环境中，如果用户垃圾篓目录不存在，程序会尝试创建
`~/.local/share/Trash/files` 和 `~/.local/share/Trash/info` 等必要目录。

如果系统没有可用垃圾篓、目标卷只读、权限不足，或 macOS Recovery 等环境缺少完整桌面
Trash 能力，移动到垃圾篓会失败。`rmf` 会打印错误并返回非 0 退出码，不会自动改为永久删除。

## Build

```bash
cargo build --release
sudo install -m 755 target/release/rmf /usr/local/bin/rmf
```

## Example

```bash
# download rm_release_url and mv to C:\Windows\System32\rmf.exe

# wget rm_release_url -O /usr/local/bin/rmf

sudo chmod +x /usr/local/bin/rmf

# ~/.zshrc
alias rm="echo 'use rmf'; rmf"

rmf -iv file.txt
rmf -rf dir/
```

## [Trash](https://command-not-found.com/trash-empty)

```bash
apt/yum/brew/dnf install trash-cli

trash-empty
```

                       
