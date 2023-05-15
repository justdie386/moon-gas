use std::{
    path::Path,
    process::{Command, Output},
};

struct Compiler(&'static str);
impl Compiler {
    pub fn compile(&self, file: impl AsRef<Path>) -> std::io::Result<Output> {
        Command::new(self.0).arg("-c").arg(file.as_ref()).arg("-I/usr/include/lua5.1").output()
    }
}

const COMPILER: Compiler = Compiler("cc");

pub fn build(path: String, output: String) {
    if cfg!(unix) {
        for file in path.split(',') {
            if COMPILER.compile(&file).is_ok() {
                println!("{} {}", "compiled ",file);
            }
        }
    }
}