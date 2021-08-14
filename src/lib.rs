// wsl_open_vscode

// region: auto_md_to_doc_comments include README.md A //!
//! # wsl_open_vscode
//!
//! **Open folder in VSCode from WSL2 simply with `$ code .`**  
//! ***[repo](https://github.com/lucianobestia/dropbox_backup_to_external_disk/); version: 0.1.0  date: 2021-08-14 authors: Luciano Bestia***  
//!
//! Inside WSL2 I want to open a folder in VSCode. But VSCode is in Win10.  
//! This is useful when navigating in the WSL2 Debian file system.  
//! It used to work in WSL1 like this:  
//!
//! ```bash
//! code .
//! ```
//!
//! but it does not work anymore in WSL2.  
//! Let's make a rust utility for that tiny problem.  
//!
//! ## Open folder in Win10 VSCode
//!
//! I prepared a symbolic link to the `code.exe` in windows.
//!
//! ```bash
//! sudo ln -s "/mnt/c/Users/Luciano/AppData/Local/Programs/Microsoft VS Code/Code.exe" /usr/bin/vscode_in_win
//! # now I can open a folder like this
//! vscode_in_win --remote wsl+Debian ~/rustprojects/qvs20
//! ```
//!
//! Sadly I cannot use the `.` as an argument. I don't know why.  
//! I will transform the `.` into the Linux path for the `env::current_dir()`.  
//!
//! ## Development
//!
//! This is a simple binary. For good habit I separated the lib from the bin.  
//! To build use this:  
//!
//! ```bash
//! cargo build --release
//! # experimentally using strip to make the binary 10x smaller:
//! strip target/release/wsl_open_vscode
//! ```
//!
//! Maybe the file needs to be marked as executable, I am not sure:
//!
//! ```bash
//! chmod a+x target/release/wsl_open_vscode
//! ```
//!
//! Copy the file `target/release/wsl_open_vscode` to `/usr/bin` and change the name to simply `code`.  
//!
//! ```bash
//! sudo cp target/release/wsl_open_vscode /usr/bin/code
//! ```
//!
//! Now we can open the browser like this:  
//!
//! ```bash
//! cd ~/rustprojects/qvs20
//! code .
//! ```
//!
// endregion: auto_md_to_doc_comments include README.md A //!

use std::env;
use unwrap::unwrap;

pub fn open_vscode(arg_1: &str) {
    // absolute Linux path
    let mut arg = arg_1.to_string();
    if arg_1 == "." {
        //linux pwd working directory
        let path = unwrap!(env::current_dir());
        arg = path.display().to_string();
    }

    // call the symbolic link for vscode
    // sudo ln -s "/mnt/c/Users/Luciano/AppData/Local/Programs/Microsoft VS Code/Code.exe" /usr/bin/vscode_in_win
    // /usr/bin/vscode_in_win
    println!("/usr/bin/vscode_in_win --remote wsl+Debian {}", &arg);
    unwrap!(unwrap!(std::process::Command::new("/usr/bin/vscode_in_win")
        .arg("--remote")
        .arg("wsl+Debian")
        .arg(arg)
        .spawn())
    .wait());
}
