use colored::Colorize;
use std::process::Command;



pub fn build(path: String, output: String){
    let  _stage1 = String::from("Compilling ");
    let _status = 0;
    let _check = false;
    let _step = 0;
    if cfg!(unix) {
        for file in path.split(',') {
        let _ = Command::new("cc").arg("-c").arg(file).arg("-I/usr/include/lua5.1").arg("-llua5.1").output().expect("Failed to execute command");
            println!("{} {}", "compiled ".blue().bold(), file);
      }

        let mut compile =  Command::new("cc");
        for name in output.split(','){
            compile.arg(name);
        }
        let _ = compile.arg("-shared").arg("-o").arg("power.so").spawn();
            println!("{} ", "Success".blue().bold());

    }
}