use std::path::PathBuf;
const SCRIPT_ROOT:&str = "tree-sitter-c";
fn main() {
    let dir: PathBuf = [SCRIPT_ROOT, "src"].iter().collect();
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile(SCRIPT_ROOT);
}