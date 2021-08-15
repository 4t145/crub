use tree_sitter::{Language, Parser};
use std::{fs, path::{Path, PathBuf}, str::FromStr};
use clap::{App, /* AppSettings, Clap,  */load_yaml};

use crate::build::Compiler;

extern "C" { fn tree_sitter_c() -> Language; }

mod headgen;
mod utils;
mod init;
mod modtool;
mod logger;
mod build;
fn main() {

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(ref matched) = matches.subcommand_matches("init") {

        if !matched.is_present("nogit") {
            init::git_init();  
        }

        // let name = matched.value_of("name").unwrap_or("");
        
        init::init();
        logger::ok("initial", "initialize the lib!!");
    }

    if let Some(ref matched) = matches.subcommand_matches("headgen") {
        let src_path = matched.value_of("src").unwrap_or("./mod.c");
        let dst_path = matched.value_of("dst").unwrap_or("./mod.h");
        let language = unsafe { tree_sitter_c() };
        let mut parser = Parser::new();
        parser.set_language(language).unwrap();
        headgen::gen(
            Path::new(src_path), 
            Path::new(dst_path), 
            &mut parser
        );
    }

    if let Some(ref matched) = matches.subcommand_matches("buildobj") {
        logger::info("Building", "reading Build.toml");
        let build_toml = fs::read("./Build.toml").unwrap();
        let build_config: build::Config = toml::from_slice(&build_toml).unwrap();
        let compiler = matched.value_of("use").unwrap_or("gcc");
        logger::info("Compiling", "compiler choosing");
        let compiler = match compiler {
            "gcc" => build_config.tool.gcc.unwrap(),
            _ => unimplemented!()
        };
        let lib = build::index_lib().unwrap();
        build::build_obj_by(&compiler, &lib);

        // build::excute(executable.as_path());
    }


    if let Some(ref matched) = matches.subcommand_matches("link") {
        logger::info("Building", "reading Build.toml");
        let build_toml = fs::read("./Build.toml").unwrap();
        let build_config: build::Config = toml::from_slice(&build_toml).unwrap();
        let compiler = matched.value_of("use").unwrap_or("gcc");
        logger::info("Compiling", "compiler choosing");
        let compiler = match compiler {
            "gcc" => build_config.tool.gcc.unwrap(),
            _ => unimplemented!()
        };

        let path_buf = PathBuf::from_str("./build/debug/obj").unwrap();
        let mut objs = vec![];
        build::collect_objs(&path_buf, &mut objs);
        let bin = matched.is_present("bin");
        let dll = matched.is_present("dll");
        let lib = matched.is_present("lib");
        if !(bin||dll||lib) || bin {
            let path = Path::new("./build/debug/main.exe");
            compiler.link_bin(&objs, path);
        } 
        if dll {
            let path = Path::new("./build/debug/lib.dll");
            compiler.link_bin(&objs, path);
        }
        if lib {
            let path = Path::new("./build/debug/lib.lib");
            compiler.link_bin(&objs, path);            
        }

        logger::ok("Finished", "linked ALL ");
    }
}
