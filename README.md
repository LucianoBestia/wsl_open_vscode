# wsl_open_vscode

**Open folder in VSCode from WSL2 simply with `$ code .`**  
***[repository](https://github.com/lucianobestia/wsl_open_vscode/); version: 2021.815.1030  date: 2021-08-15 authors: Luciano Bestia***  

Inside WSL2 I want to open a folder in VSCode. But VSCode is in Win10.  
This is useful when navigating in the WSL2 Debian file system.  
It used to work in WSL1 like this:  

```bash
code .
```

but it does not work anymore in WSL2.  

## Open folder in Win10 VSCode

I will prepare a symbolic link to the `VSCode_Path\bin\code` file in windows.
This is a shebang file for Linux.

```bash
sudo ln -sf "/mnt/c/Users/Luciano/AppData/Local/Programs/Microsoft VS Code/bin/code" /usr/bin/code
# now I can open a folder like this
cd ~/rustprojects/qvs20
code .
```

