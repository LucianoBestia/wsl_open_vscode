# wsl_open_vscode

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
To build and run with parameters use this:  

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
# wsl_open_vscode
