use tree_sitter::{Language, Parser};
use std::{/* fs,  */path /* io, */ /* str::Matches */};
use clap::{App, /* AppSettings, Clap,  */load_yaml};

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
            path::Path::new(src_path), 
            path::Path::new(dst_path), 
            &mut parser
        );
    }

    if let Some(ref matched) = matches.subcommand_matches("run") {
        let compiler = matched.value_of("use").unwrap_or("gcc");
        let executable = build::build_by(compiler).unwrap();
        build::excute(executable.as_path());
    }
}
