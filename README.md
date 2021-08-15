[comment]: # (auto_md_to_doc_comments segment start A)

# wsl_open_vscode

[comment]: # (auto_cargo_toml_to_md start)

**Open folder in VSCode from WSL2 simply with `$ code .`**  
***[repository](https://github.com/lucianobestia/dropbox_backup_to_external_disk/); version: 2021.815.1030  date: 2021-08-15 authors: Luciano Bestia***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-28-green.svg)](https://github.com/LucianoBestia/wsl_open_vscode/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-59-blue.svg)](https://github.com/LucianoBestia/wsl_open_vscode/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-9-purple.svg)](https://github.com/LucianoBestia/wsl_open_vscode/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/wsl_open_vscode/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/wsl_open_vscode/)

[comment]: # (auto_lines_of_code end)

Inside WSL2 I want to open a folder in VSCode. But VSCode is in Win10.  
This is useful when navigating in the WSL2 Debian file system.  
It used to work in WSL1 like this:  

```bash
code .
```

but it does not work anymore in WSL2.  
Let's make a rust utility for that tiny problem.  

## Open folder in Win10 VSCode

I prepared a symbolic link to the `code.exe` in windows.

```bash
sudo ln -s "/mnt/c/Users/Luciano/AppData/Local/Programs/Microsoft VS Code/Code.exe" /usr/bin/vscode_in_win
# now I can open a folder like this
vscode_in_win --remote wsl+Debian ~/rustprojects/qvs20
```

Sadly I cannot use the `.` as an argument. I don't know why.  
I will transform the `.` into the Linux path for the `env::current_dir()`.  

## Development

This is a simple binary. For good habit I separated the lib from the bin.  
To build use this:  

```bash
cargo build --release
# experimentally using strip to make the binary 10x smaller:
strip target/release/wsl_open_vscode
```

Maybe the file needs to be marked as executable, I am not sure:

```bash
chmod a+x target/release/wsl_open_vscode
```

Copy the file `target/release/wsl_open_vscode` to `/usr/bin` and change the name to simply `code`.  

```bash
sudo cp target/release/wsl_open_vscode /usr/bin/code
```

Now we can open the browser like this:  

```bash
cd ~/rustprojects/qvs20
code .
```

[comment]: # (auto_md_to_doc_comments segment end A)
