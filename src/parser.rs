pub mod css;

use std::collections::HashMap;
use std::default::Default;
use std::io;
use std::iter::repeat;
use std::string::String;

use html5ever::{
    parse_document,
    parse_fragment,
};
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom as rcdom;
use rcdom::{Handle, NodeData, RcDom};

const REPORT_PARSER_ERRORS: bool = true;

fn convert_impl(handle: &Handle) -> Vec<crate::data::Node> {
    let node = handle;
    match node.data {
        NodeData::Text { ref contents } => {
            let text = escape_default(&contents.borrow());
            let is_empty = text
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.clone())
                .flat_map(|x| {x.lines()})
                .filter(|x| {!x.is_empty()})
                .filter(|x| {x != &"\\n"})
                .collect::<Vec<_>>();
            if is_empty.is_empty() {
                vec![]
            } else {
                // println!("{:?}", text);
                vec![crate::data::Node::Text(text)]
            }
        },
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            let tag = format!("{}", name.local);
            let attrs = attrs
                .borrow()
                .iter()
                .map(|x| {
                    (format!("{}", x.name.local), format!("{}", x.value))
                })
                .collect::<HashMap<_, _>>();
            let children = node
                .children
                .borrow()
                .iter()
                .map(|x| {
                    convert_impl(x)
                })
                .filter(|x| !x.is_empty())
                .flatten()
                .collect::<Vec<_>>();
            // vec![crate::data::Node::Element(Box::new(crate::data::Element {
            //     tag,
            //     attrs,
            //     children,
            // }))]
            vec![crate::data::Node::new_element(
                &tag,
                attrs,
                &children,
            )]
        },
        _ => {
            node
                .children
                .borrow()
                .iter()
                .map(|x| {
                    convert_impl(x)
                })
                .filter(|x| !x.is_empty())
                .flatten()
                .collect::<Vec<_>>()
        }
    }
}

fn convert_root(handle: &Handle, document_mode: bool) -> Vec<crate::data::Node> {
    let result = convert_impl(handle);
    if !document_mode {
        match &result[..] {
            [crate::data::Node::Element(element)] if (element.tag == String::from("html")) => {
                element.children.clone()
            }
            _ => {
                unimplemented!()
            }
        }
    } else {
        result
    }
}

// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    // s.chars().flat_map(|c| c.escape_default()).collect()
    String::from(s)
}

#[derive(Debug, Clone)]
pub struct ParsedResult {
    pub payload: Vec<crate::data::Node>,
    pub errors: Vec<String>,
}


fn tokenizer_config() -> html5ever::tokenizer::TokenizerOpts {
    use markup5ever::{QualName, Namespace, LocalName, Prefix};
    use markup5ever::interface::tree_builder::QuirksMode;
    use html5ever::tokenizer::TokenizerOpts;
    let mut ops: TokenizerOpts = Default::default();
    // ops.exact_errors = false;
    ops
}

fn parser_config() -> html5ever::driver::ParseOpts {
    use markup5ever::{QualName, Namespace, LocalName, Prefix};
    use markup5ever::interface::tree_builder::QuirksMode;
    let mut ops: html5ever::driver::ParseOpts = Default::default();
    ops.tree_builder = {
        let mut tree_build_ops = html5ever::tree_builder::TreeBuilderOpts::default();
        tree_build_ops.exact_errors = false;
        tree_build_ops.scripting_enabled = true;
        tree_build_ops
    };
    ops.tokenizer = tokenizer_config();
    ops
}


pub fn parse_html_str(html_str: &str) -> ParsedResult {
    use std::io::Cursor;
    use markup5ever::{QualName, Namespace, LocalName, Prefix};
    use markup5ever::interface::tree_builder::QuirksMode;
    let mut source = Cursor::new(String::from(html_str));
    let default_env = QualName::new(None, ns!(html), LocalName::from("div"));
    let mut document_mode = {
        html_str.contains("<html>")
    };
    let dom = {
        if document_mode {
            parse_document(
                RcDom::default(),
                parser_config(),
            )
            .from_utf8()
            .read_from(&mut source)
            .unwrap()
        } else {
            parse_fragment(
                RcDom::default(),
                parser_config(),
                default_env,
                Vec::new(),
            )
            .from_utf8()
            .read_from(&mut source)
            .unwrap()
        }
    };
    
    // TRAVERSE
    let mut payload = convert_root(&dom.document, document_mode);

    if !dom.errors.is_empty() {
        if REPORT_PARSER_ERRORS {
            eprintln!("\nParse errors:");
            for err in dom.errors.iter() {
                eprintln!("    {}", err);
            }
        }
    }
    let errors = dom
        .errors
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<_>>();

    ParsedResult{payload, errors}
}






