use std::path::{PathBuf, Path};
use std::convert::AsRef;
pub mod detect_indent;
use crate::data::Either;

pub fn is_inline_tag(tag: &str) -> bool {
    if (tag == "a") {return true}
    if (tag == "abbr") {return true}
    if (tag == "audio") {return true}
    if (tag == "b") {return true}
    if (tag == "bdo") {return true}
    if (tag == "br") {return true}
    if (tag == "button") {return true}
    if (tag == "canvas") {return true}
    if (tag == "cite") {return true}
    if (tag == "code") {return true}
    if (tag == "command") {return true}
    if (tag == "data") {return true}
    if (tag == "datalist") {return true}
    if (tag == "dfn") {return true}
    if (tag == "em") {return true}
    if (tag == "embed") {return true}
    if (tag == "i") {return true}
    if (tag == "iframe") {return true}
    if (tag == "img") {return true}
    if (tag == "input") {return true}
    if (tag == "kbd") {return true}
    if (tag == "keygen") {return true}
    if (tag == "label") {return true}
    if (tag == "mark") {return true}
    if (tag == "math") {return true}
    if (tag == "meter") {return true}
    if (tag == "noscript") {return true}
    if (tag == "object") {return true}
    if (tag == "output") {return true}
    if (tag == "picture") {return true}
    if (tag == "progress") {return true}
    if (tag == "q") {return true}
    if (tag == "ruby") {return true}
    if (tag == "samp") {return true}
    if (tag == "script") {return true}
    if (tag == "select") {return true}
    if (tag == "small") {return true}
    if (tag == "span") {return true}
    if (tag == "strong") {return true}
    if (tag == "sub") {return true}
    if (tag == "sup") {return true}
    if (tag == "svg") {return true}
    if (tag == "textarea") {return true}
    if (tag == "time") {return true}
    if (tag == "var") {return true}
    if (tag == "video") {return true}
    if (tag == "wbr") {return true}
    false
}

pub fn is_header_tag(tag: &str) -> bool {
    tag == "h1" ||
    tag == "h2" ||
    tag == "h3" ||
    tag == "h4" ||
    tag == "h5" ||
    tag == "h6"
}


pub fn get_indent_level(source: &str) -> usize {
    detect_indent::detect_indent(source).amount()
}

pub fn get_indent_str(source: &str) -> String {
    detect_indent::detect_indent(source).indent().to_owned()
}

pub fn trim_indent(source: &str) -> String {
    let level = get_indent_str(source);
    source
        .lines()
        .map(|line| {
            line.strip_prefix(&level).unwrap_or(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn load_file(ctx: &crate::data::Context, path: &str) -> String {
    use std::path::PathBuf;
    let source_dir = ctx.source_dir();
    let root_dir = ctx.root_dir.clone();
    let source_path = root_dir.join(source_dir.join(path));
    read_file_or_panic(source_path)
}


pub fn read_file_or_panic<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref().to_owned();
    if !path.exists() {
        eprintln!("missing file {:?}", path);
        panic!()
    }
    let contents = std::fs::read(&path).unwrap();
    String::from_utf8(contents).unwrap()
}

