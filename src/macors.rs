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
        if let Some(value) = node.get_attr("src") {
            let contents = load_file(&ctx, &value);
            let embeded_contents = node
                .get_children()
                .into_iter()
                .map(|x| {x.to_html_str(0)})
                .collect::<Vec<_>>()
                .join("\n");
            let contents = contents.replace("<content></content>", &embeded_contents);
            let contents = Node::parse_str(&contents).into_fragment();
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
                &[Node::new_text(&format!("$${}$$", "xxx"))]
            )
        };
        let inline_latex = |value: String| {
            Node::new_element(
                "span",
                HashMap::new(),
                &[Node::new_text(&format!("\\({}\\)", "..."))]
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
        }
        if let Some(value) = node.get_text() {
            let new_node = if node.has_attr("block") {
                block_latex(value)
            } else {
                inline_latex(value)
            };
            *node = new_node;
        }
    })
}


pub fn markdown_tag(ctx: &Context) -> Rc<dyn Fn(&mut Node)> {
    let ctx = ctx.clone();
    Rc::new(move |node: &mut Node| {
        if !node.is_tag("markdown") {return}
        let mut children = Vec::<Node>::new();
        for child in node.get_children() {
            if let Some(value) = child.get_text() {
                children.push(crate::utils::compile_markdown(
                    crate::utils::trim_indent(&value)
                ));
            } else {
                children.push(child.clone());
            }
        }
        *node = Node::Fragment(children);
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
    let mut children = Vec::<Node>::new();
    for child in nodes {
        if let Some(value) = child.get_text() {
            children.push(crate::utils::compile_markdown(
                crate::utils::trim_indent(&value)
            ));
        } else {
            children.push(child.clone());
        }
    }
    children
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
