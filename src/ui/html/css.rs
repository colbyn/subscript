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
use wasm_bindgen::JsValue;
use wasm_bindgen::closure;
use wasm_bindgen::closure::Closure;

use crate::browser::{self, Browser, Callback, console, DomRef};
use crate::ui::dom::style_mount::*;


///////////////////////////////////////////////////////////////////////////////
// STYLING
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Styling {
    id: String,
    rules: Rc<RefCell<Vec<Rule>>>,
    pseudo_classes: Rc<RefCell<Vec<PseudoClass>>>,
}

impl Styling {
    ///////////////////////////////////////////////////////////////////////////
    // INTERNAL
    ///////////////////////////////////////////////////////////////////////////
    fn stringify(&self) -> Vec<String> {
        // SETUP
        let mut results: Vec<String> = Vec::new();
        // RULES
        let rules: Vec<String> = self.rules.borrow()
            .iter()
            .map(|rule| rule.stringify())
            .collect::<Vec<String>>();
        let rules: String = format!(
            "{selector} {{{body}}}",
            selector=self.css_id_selector(),
            body=rules.join(""),
        );
        // PSEUDO-CLASSES
        let mut pseudo_classes: Vec<String> = self.pseudo_classes.borrow()
            .iter()
            .map(|decl| decl.stringify(&self.id))
            .collect::<Vec<String>>();
        // RETURN
        results.push(rules);
        results.append(&mut pseudo_classes);
        results
    }
    
    ///////////////////////////////////////////////////////////////////////////
    // EXTERNAL
    ///////////////////////////////////////////////////////////////////////////
    pub fn new(node_id: String) -> Self {
        Styling {
            id: node_id,
            rules: Rc::new(RefCell::new(Vec::new())),
            pseudo_classes: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub fn css_id_selector(&self) -> String {
        format!("#{id}", id=self.id)
    }
    pub fn add_style(&self, new: Style) {
        match new {
            Style::Rule(x) => self.rules.borrow_mut().push(x),
            Style::PseudoClass(x) => self.pseudo_classes.borrow_mut().push(x),
        }
    }
    pub fn init(&self, live: &StyleMount) {
        for decl in self.stringify() {
            live.insert(&decl);
        }
    }
    pub fn clear(&self, live: &StyleMount) {
        self.rules.borrow_mut().clear();
        self.pseudo_classes.borrow_mut().clear();
        live.delete(&self.id);
    }
    pub fn sync(&self, other: &Styling, live: &StyleMount) {
        let unchanged = self.rules == other.rules && self.pseudo_classes == other.pseudo_classes;
        if !unchanged {
            // CLEAR
            self.clear(live);
            self.rules.replace(
                other.rules.borrow().clone()
            );
            self.pseudo_classes.replace(
                other.pseudo_classes.borrow().clone()
            );
            // SET
            self.init(live);
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
// STYLE
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Style {
    Rule(Rule),
    PseudoClass(PseudoClass),
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct PseudoClass {
    name: String,
    rules: Vec<Rule>
}

impl PseudoClass {
    pub fn css_selector(&self, id: &String) -> String {
        format!("#{id}:{name}", id=id, name=self.name)
    }
    pub fn stringify(&self, id: &String) -> String {
        let decls: String = self.rules.iter().map(|x| x.stringify()).collect::<Vec<String>>().join("");
        format!(
            "{selector} {{{body}}}",
            selector=self.css_selector(id),
            body=decls,
        )
    }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Rule {
    property: String,
    value: String,
}

impl Rule {
    pub fn stringify(&self) -> String {
        format!("{prop}: {value};", prop=self.property, value=self.value)
    }
}

// impl Style {
//     pub fn render_decls(selector: &String, styles: &Vec<Style>) -> String {
//         let mut inner: Vec<String> = Vec::new();
//         for style in styles {
//             match style.render_decl() {
//                 Some(decl) => inner.push(decl),
//                 _ => {}
//             }
//         }
//         format!(
//             "{selector} {{{body}}}",
//             selector=selector,
//             body=inner.join(" "),
//         )
//     }
//     pub fn render_decl(&self) -> Option<String> {
//         match &self {
//             Style::Style{property, value} => {
//                 let property = property.replace("_", "-");
//                 Some(format!(
//                     "{prop}: {value};",
//                     prop=property,
//                     value=value,
//                 ))
//             },
//             Style::PseudoClass(name, body) => None,
//         }
//     }
//     pub fn render_pseudo_selector(&self, node_id: &String) -> Option<String> {
//         match &self {
//             Style::Style{..} => None,
//             Style::PseudoClass(pseudo_name, body) => {
//                 let selector = format!(
//                     "#{id}:{pseudo_name}",
//                     id=node_id,
//                     pseudo_name=pseudo_name,
//                 );
//                 Some(Style::render_decls(&selector, body))
//             },
//         }
//     }
// }


///////////////////////////////////////////////////////////////////////////////
// CSS VALUES
///////////////////////////////////////////////////////////////////////////////


pub trait CssValue {
    fn stringify(&self) -> String;
}

impl CssValue for String {
    fn stringify(&self) -> String {
        self.clone()
    }
}

impl CssValue for &str {
    fn stringify(&self) -> String {
        self.clone().to_owned()
    }
}

pub mod value {
    use super::*;
    
    ///////////////////////////////////////////////////////////////////////////
    // CSS VALUES - COLORS
    ///////////////////////////////////////////////////////////////////////////
    pub fn rgb(r: u32, g: u32, b: u32) -> impl CssValue {
        format!(
            "rgb({r},{g},{b})",
            r=r,
            g=g,
            b=b,
        )
    }

    pub fn hex(x: &str) -> impl CssValue {
        x.to_owned()
    }
}


