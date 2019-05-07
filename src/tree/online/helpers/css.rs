use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::any::*;
use std::marker::Sized;
use either::Either;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;

use crate::browser::*;
use crate::tree::offline::data::*;
use crate::tree::online::data::*;
use crate::process::data::*;


/// Helper type for rendering into syntactic form.
#[derive(Debug, PartialEq, Clone)]
pub struct GlobalMediaQueries {
    pub entries: HashMap<BTreeSet<Rule>, Vec<(String, String)>>,
}

impl GlobalMediaQueries {
    pub fn new() -> Self {
        GlobalMediaQueries {
            entries: HashMap::new()
        }
    }
    pub fn insert(&mut self, node_id: &String, media_query: SelfMediaQueryDeclaration) {
        let entry: (String, String) = {
            let rules = media_query.rules
                .iter()
                .map(|x| render_rule(x))
                .collect::<Vec<String>>()
                .join("");
            let selector = format!("[{node_id}]", node_id=node_id);
            let body = format!("{{{body}}}", body=rules);
            (selector, body)
        };
        match self.entries.get_mut(&media_query.selector) {
            Some(value) => {
                value.push(entry);
            }
            None => {
                self.entries.insert(media_query.selector.clone(), vec![entry]);
            }
        }
    }
}

pub fn render_style_nodes_tree(nodes: &StyleNodesTree) -> String {
    // SETUP
    let mut global_declarations: Vec<String> = Vec::new();
    let mut global_media_queries = GlobalMediaQueries::new();
    // GO
    for (node_id, node) in nodes.iter() {
        // RULES
        {
            let rules_selector = format!("[{id}]", id=node_id);
            let rules_body: String = format!(
                "{{{body}}}",
                body = node.self_rules
                    .iter()
                    .map(|rule| render_rule(rule))
                    .collect::<Vec<String>>()
                    .join("")
            );
            global_declarations.push(
                render_decl(&rules_selector, &rules_body)
            );
        }
        // SELF PSEUDO-SELECTORS
        {
            let mut self_pseudo_selectors: Vec<(String, String)> = node.self_pseudo_selectors
                .iter()
                .map(|decl| render_self_pseudo_declaration(decl, &node_id))
                .collect::<Vec<(String, String)>>();
            global_declarations.append(
                &mut self_pseudo_selectors
                    .iter()
                    .map(|(k, v)| {render_decl(k, v)})
                    .collect::<Vec<String>>()
            );
        }
        // SELF MEDIA-QUERIES
        node.self_media_queries.iter().for_each(|query| {
            global_media_queries.insert(&node_id, query.clone());
        });
    }
    // FINALIZE MEDIA-QUERIES
    for (header, decls) in global_media_queries.entries {
        let media_selector: String = format!(
            "@media {}",
            header
                .iter()
                .map(|rule| {
                    format!(
                        "({prop}: {value})",
                        prop=rule.property.replace("_", "-"),
                        value=rule.value
                    )
                })
                .collect::<Vec<String>>()
                .join(" and ")
        );
        let decls: String = {
            decls
                .iter()
                .map(|(k, v)| render_decl(k, v))
                .collect::<Vec<String>>()
                .join("\n\t")
        };
        global_declarations.push(format!(
            "{media_selector} {{{body}}}",
            media_selector=media_selector,
            body=decls
        ));
    }
    // DONE
    global_declarations.join("\n")
}

pub fn render_rule(rule: &Rule) -> String {
    format!(
        "{prop}: {value};",
        prop=rule.property.replace("_", "-"),
        value=rule.value
    )
}
pub fn render_decl(selector: &String, body: &String) -> String {
    format!(
        "{selector} {body}",
        selector=selector,
        body=body,
    )
}
pub fn render_self_pseudo_declaration(
    decl: &SelfPseudoDeclaration,
    node_id: &String,
) -> (String, String) {
    let decls: String = decl.rules
        .iter()
        .map(|x| render_rule(x))
        .collect::<Vec<String>>()
        .join("");
    let selector = format!(
        "[{id}]{selector}",
        id=node_id,
        selector=decl.selector
    );
    (selector, format!("{{{body}}}", body=decls))
}
pub fn render_self_media_query_declaration(
    decl: &SelfMediaQueryDeclaration,
    node_id: &String,
) -> (String, String) {
    let decls: String = decl.rules
        .iter()
        .map(|x| render_rule(x))
        .collect::<Vec<String>>()
        .join("");
    let selector: String = format!(
        "@media {}",
        decl.selector
            .iter()
            .map(|rule| {
                format!(
                    "({prop}: {value})",
                    prop=rule.property.replace("_", "-"),
                    value=rule.value
                )
            })
            .collect::<Vec<String>>()
            .join(" and ")
    );
    (
        selector,
        format!(
            "{{
                [{id}] {{{body}}}
            }}",
            id=node_id,
            body=decls
        )
    )
}

