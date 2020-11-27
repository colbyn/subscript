use std::rc::Rc;
use std::cell::RefCell;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::convert::AsRef;

use crate::data::*;
use crate::utils::{
    cache_file_dep,
};


///////////////////////////////////////////////////////////////////////////////
// MACROS
///////////////////////////////////////////////////////////////////////////////

pub fn include_tag(ctx: &Context) -> Macro {
    let ctx = ctx.clone();
    Macro::match_tag("include", Rc::new(move |node: &mut Node| {
        let source_dir = ctx.source_dir();
        let root_dir = ctx.root_dir.clone();
        if let Some(src_path_str) = node.get_attr("src") {
            let contents = || -> Option<String> {
                let src_path = FilePath::resolve_include_path(
                    &ctx,
                    &src_path_str,
                )?;
                if !src_path.exists() {
                    let source_dir = ctx.source_dir();
                    eprintln!("MISSING: {}", src_path);
                    eprintln!(" ORIGINAL {}", src_path_str);
                    eprintln!("     SOURCE_DIR {}", source_dir);
                    panic!()
                }
                let base: String = src_path.load_text_file();
                let had_doctype = base.contains("<!DOCTYPE html>");
                let mut base = Node::parse_str(&base);
                // Provision the new document:
                {
                    let mut new_ctx = ctx.clone();
                    new_ctx.source = ctx
                        .source_dir()
                        .join(&ctx.root_dir, &src_path)
                        .unwrap();
                    hooks::document(&new_ctx, &mut base);
                }
                let mut base = base.to_html_str(0);
                if had_doctype {
                    base = format!("<!DOCTYPE html>\n{}", base);
                }
                Some(base)
            };
            let embeded_contents = Node::Fragment(node.get_children()).to_html_str(0);
            let contents = contents()
                .unwrap()
                .replace("<content></content>", &embeded_contents);
            let mut new_node = Node::parse_str(&contents);
            *node = new_node;
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
                    // let src = node.get_attr("src").unwrap();
                    // // let value = load_file(&ctx, &src);
                    // let value = unimplemented!();
                    // let new_node = block_latex(value);
                    // *node = new_node;
                    eprintln!("[unimplemented] tex tags with src link");
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

pub fn img_tag(ctx: &Context) -> Macro {
    let ctx = ctx.clone();
    let processed_attr = "ss.img.processed";
    Macro::match_tag("img", Rc::new(move |node: &mut Node| {
        node.get_attr("src")
            .and_then(|x| FilePath::resolve_include_path(&ctx, &x))
            .map(|src_path| {
                if !node.has_attr(processed_attr) && !ctx.fast_upate_mode {
                    let new_src = cache_file_dep(&ctx, &src_path);
                    node.set_attr("src", format!(
                        "/{}",
                        new_src
                    ));
                    node.set_attr(processed_attr, String::from(""));
                }
            });
    }))
}

pub fn link_tag(ctx: &Context) -> Macro {
    let ctx = ctx.clone();
    let processed_attr = "ss.link.processed";
    Macro::match_tag("link", Rc::new(move |node: &mut Node| {
        node.get_attr("href")
            .and_then(|x| FilePath::resolve_include_path(&ctx, &x))
            .map(|src_path| {
                if !node.has_attr(processed_attr) {
                    let new_src = cache_file_dep(&ctx, &src_path);
                    node.set_attr("href", format!(
                        "/{}",
                        new_src
                    ));
                    node.set_attr(processed_attr, String::from(""));
                }
            });
    }))
}

pub fn element_self_styles(element: &mut Element) {
    let mut set_node_id = false;
    let node_id = {
        if let Some(uid) = element.attrs.get("id") {
            uid.clone()
        } else {
            format!(
                "id_{}",
                rand::random::<u64>()
            )
        }
    };
    for child in element.children.iter_mut() {
        if child.is_tag("style") && child.has_attr("self") {
            if let Some(contents) = child.get_text_contents() {
                let new_contents = contents.replace("self", &format!(
                    "#{}",
                    node_id
                ));
                child.replace_children(vec![Node::new_text(&new_contents)]);
                set_node_id = true;
            }
        }
    }
    if set_node_id {
        element.attrs.insert(String::from("id"), node_id);
    }
}


pub fn subscript_deps(ctx: &Context) -> Macro {
    let ctx = ctx.clone();
    Macro::match_tag("head", Rc::new(move |node: &mut Node| {
        let deps = Node::parse_str(include_str!("../assets/deps.html"));
        node.append_children(deps.into_fragment());
    }))
}

pub fn hoist_style_tags(ctx: &Context, html: &mut Node) {
    let ctx = ctx.clone();
    let style_tags = Rc::new(RefCell::new(Vec::<Node>::new()));
    let ret_macro = Macro::match_tag("style", Rc::new({
        let style_tags = style_tags.clone();
        move |node: &mut Node| {
            let style_tags = style_tags.clone();
            style_tags.borrow_mut().push(node.clone());
            *node = Node::Fragment(Default::default());
        }
    }));
    let hoist_macro = Macro::match_tag("head", Rc::new(move |node: &mut Node| {
        let style_tags: Vec<Node> = style_tags.borrow().clone();
        node.append_children(style_tags);
    }));
    html.apply(ret_macro);
    html.apply(hoist_macro);
}

pub fn table_of_contents(ctx: &Context, html: &mut Node) {
    if ctx.fast_upate_mode {
        return;
    }
    html.eval(Rc::new(|node: &mut Node| {
        if let Some(tag) = node.tag() {
            let mut set_id = || {
                if node.get_attr("id").is_none() {
                    node.set_attr("id", format!(
                        "{}",
                        rand::random::<u64>()
                    ))
                }
            };
            match &tag[..] {
                "h1" => set_id(),
                "h2" => set_id(),
                "h3" => set_id(),
                "h4" => set_id(),
                "h5" => set_id(),
                "h6" => set_id(),
                _ => ()
            }
        }
    }));
    fn runner(node: &Node) -> Vec<Node> {
        let new_entry = |tag: &str, children: String, uid: &String| {
            let mut li_attrs = HashMap::default();
            li_attrs.insert(String::from("for"), String::from(tag));
            let mut a_attrs = HashMap::default();
            a_attrs.insert(String::from("href"), format!(
                "#{}",
                uid
            ));
            let result = Node::new_element(
                "li",
                li_attrs,
                &[Node::new_element(
                    "a",
                    a_attrs,
                    &[Node::new_text(&children)]
                )]
            );
            vec![result]
        };
        match node {
            Node::Element(element) if &element.tag == "h1" => {
                let uid = element.attrs.get("id").unwrap();
                let children = node.get_children_as_text().join(" ");
                new_entry("h1", children, uid)
            }
            Node::Element(element) if &element.tag == "h2" => {
                let uid = element.attrs.get("id").unwrap();
                let children = node.get_children_as_text().join(" ");
                new_entry("h2", children, uid)
            }
            Node::Element(element) if &element.tag == "h3" => {
                let uid = element.attrs.get("id").unwrap();
                let children = node.get_children_as_text().join(" ");
                new_entry("h3", children, uid)
            }
            Node::Element(element) if &element.tag == "h4" => {
                let uid = element.attrs.get("id").unwrap();
                let children = node.get_children_as_text().join(" ");
                new_entry("h4", children, uid)
            }
            Node::Element(element) if &element.tag == "h5" => {
                let uid = element.attrs.get("id").unwrap();
                let children = node.get_children_as_text().join(" ");
                new_entry("h5", children, uid)
            }
            Node::Element(element) if &element.tag == "h6" => {
                let uid = element.attrs.get("id").unwrap();
                let children = node.get_children_as_text().join(" ");
                new_entry("h6", children, uid)
            }
            Node::Element(element) => {
                return element.children.iter().flat_map(|x| runner(x)).collect()
            }
            Node::Fragment(nodes) => {
                nodes
                    .iter()
                    .flat_map(|x| runner(x))
                    .collect()
            }
            _ => Vec::new()
        }
    }
    let headers = runner(html);
    html.eval(Rc::new(move |node: &mut Node| {
        if node.is_tag("toc") {
            let mut attrs = HashMap::default();
            attrs.insert(String::from("toc"), String::default());
            *node = Node::new_element(
                "ul",
                attrs,
                &headers
            );
        }
    }));
}

/// Macro entrypoints.
pub mod hooks {
    use super::*;

    /// Custom elements use the 'document' hook.
    pub fn document(ctx: &Context, html: &mut Node) {
        html.apply(include_tag(&ctx));
        html.apply(items_tag(&ctx));
        html.apply(latex_suit(&ctx));
        html.apply(note_tag(&ctx));
        html.apply(img_tag(&ctx));
        html.apply(link_tag(&ctx));
    }
    /// Apply this once to the entire document **before** serializing such to a string.
    /// This is where e.g. runtime dependencies are inserted.
    pub fn finalize_document(ctx: &Context, html: &mut Node) {
        html.apply(subscript_deps(&ctx));
        hoist_style_tags(&ctx, html);
        table_of_contents(&ctx, html);
    }
    /// Gets called whenever new elements are created (includes elements from the parser).
    pub fn new_element(element: &mut Element) {
        element_self_styles(element);
    }
}

