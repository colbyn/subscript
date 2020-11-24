use std::rc::Rc;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::convert::AsRef;

use crate::data::*;
use crate::utils::load_file;

///////////////////////////////////////////////////////////////////////////////
// MACROS
///////////////////////////////////////////////////////////////////////////////

pub fn include_tag(ctx: &Context) -> Macro {
    let ctx = ctx.clone();
    Macro::match_tag("include", Rc::new(move |node: &mut Node| {
        let source_dir = ctx.source_dir();
        let root_dir = ctx.root_dir.clone();
        if let Some(src_path) = node.get_attr("src") {
            let contents = load_file(&ctx, &src_path);
            let embeded_contents = Node::Fragment(node.get_children()).to_html_str(0);
            let contents = contents.replace("<content></content>", &embeded_contents);
            *node = Node::parse_str(&contents);
        }
    }))
}

pub fn latex_suit(ctx: &Context) -> Macro {
    let ctx = ctx.clone();
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
    Macro::new_void("latex-suit", Rc::new({
        let ctx = ctx.clone();
        move |node: &mut Node| -> Option<()> {
            match node.tag()?.as_ref() {
                /// External File (Block)
                "tex" if node.has_attr("src") => {
                    let src = node.get_attr("src").unwrap();
                    let value = load_file(&ctx, &src);
                    let new_node = block_latex(value);
                    *node = new_node;
                    Some(())
                }
                /// LaTeX Math Block
                "texblock" | "tex" if node.has_attr("block") => {
                    let text_contents = node.get_text_contents()?;
                    let new_node = block_latex(text_contents);
                    *node = new_node;
                    Some(())
                },
                /// LaTeX Inline Math
                "tex" if !node.has_attr("block") => {
                    let text_contents = node.get_text_contents()?;
                    let new_node = inline_latex(text_contents);
                    *node = new_node;
                    Some(())
                },
                /// LaTeX Equation (Block)
                "equation" => {
                    let text_contents = node.get_text_contents()?;
                    let new_node = block_latex(format!(
                        "\\begin{{equation}}\n\\begin{{split}}\n{txt}\n\\end{{split}}\n\\end{{equation}}",
                        txt=text_contents
                    ));
                    *node = new_node;
                    Some(())
                },
                _ => Some(())
            }
        }
    }))
}

pub fn note_tag(ctx: &Context) -> Macro {
    let ctx = ctx.clone();
    Macro::new_void("note", Rc::new(move |node: &mut Node| {
        if !node.is_tag("note") {return Some(())}
        let mut attrs = HashMap::new();
        attrs.insert(String::from("note"), String::new());
        *node = Node::new_element(
            "div",
            attrs,
            &node.get_children(),
        );
        Some(())
    }))
}

pub fn items_tag(ctx: &Context) -> Macro {
    let ctx = ctx.clone();
    Macro::match_tag("items", Rc::new(|node: &mut Node| {
        let mut new_children = Vec::<Node>::new();
        for child in node.get_children() {
            if child.is_tag("li") {
                new_children.push(child);
            } else {
                new_children.push(
                    Node::new_element("li", Default::default(), &[child])
                );
            }
        }
        *node = Node::new_element(
            "ul",
            Default::default(),
            &new_children
        )
    }))
}

pub fn list_tag(ctx: &Context) -> Macro {
    let ctx = ctx.clone();
    Macro::match_tag("list", Rc::new(|node: &mut Node| {
        let mut new_children = Vec::<Node>::new();
        for child in node.get_children() {
            if child.is_tag("li") {
                new_children.push(child);
            } else {
                new_children.push(
                    Node::new_element("li", Default::default(), &[child])
                );
            }
        }
        *node = Node::new_element(
            "ol",
            Default::default(),
            &new_children
        )
    }))
}

