use super::logger;
use std::{fs::{self, read_dir}, path::{Path, PathBuf}, process::Command, str::FromStr};

use serde::{Deserialize};
use std::ffi::{OsString};
#[derive(Deserialize)]
pub struct Config {
    pub tool: Tool,
    pub bin: Vec<Box<Bin>>
}

#[derive(Deserialize)]
pub struct Bin {
    pub name:String,
    pub path:String,

}
#[derive(Deserialize)]
pub struct Tool {
    pub gcc: Option<Gcc>
}
pub trait Compiler {
    fn compile_obj(&self, src: &Path, dst: &Path);
    fn link_dll(&self, objs: &Vec<PathBuf>, out_path: &Path);
    fn link_lib(&self, objs: &Vec<PathBuf>, out_path: &Path);
    fn link_bin(&self, objs: &Vec<PathBuf>, out_path: &Path);
}

#[derive(Deserialize)]
pub struct Gcc {
    bin: String,
}

impl Compiler for Gcc {
    fn compile_obj(&self, src: &Path, dst: &Path) {
        let bin = self.bin.clone();
        let mut cmd = Command::new(bin);
        cmd.arg("-c")
        .arg(src)
        .arg("-o")
        .arg(dst);
        cmd.output().expect("error occurs while gcc compiling");
    }

    fn link_bin(&self, objs: &Vec<PathBuf>, out_path: &Path) {
        let mut cmd = Command::new(self.bin.clone());
        cmd
        .args(objs)
        .arg("-o").arg(out_path);

        cmd.output().expect("error occurs while gcc linking");
    }

    fn link_dll(&self, objs: &Vec<PathBuf>, out_path: &Path) {
        let mut cmd = Command::new(self.bin.clone());
        cmd
        .args(objs)
        .arg("-o").arg(out_path);

        cmd.output().expect("error occurs while gcc linking");
    }

    fn link_lib(&self, objs: &Vec<PathBuf>, out_path: &Path) {
        let mut cmd = Command::new(self.bin.clone());
        cmd
        .args(objs)
        .arg("-o").arg(out_path);

        cmd.output().expect("error occurs while gcc linking");
    }
}

pub struct Mod {
    name: OsString,
    submods:Vec<Box<Mod>>,
    srcs:Vec<OsString>,
    tests:Vec<OsString>
}

impl Mod {
    fn new(name:OsString) -> Self {
        Self {
            name,
            submods: Vec::new(),
            tests: Vec::new(),
            srcs: Vec::new(),
        }
    }

    fn index(&mut self, path_buf: &mut PathBuf) {
        path_buf.push(self.name.clone());
        for entry in read_dir(&path_buf).unwrap() {
            let path = entry.unwrap().path();
            let filename = path.file_name().unwrap().to_os_string();
            if path.is_dir() {
                
                let info = format!("Indexing submod {}", path.to_str().unwrap());
                logger::info("Indexing", info.as_str());

                let mut submod = Mod::new(filename);
                submod.index(path_buf);
                self.submods.push(Box::new(submod));
            }
            else if path.ends_with("test.c") {

                let info = format!("Indexing test {}", path.to_str().unwrap());
                logger::info("Indexing", info.as_str());

                self.tests.push(filename);
            }
            else if path.extension().unwrap()=="c" {

                let info = format!("Indexing sourcefile {}", path.to_str().unwrap());
                logger::info("Indexing", info.as_str());

                self.srcs.push(filename);
            }
        }
        path_buf.pop();
    }

    fn build_obj<C:Compiler>(
        &self, 
        builder_path_buf: &mut PathBuf, 
        src_path_buf: &mut PathBuf,
        compiler: &C) 
    {
        builder_path_buf.push(self.name.clone());
        src_path_buf.push(self.name.clone());
        fs::create_dir(&builder_path_buf).unwrap_or(());
        for submod in &self.submods {
            let info = format!(
                "Building submod from {} to {}", 
                src_path_buf.to_str().unwrap(), 
                builder_path_buf.to_str().unwrap()
            );
            logger::info("Compiling", info.as_str());
            submod.build_obj(builder_path_buf, src_path_buf, compiler);
        }
        for src in &self.srcs {

            let mut objname = src.clone();
            objname.push(".obj");
            src_path_buf.push(src.clone());
            builder_path_buf.push(objname);

            let info = format!(
                "Building source file from {} to {}", 
                src_path_buf.to_str().unwrap(), 
                builder_path_buf.to_str().unwrap()
            );
            logger::info("Compiling", info.as_str());

            compiler.compile_obj(&src_path_buf, &builder_path_buf);
            builder_path_buf.pop();
            src_path_buf.pop();
        }
        src_path_buf.pop();
        builder_path_buf.pop();
    }
}

pub fn collect_objs(path_buf: &PathBuf, targets:&mut Vec<PathBuf>) {
    for entry in read_dir(&path_buf).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            // let filename = path.file_name().unwrap().to_os_string();
            collect_objs(&path, targets);
        }
        else if path.extension().unwrap()=="obj" {
            targets.push(path);
        }
    }
}

pub fn index_lib() -> Result<Mod, ()>{
    let mut lib = Mod::new(OsString::from(""));
    logger::info("Indexing", "index ./src/*");
    let mut src_path_buf = PathBuf::from_str("./src").unwrap();
    lib.index(&mut src_path_buf);
    return Ok(lib);
}

pub fn build_obj_by<C:Compiler>(compiler: &C, module: &Mod ) { 
    let mut builder_path_buf = PathBuf::from_str("./build").unwrap();
    builder_path_buf.push("debug");
    fs::create_dir(&builder_path_buf).unwrap_or(());

    logger::info("Compiling", "ready to compile");
    builder_path_buf.push("obj");
    fs::create_dir(&builder_path_buf).unwrap_or(());
    logger::info("Compiling", "start to compile");
    let mut src_path_buf = PathBuf::from_str("./src").unwrap();
    module.build_obj(&mut builder_path_buf, &mut src_path_buf, compiler);
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
