# wsl_open_vscode

**Open folder in VSCode from WSL2 simply with $ code .**  
***version: 2021.815.1030  date: 2021-08-15 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/wsl_open_vscode)***  

![status](https://img.shields.io/badge/obsolete-red) 
![status](https://img.shields.io/badge/archived-red) 
![status](https://img.shields.io/badge/tutorial-yellow) 
![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/461901148.svg)

Hashtags: #rustlang #tutorial  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

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

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
