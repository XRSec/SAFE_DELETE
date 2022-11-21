# SAFE DELETE
安全的删除文件/文件夹

## Example

```bash
# download rm_release_url and mv to C:\Windows\System32\rmf.exe 

# wget rm_release_url -O /usr/local/bin/rmf

sudo chmod +x /usr/local/bin/rmf

# ~/.zshrc
alias rm="echo 'use rmf'; rmf"
```