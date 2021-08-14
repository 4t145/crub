use std::{fs, path::{Path, PathBuf}, process::Command, str::FromStr};
use toml;
use serde::{Deserialize};
use super::logger;

#[derive(Deserialize)]
struct Config {
    tool: Tool,
}

#[derive(Deserialize)]
struct Tool {
    gcc: Option<Gcc>
}

#[derive(Deserialize)]
struct Gcc {
    bin: String,
}

impl Gcc {
    fn compile_obj(&self, src: &Path, dst: &Path) {
        let bin = self.bin.as_str().clone();
        let mut cmd = Command::new(bin);
        cmd.arg("-c")
        .arg(src)
        .arg("-o")
        .arg(dst);
        cmd.output().expect("error occurs while gcc compiling");
    }

    fn link(&self, objs: Vec<&Path>, out_path: &Path) {
        let mut cmd = Command::new(self.bin.as_str());
        cmd
        .args(objs)
        .arg("-o").arg(out_path);

        cmd.output().expect("error occurs while gcc linking");
    }
}

pub fn build_by(compiler: &str) -> Result<PathBuf, ()> {
    let mut path = PathBuf::from_str("./").unwrap();
    let mut src_path = PathBuf::from_str("./src").unwrap();
    path.push("./Build.toml");
    logger::info("Building", "reading Build.toml");
    let build_toml = fs::read(&path).unwrap();
    let build_config: Config = toml::from_slice(&build_toml).unwrap();
    path.pop();
    path.push("build");
    path.push("debug");
    fs::create_dir(&path).unwrap();
    match compiler {
        "gcc" => {
            logger::info("Building", "ready to build");
            let gcc = build_config.tool.gcc.unwrap();
            path.push("obj");
            fs::create_dir(&path).unwrap();
            let filename = String::from("lib");
            let src_filename = format!("{}.c",filename);
            let obj_filename = format!("{}.obj",filename);
            
            src_path.push(src_filename);
            path.push(obj_filename);
            gcc.compile_obj(&src_path, &path);
            let lib_obj_path = path.clone();
            path.pop();
            path.pop();

            let exe_filename = format!("{}.exe",filename);
            path.push(exe_filename);
            let exe_path = path.as_path();

            let objs = vec![lib_obj_path.as_path()];
            gcc.link(objs, exe_path);
            logger::info("Building", "building by gcc");
            return Ok(exe_path.to_path_buf()); 
        }
        _ => {Err(())}
    }
    // Command::new().arg("init").output().expect("git init fail!");

}


pub fn excute(path: &Path) {
    logger::ok("starting", "start to run");
    let mut cmd = Command::new(path);
    let res= cmd.output();
    match res {
        Ok(output) => {
            let stdout = String::from_utf8(output.stdout).unwrap();
            println!("\t{}", stdout);
            logger::ok("finished", "excuted finished");

        }
        Err(err) => {
            logger::err("fail", err.to_string().as_str());
        }
    }
}
