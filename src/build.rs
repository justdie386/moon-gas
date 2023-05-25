use colored::Colorize;
use std::process::Command;
use std::thread::sleep;
use lazy_static::lazy_static;
use std::time::Duration;
use std::sync::RwLock;

lazy_static! {
    static ref VERSION: RwLock<String> = RwLock::new(String::new());
}

pub fn set_version(version: String) {
    let mut write_lock = VERSION.write().unwrap();
    *write_lock = version;
}
pub fn build(path: String, output: String, flag: String) {
    let read_lock = VERSION.read().unwrap();
    println!("Version: {}", *read_lock);
    let lua_link = String::from("-I/usr/include/");
    let lua_link = lua_link + &*read_lock;
    let lua_link2 = String::from("-llua");
    let lua_link2 = lua_link2 + &*read_lock;
    let lua_location = String::from("/usr/share/lua/");
    let lua_location = lua_location + &*read_lock;
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
                .arg(lua_link.clone())
                .arg(lua_link2.clone())
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
            .arg(lua_link2.clone())
            .arg("-o")
            .arg(output.clone())
            .arg(lua_link.clone())
            .spawn();
        
        println!("{} ", "Success".blue().bold());
        sleep(Duration::from_millis(200));
        if let Some(folder) = output.find('/') {
            let file = &output[..folder];
            let _ = Command::new("cp")
            .arg("-r")
            .arg(file)
            .arg(lua_location.clone())
            .spawn();
        } else {
            let _ = Command::new("cp")
            .arg("-r")
            .arg(output.clone())
            .arg(lua_location.clone())
            .spawn(); 
        }

        println!("{} {}", "Installed to lua at".blue().bold(), lua_location.clone().blue());
    }
}
