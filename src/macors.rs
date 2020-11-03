use std::rc::Rc;
use std::path::PathBuf;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::data::*;

pub fn include_tag(ctx: &Context) -> Rc<dyn Fn(&mut Node)> {
    let ctx = ctx.clone();
    Rc::new(move |node: &mut Node| {
        let source_dir = ctx.source_dir();
        let root_dir = ctx.root_dir.clone();
        if !node.is_tag("include") {return}
        if let Some(value) = node.get_attr("src") {
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
            let contents = String::from_utf8(contents).unwrap();
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
            Node::element(
                "div",
                HashMap::new(),
                &[Node::text(&format!("$${}$$", "xxx"))]
            )
        };
        let inline_latex = |value: String| {
            Node::element(
                "span",
                HashMap::new(),
                &[Node::text(&format!("\\({}\\)", "..."))]
            )
        };
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

