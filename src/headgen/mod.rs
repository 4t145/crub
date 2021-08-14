mod query;
use std::{/* ffi::OsStr, */ fs, io::Write, path};
use super::utils::banner;
use tree_sitter::{Node, Parser, Query, QueryCursor};
use regex::Regex;

// overwrite
pub fn gen(src_path: &path::Path, dst_path: &path::Path, parser: &mut Parser) {
    let srcfile = fs::read(src_path).unwrap();
    let tree = parser.parse(&srcfile, None).unwrap();
    let language = parser.language().unwrap();
    
    let default_text_callback = |_node:Node|[];
    let mut head_handle = fs::File::create(dst_path).unwrap();
    let mut write_buf = String::with_capacity(80);
    
    let private_regex = Regex::new(r"^\**_").unwrap();
    {
        let query = Query::new(language, query::CONST_QUERY).unwrap();
        let mut cursor = QueryCursor::new();
        let exs = cursor
        .captures(&query, tree.root_node(), default_text_callback).step_by(3);


        head_handle.write(banner("CONSTS").as_bytes()).unwrap();
        
        for (matched, _) in exs {

            let range_type = matched.captures[1].node.byte_range();
            let text_type = std::str::from_utf8(&srcfile[range_type]).unwrap();
            // dbg!(text_type);

            let range_id = matched.captures[2].node.byte_range();
            let text_id = std::str::from_utf8(&srcfile[range_id]).unwrap();
            // dbg!(text_id);

            if private_regex.is_match(text_id) {continue;}
            write_buf.push_str("\nconst ");
            write_buf.push_str(text_type);
            write_buf.push_str(" ");
            write_buf.push_str(text_id);
            write_buf.push_str(";\n");
            head_handle.write(write_buf.as_bytes()).unwrap();
            write_buf.clear();
        }
    }
    {
        let struct_type_query = Query::new(language, query::STRUCT_TYPE_QUERY).unwrap();
        let mut cursor_stype = QueryCursor::new();
        let stypes = cursor_stype
        .captures(&struct_type_query, tree.root_node(), default_text_callback)
        .step_by(2);

        head_handle.write(banner("STRUCT TYPES").as_bytes()).unwrap();
        for (matched, _) in stypes {
            // dbg!(matched.captures);

            let range_type = matched.captures[0].node.byte_range();
            let text_type = std::str::from_utf8(&srcfile[range_type]).unwrap();

            let range_dec = matched.captures[1].node.byte_range();
            let text_dec = std::str::from_utf8(&srcfile[range_dec]).unwrap();

            if private_regex.is_match(text_dec) {continue;}
            write_buf.push_str("\ntypedef struct ");
            write_buf.push_str(text_type);
            write_buf.push_str(" ");
            write_buf.push_str(text_dec);
            write_buf.push_str(";\n");
            head_handle.write(write_buf.as_bytes()).unwrap();
            write_buf.clear();
        }
    }
    {
        let fn_query = Query::new(language, query::FN_QUERY).unwrap();
        let mut cursor_fn = QueryCursor::new();
        let fns = cursor_fn
        .captures(&fn_query, tree.root_node(), default_text_callback)
        .step_by(1);
        
        head_handle.write(banner("FUNCTIONS").as_bytes()).unwrap();
        for (matched, _) in fns {
            // dbg!("{:?}", matched.captures);
            if matched.captures.len()!=2 {
                continue;
            }

            let range_name = matched.captures[0].node.byte_range();
            let text_name = std::str::from_utf8(&srcfile[range_name]).unwrap();

            let range_type = matched.captures[1].node.byte_range();
            let text_type = std::str::from_utf8(&srcfile[range_type]).unwrap();

            if private_regex.is_match(text_type) {continue;}
            write_buf.push_str("\n");
            write_buf.push_str(text_name);
            write_buf.push_str(" ");
            write_buf.push_str(text_type);
            write_buf.push_str(";\n");

            head_handle.write(write_buf.as_bytes()).unwrap();
            write_buf.clear();
        }
    }

}