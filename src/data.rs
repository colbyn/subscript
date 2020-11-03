use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::Cow;
use std::collections::HashSet;
use std::path::PathBuf;

use crate::parser;
use crate::macors;


///////////////////////////////////////////////////////////////////////////////
// HELPERS
///////////////////////////////////////////////////////////////////////////////

pub enum Either<L, R> {
    Left(L),
    Right(R),
}


///////////////////////////////////////////////////////////////////////////////
// HTML TREE AST
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element(Box<Element>),
    Text(String),
    Fragment(Vec<Node>),
}

impl Node {
    pub fn to_html_str(&self, indent_level: usize) -> String {
        let level = {
            if indent_level == 0 {
                String::from("")
            } else {
                std::iter::repeat(" ").take(indent_level * 2).collect::<String>()
            }
        };
        match self {
            Node::Element(element) => {
                let attrs = element.attrs
                    .iter()
                    .map(|(key, value)| {
                        format!("{}=\"{}\"", key, value)
                    })
                    .collect::<Vec<_>>();
                let attrs = attrs.join(" ");
                let attrs = {
                    if !element.attrs.is_empty() {
                        format!(" {}", attrs)
                    } else {
                        String::new()
                    }
                };
                let children = element.children
                    .iter()
                    .map(|child| {
                        child.to_html_str(indent_level + 1)
                    })
                    .collect::<Vec<_>>();
                let children = children.join("");
                if element.children.len() == 0 {
                    format!(
                        "{lvl}<{tag}{attrs}></{tag}>",
                        lvl=level,
                        tag=element.tag,
                        attrs=attrs,
                    )
                } else if self.is_inline_node() {
                    format!(
                        "<{tag}{attrs}>{children}</{tag}>",
                        tag=element.tag,
                        attrs=attrs,
                        children=children
                    )
                } else if self.only_inline_children() {
                    format!(
                        "{lvl}<{tag}{attrs}>{children}</{tag}>\n",
                        lvl=level,
                        tag=element.tag,
                        attrs=attrs,
                        children=children
                    )
                } else {
                    format!(
                        "{lvl}<{tag}{attrs}>\n{children}{lvl}</{tag}>\n",
                        lvl=level,
                        tag=element.tag,
                        attrs=attrs,
                        children=children
                    )
                }
            }
            Node::Text(text) => {
                text.clone()
            }
            Node::Fragment(xs) => {
                let children = xs
                    .iter()
                    .map(|child| {
                        child.to_html_str(indent_level)
                    })
                    .collect::<Vec<_>>();
                children.join("\n")
            }
        }
    }
    pub fn into_fragment(self) -> Vec<Node> {
        match self {
            Node::Fragment(xs) => {xs}
            _ => vec![]
        }
    }
    pub fn only_text_children(&self) -> bool {
        self.get_children()
            .into_iter()
            .all(|x| {
                x.is_text()
            })
    }
    pub fn only_inline_children(&self) -> bool {
        self.get_children()
            .into_iter()
            .all(|x| {
                x.is_inline_node()
            })
    }
    pub fn parse_str(html_str: &str) -> Self {
        Node::Fragment(crate::parser::parse_html_str(html_str).payload)
    }
    pub fn apply(&mut self, f: Rc<dyn Fn(&mut Node)>) {
        match self {
            Node::Element(element) => {
                for child in element.children.iter_mut() {
                    child.apply(f.clone());
                }
            }
            Node::Fragment(xs) => {
                for x in xs.iter_mut() {
                    x.apply(f.clone());
                }
            }
            _ => {}
        }
        f(self);
    }
    pub fn tag(&self) -> Option<String> {
        match self {
            Node::Element(element) => Some(element.tag.clone()),
            _ => None
        }
    }
    pub fn is_tag(&self, tag: &str) -> bool {
        self.tag() == Some(String::from(tag))
    }
    pub fn has_attr(&self, key: &str) -> bool {
        match self {
            Node::Element(element) => {
                element.attrs.contains_key(key)
            },
            _ => false
        }
    }
    pub fn get_attr(&self, key: &str) -> Option<String> {
        match self {
            Node::Element(element) => {
                if let Some(key) = element.attrs.get(key) {
                    Some(key.clone())
                } else {
                    None
                }
            },
            _ => None
        }
    }
    pub fn replace_children(&mut self, new_children: Vec<Node>) {
        match self {
            Node::Element(element) => {
                element.children = new_children;
            },
            _ => ()
        }
    }
    pub fn get_children(&self) -> Vec<Node> {
        match self {
            Node::Element(element) => {
                element.children.clone()
            },
            _ => vec![]
        }
    }
    pub fn normalize(self) -> Self {
        match self {
            Node::Element(mut element) => {
                let mut new_children = Vec::<Node>::new();
                for child in element.children.into_iter() {
                    match child {
                        Node::Fragment(mut xs) => {
                            for x in xs {
                                new_children.push(x.normalize())
                            }
                        }
                        node => {
                            new_children.push(node.normalize())
                        }
                    }
                }
                element.children = new_children;
                Node::Element(element)
            }
            Node::Fragment(elements) => {
                let mut new_children = Vec::<Node>::new();
                for child in elements.into_iter() {
                    match child {
                        Node::Fragment(mut xs) => {
                            for x in xs {
                                new_children.push(x.normalize())
                            }
                        }
                        node => {
                            new_children.push(node.normalize())
                        }
                    }
                }
                Node::Fragment(new_children)
            }
            node => node
        }
    }
    pub fn get_text(&self) -> Option<String> {
        match self {
            Node::Text(x) => Some(x.clone()),
            _ => None
        }
    }
    pub fn is_text(&self) -> bool {
        self.get_text().is_some()
    }
    pub fn new_element(
        tag: &str,
        attrs: HashMap<String, String>,
        children: &[Node],
    ) -> Self {
        Node::Element(Box::new(Element{
            tag: String::from(tag),
            attrs,
            children: children.to_owned(),
        }))
    }
    pub fn new_text(value: &str) -> Self {
        Node::Text(String::from(value))
    }
    pub fn is_inline_node(&self) -> bool {
        if self.get_attr("block").is_some() {
            return false;
        }
        match self {
            Node::Element(element) => {
                if crate::utils::is_inline_tag(&element.tag) {
                    return true;
                }
                if element.tag == String::from("tex") {
                    return true;
                }
                false
            },
            Node::Fragment(xs) => {
                xs.iter().all(|x| x.is_inline_node())
            }
            Node::Text(..) => true,
        }
    }
}

impl Default for Node {
    fn default() -> Self {Node::Fragment(vec![])}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub tag: String,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Node>,
}

///////////////////////////////////////////////////////////////////////////////
// COMPILER MEAT
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Context {
    pub source: PathBuf,
    pub root_dir: PathBuf,
}

impl Context {
    pub fn source_dir(&self) -> PathBuf {
        self.source.parent().unwrap().to_owned()
    }
}

///////////////////////////////////////////////////////////////////////////////
// TEST
///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let source = include_str!("../test/test.html");
    let mut html = Node::parse_str(source);
    let ctx = Context{
        source: PathBuf::from("./test/test.html"),
        root_dir: PathBuf::from("./test"),
    };
    html.apply(macors::markdown_tag(&ctx));
    html.apply(macors::include_tag(&ctx));
    html.apply(macors::tex_tag(&ctx));
    html.apply(macors::note_tag(&ctx));
    let mut html = html.normalize();
    println!("{}", html.to_html_str(0))
}



