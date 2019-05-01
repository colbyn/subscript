use std::fmt;
use std::fmt::Debug;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use either::Either;
use serde::{self, Serialize, Deserialize};
use web_sys::console;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure;
use wasm_bindgen::closure::Closure;


#[derive(Debug, Clone)]
pub struct StyleMount {
    pub mount: web_sys::HtmlStyleElement,
}


impl StyleMount {
    pub fn new() -> Self {
        StyleMount {
            mount: mk_raw_style_mount(),
        }
    }
    pub fn delete(&self, node_id: &String) {
        let rules: web_sys::StyleSheet = self.mount.sheet().expect("missing sheet property");
        let rules: wasm_bindgen::JsValue = std::convert::From::from(
            self.mount.sheet().expect("missing sheet property")
        );
        let rules: web_sys::CssStyleSheet = std::convert::From::from(rules);
        let rule_list: web_sys::CssRuleList = rules.css_rules().expect("missing cssRules property");
        for ix in (1..rule_list.length()).map(|x| x - 1).rev() {
            let rule: web_sys::CssRule = rule_list.item(ix).expect("rule index error");
            let rule: wasm_bindgen::JsValue = std::convert::From::from(rule);
            let rule: web_sys::CssStyleRule = std::convert::From::from(rule);
            let selector = rule.selector_text();
            if selector.contains(node_id.as_str()) {
                rules.delete_rule(ix).expect("unable to delete css rule");
            }
        }
    }
    pub fn insert(&self, contents: &String) {
        let rules: web_sys::StyleSheet = self.mount.sheet().expect("missing sheet property");
        let rules: wasm_bindgen::JsValue = std::convert::From::from(
            self.mount.sheet().expect("missing sheet property")
        );
        let rules: web_sys::CssStyleSheet = std::convert::From::from(rules);
        rules.insert_rule(contents.as_str()).expect("failed to insert rule");
    }
}

///////////////////////////////////////////////////////////////////////////////
// INTERNAL UTILS
///////////////////////////////////////////////////////////////////////////////

fn mk_raw_style_mount() -> web_sys::HtmlStyleElement {
    let window: web_sys::Window = web_sys::window()
        .expect("window not available");
    let document = window
        .document()
        .expect("document not available");
    let body: web_sys::Node = std::convert::From::from(
        document.body().expect("document.body not available")
    );
    let style_mount: web_sys::Node = std::convert::From::from(
        document.create_element("style").unwrap()
    );
    {
        let element: JsValue = std::convert::From::from(style_mount.clone());
        let element: web_sys::Element = std::convert::From::from(element);
        element.set_attribute("live-csson-interface", "")
            .expect("setAttribute failed");
    }
    body.append_child(&style_mount);
    let style_mount = {
        let style_mount: wasm_bindgen::JsValue = std::convert::From::from(style_mount);
        let style_mount: web_sys::HtmlStyleElement = std::convert::From::from(style_mount);
        style_mount
    };
    style_mount
}


