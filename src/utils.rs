pub mod detect_indent;

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


pub fn compile_markdown(source: String) -> crate::data::Node {
    let html_str = {
        use comrak::{markdown_to_html, ComrakOptions};
        let mut options = ComrakOptions::default();
        options.render.unsafe_ = true;
        options.render.unsafe_ = true;
        let out = markdown_to_html(&source, &options);
        out
    };
    crate::data::Node::parse_str(&html_str)
}