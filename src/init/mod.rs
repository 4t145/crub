use std::{fs, io::Write, path::{PathBuf}, str::FromStr};
const LIB_C:&str = 
r#"#include "lib.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
    printf("hello world!");
    return 0;
}
"#;

const CRUB_TOML:&str = 
r#"[lib]
name = ""
version = "0.1.0"

[dependencies]
stdio = "*"
"#;

const BUILD_TOML:&str = 
r#"[tool.gcc]
bin = "gcc"
"#;

pub fn init() {
    
    fs::create_dir("./src").unwrap();
    fs::create_dir("./include").unwrap();
    fs::create_dir("./build").unwrap();
    let mut path = PathBuf::from_str("./").unwrap();

    path.push("Crub.toml");
    let mut crub_toml = fs::File::create(&path).unwrap();
    crub_toml.write(&CRUB_TOML.as_bytes()).unwrap();
    path.pop();

    path.push("Build.toml");
    let mut crub_toml = fs::File::create(&path).unwrap();
    crub_toml.write(&BUILD_TOML.as_bytes()).unwrap();
    path.pop();

    path.push("src");
    path.push("lib.c");
    let mut lib_c = fs::File::create(&path).unwrap();
    lib_c.write(&LIB_C.as_bytes()).unwrap();
    path.pop();
    path.push("lib.h");
    let mut _lib_h = fs::File::create(&path).unwrap();

    path.pop();

}

pub fn git_init() {
    use std::process::Command;
    Command::new("git").arg("init").output().expect("git init fail!");
}