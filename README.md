# wsl_open_vscode

**Open folder in VSCode from WSL2 simply with `$ code .`**  
***version: 2021.815.1030  date: 2021-08-15 author: [dev_Bestia](https://bestia.dev) repository: [GitHub](https://github.com/LucianoBestia/wsl_open_vscode)***  

Inside WSL2 I want to open a folder in VSCode. But VSCode is in Win10.  
This is useful when navigating in the WSL2 Debian file system.  
It used to work in WSL1 like this:  

```bash
code .
```

but it does not work anymore in WSL2.  

## Symbolic link

I will prepare a symbolic link to the `VSCode_Path\bin\code` file in windows.
This is a shebang file for Linux prepared for VSCode.

```bash
sudo ln -sf "/mnt/c/Users/Luciano/AppData/Local/Programs/Microsoft VS Code/bin/code" /usr/bin/code
```

Now I can open a folder like this:

```bash
cd ~/rustprojects/qvs20
code .
```

## from windows

I use the TotalCommander file manager in windows.  
When I am inside a Linux folder, I can open it in the terminal with

```powershell
wsl
```

Then in the Linux terminal I run:

```bash
code .
```
