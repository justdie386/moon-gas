use colored::Colorize;
use std::process::Command;
use std::thread::sleep;
use lazy_static::lazy_static;
use std::time::Duration;
use std::sync::RwLock;

lazy_static! {
    static ref VERSION: RwLock<String> = RwLock::new(String::new());
}

pub fn setVersion(version: String) {
    let mut write_lock = VERSION.write().unwrap();
    *write_lock = version;
}
pub fn build(path: String, output: String, flag: String) {
    let read_lock = VERSION.read().unwrap();
    println!("Version: {}", *read_lock);
    let mut rm = Command::new("rm");
    let mut compile = Command::new("cc");
    let mut ocompile: Command = Command::new("cc");
    if flag.is_empty() != true {
        for args in flag.split(' ') {
            compile.arg(args);
            ocompile.arg(args);

        }
    }
    if cfg!(unix) {
                for file in path.split(',') {
            ocompile
                .arg("-c")
                .arg(file)
                .arg("-I/usr/include/lua5.1")
                .arg("-llua5.1")
                .output()
                .expect("Failed to execute command");
            println!("{} {}", "compiled ".blue().bold(), file);
                let parts: Vec<&str> = file.split('/').collect();
                if let Some(name) = parts.last() {
                    let mut fullname = String::from(file);
                    if fullname.contains(".h") {
                        fullname.push('.');
                        fullname.push('g');
                        fullname.push('c');
                        fullname.push('h');
                    rm.arg(fullname);
                    } else {
                        let mut fullname = String::from(*name);
                        fullname.truncate(name.len() - 1);
                        fullname.push('o');
                        compile.arg(file);
                        rm.arg(fullname); 
                    }
                }
            }
        sleep(Duration::from_millis(600));
        let _ = rm.spawn();
        let _ = compile
            .arg("-shared")
            .arg("-o")
            .arg(output.clone())
            .arg("-I/usr/include/lua5.1")
            .spawn();
        
        println!("{} ", "Success".blue().bold());
        sleep(Duration::from_millis(200));
        if let Some(folder) = output.find('/') {
            let file = &output[..folder];
            let _ = Command::new("cp")
            .arg("-r")
            .arg(file)
            .arg("/usr/share/lua/5.1/")
            .spawn();
        } else {
            let _ = Command::new("cp")
            .arg("-r")
            .arg(output.clone())
            .arg("/usr/share/lua/5.1/")
            .spawn(); 
        }

        println!("{}", "Installed to lua".blue().bold());
    }
}
