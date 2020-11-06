use std::rc::Rc;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::convert::AsRef;

use crate::data::*;

///////////////////////////////////////////////////////////////////////////////
// MACROS
///////////////////////////////////////////////////////////////////////////////

pub fn include_tag(ctx: &Context) -> Rc<dyn Fn(&mut Node)> {
    let ctx = ctx.clone();
    Rc::new(move |node: &mut Node| {
        let source_dir = ctx.source_dir();
        let root_dir = ctx.root_dir.clone();
        if !node.is_tag("include") {return}
        if let Some(src_path) = node.get_attr("src") {
            let contents = load_file(&ctx, &src_path);
            let embeded_contents = node
                .get_children()
                .into_iter()
                .map(|x| {x.to_html_str(0)})
                .collect::<Vec<_>>()
                .join("");
            let contents = contents.replace("<content></content>", &embeded_contents);
            let contents = html_or_markdown_from_text(contents);
            *node = Node::Fragment(contents);
        }
    })
}

pub fn tex_tag(ctx: &Context) -> Rc<dyn Fn(&mut Node)> {
    let ctx = ctx.clone();
    Rc::new(move |node: &mut Node| {
        if !node.is_tag("tex") {return}
        let block_mode = node.get_attr("block").is_some();
        let block_latex = |value: String| {
            Node::new_element(
                "div",
                HashMap::new(),
                &[Node::new_text(&format!("$${}$$", value))]
            )
        };
        let inline_latex = |value: String| {
            Node::new_element(
                "span",
                HashMap::new(),
                &[Node::new_text(&format!("\\({}\\)", value))]
            )
        };
        if let Some(src) = node.get_attr("src") {
            let value = load_file(&ctx, &src);
            let new_node = if node.has_attr("block") {
                block_latex(value)
            } else {
                inline_latex(value)
            };
            *node = new_node;
        } else if let Some(value) = node.get_text_contents() {
            let new_node = if node.has_attr("block") {
                block_latex(value)
            } else {
                inline_latex(value)
            };
            *node = new_node;
        }
    })
}

pub fn note_tag(ctx: &Context) -> Rc<dyn Fn(&mut Node)> {
    let ctx = ctx.clone();
    Rc::new(move |node: &mut Node| {
        if !node.is_tag("note") {return}
        let mut attrs = HashMap::new();
        attrs.insert(String::from("note"), String::new());
        *node = Node::new_element(
            "div",
            attrs,
            &markdown_children(node.get_children()),
        );
    })
}


///////////////////////////////////////////////////////////////////////////////
// HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn markdown_children(nodes: Vec<Node>) -> Vec<Node> {
    let source = Node::Fragment(nodes).to_html_str(0);
    html_or_markdown_from_text(source)
}

pub fn html_or_markdown_from_text(source: String) -> Vec<Node> {
    let source = source
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");
    let compiled = crate::utils::compile_markdown(
        crate::utils::trim_indent(&source)
    );
    let compiled = compiled.normalize();
    match compiled {
        Node::Fragment(xs) => xs,
        _ => vec![compiled]
    }
}

pub fn load_file(ctx: &Context, value: &str) -> String {
    let source_dir = ctx.source_dir();
    let root_dir = ctx.root_dir.clone();
    let path = {
        if value.starts_with("~/") {
            let value = value.strip_prefix("~/").unwrap_or(&value);
            root_dir.join(PathBuf::from(value))
        } else {
            source_dir.join(PathBuf::from(value))
        }
    };
    let contents = std::fs::read(&path).expect(&format!(
        "missing file {:?}",
        path
    ));
    String::from_utf8(contents).unwrap()
}


pub fn is_markdown_ext(path: &str) -> bool {
    let path = PathBuf::from(path);
    if let Some(ext) = path.extension().and_then(|x| x.to_str()) {
        match ext {
            "md" => true,
            _ => false,
        }
    } else {
        false
    }
}