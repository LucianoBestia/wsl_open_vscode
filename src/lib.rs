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
