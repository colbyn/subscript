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
use crate::process::data::*;


///////////////////////////////////////////////////////////////////////////////
// INTERNAL - VIEW MACRO HELPERS
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! markup_argument {
    // SELF METHODS
    ($parent:expr; self.css.merge ($style_node:expr)) => {{
        $parent.merge_style_node($style_node);
    }};
    ($parent:expr; self.append ($children:expr)) => {{
        for child in &($children).clone() {
            $parent.add_child(child.clone());
        }
    }};
    // EXPRESSION
    ($parent:expr; {$value:expr}) => {
        $parent.add_child($value.clone());
    };
    // CSS
    ($parent:expr; @media [$($prop_cond:ident : $value_cond:expr),*] ($($prop:ident : $value:expr)*)) => {{
        let mut media_selectors: Vec<Rule> = Vec::new();
        $(
            media_selectors.push(Rule {
                property: stringify!($prop_cond).to_string(),
                value: $value_cond.to_owned()
            });
        )*
        let mut rules: Vec<Rule> = Vec::new();
        $(
            rules.push(Rule {
                property: stringify!($prop).to_string(),
                value: $value.to_owned()
            });
        )*
        let decl = SelfMediaQueryDeclaration {
            selector: BTreeSet::from_iter(media_selectors.into_iter()),
            rules: rules
        };
        $parent.add_style(Style::SelfMediaQuery(decl));
    }};
    ($parent:expr; : $pseudo_name:ident ($($prop:ident : $value:expr)*)) => {{
        let mut rules: Vec<Rule> = Vec::new();
        $(
            rules.push(Rule {
                property: stringify!($prop).to_string(),
                value: $value.to_owned()
            });
        )*
        let decl = SelfPseudoDeclaration {
            selector: format!(":{}", stringify!($pseudo_name).to_string()),
            rules: rules
        };
        $parent.add_style(Style::SelfPseudoSelector(decl));
    }};
    ($parent:expr; :: $pseudo_name:ident ($($prop:ident : $value:expr)*)) => {{
        let mut rules: Vec<Rule> = Vec::new();
        $(
            rules.push(Rule {
                property: stringify!($prop).to_string(),
                value: $value.to_owned()
            });
        )*
        let decl = SelfPseudoDeclaration {
            selector: format!("::{}", stringify!($pseudo_name).to_string()),
            rules: rules
        };
        $parent.add_style(Style::SelfPseudoSelector(decl));
    }};
    ($parent:expr; $prop:ident : $value:expr) => {{
        $parent.add_style(Style::SelfRule(Rule {
            property: stringify!($prop).to_string(),
            value: $value.to_owned()
        }));
    }};
    // ATTRIBUTES
    ($parent:expr; $key:ident = true) => {{
        $parent.add_attribute(stringify!($key), Either::Left(true));
    }};
    ($parent:expr; $key:ident = false) => {{
        $parent.add_attribute(stringify!($key), Either::Left(false));
    }};
    ($parent:expr; $key:ident = $value:expr) => {{
        $parent.add_attribute(stringify!($key), Either::Right($value.to_owned()));
    }};
    // EVENTS
    ($parent:expr; . $event_name:ident ($body:expr)) => {{
        $parent.add_event(stringify!($event_name), Rc::new($body));
    }};
    // CHILDREN
    ($parent:expr; text ($value:expr)) => {{
        $parent.add_child(HtmlBuild::new_text($value));
    }};
    ($parent:expr; $tag:ident ($($inner_tks:tt)*)) => {{
        let mut child_node = HtmlBuild::new_node(stringify!($tag));
        markup_arguments!(child_node; $($inner_tks)*);
        $parent.add_child(child_node);
    }};
}

#[macro_export]
macro_rules! markup_arguments {
    ($parent:expr;) => {};
    // SELF METHODS
    ($parent:expr; self.css.merge ($body:expr) $($rest:tt)*) => {{
        markup_argument!($parent; self.css.merge ($body));
        markup_arguments!($parent; $($rest)*);
    }};
    ($parent:expr; self.append ($body:expr) $($rest:tt)*) => {{
        markup_argument!($parent; self.append ($body));
        markup_arguments!($parent; $($rest)*);
    }};
    // EXPRESSION
    ($parent:expr; {$value:tt} $($rest:tt)*) => {{
        markup_argument!($parent; {$value});
        markup_arguments!($parent; $($rest)*);
    }};
    // CSS
    ($parent:expr; @media $media_header:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($parent; @media $media_header $body);
        markup_arguments!($parent; $($rest)*);
    }};
    ($parent:expr; $prop:tt : $value:tt $($rest:tt)*) => {{
        markup_argument!($parent; $prop : $value);
        markup_arguments!($parent; $($rest)*);
    }};
    ($parent:expr; : $prop:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($parent; : $prop $body);
        markup_arguments!($parent; $($rest)*);
    }};
    ($parent:expr; :: $prop:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($parent; :: $prop $body);
        markup_arguments!($parent; $($rest)*);
    }};
    // ATTRIBUTES
    ($parent:expr; $key:tt = $value:tt $($rest:tt)*) => {{
        markup_argument!($parent; $key = $value);
        markup_arguments!($parent; $($rest)*);
    }};
    // EVENTS
    ($parent:expr; . $event_name:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($parent; . $event_name $body);
        markup_arguments!($parent; $($rest)*);
    }};
    // CHILDREN
    ($parent:expr; $tag:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($parent; $tag $body);
        markup_arguments!($parent; $($rest)*);
    }};
}


///////////////////////////////////////////////////////////////////////////////
// EXTERNAL - CSS
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! css_impl {
    ($parent:expr;) => {};
    ($parent:expr; @media $media_header:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($parent; @media $media_header $body);
        css_impl!($parent; $($rest)*);
    }};
    ($parent:expr; $prop:tt : $value:tt $($rest:tt)*) => {{
        markup_argument!($parent; $prop : $value);
        css_impl!($parent; $($rest)*);
    }};
    ($parent:expr; : $prop:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($parent; : $prop $body);
        css_impl!($parent; $($rest)*);
    }};
    ($parent:expr; :: $prop:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($parent; :: $prop $body);
        css_impl!($parent; $($rest)*);
    }};
}

#[macro_export]
macro_rules! css {
    ($($x:tt)*) => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        use crate::process::data::*;
        
        let mut node: HtmlBuild<()> = HtmlBuild::new_node("div");
        css_impl!(node; $($x)*);
        let style_node: StyleNode = node
            .unpack_node()
            .expect("macro error - unpack node failed")
            .styling
            .clone();
        style_node
    }};
}



///////////////////////////////////////////////////////////////////////////////
// EXTERNAL - MARKUP
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! markup {
    () => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        use crate::process::data::*;
        
        let mut node = HtmlBuild::new_node("div");
        node
    }};
    ($($path:ident).*|) => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        use crate::process::data::*;
        
        let mut node = HtmlBuild::new_node("div");
        node
    }};
    ($($tag:ident).*| $($x:tt)*) => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::api::macros::*;
        use crate::tree::offline::data::*;
        use crate::process::data::*;
        
        let mut nodes = Vec::new();
        let tags: &str = stringify!($($tag)*);
        for tag in tags.split(" ") {
            nodes.push(HtmlBuild::new_node(tag));
        }
        let mut last = nodes
            .pop()
            .expect("must not be empty");
        nodes.reverse();
        
        
        if nodes.is_empty() {
            let tag = stringify!($($tag)*).trim();
            let mut node = HtmlBuild::new_node(tag);
            markup_arguments!(node; $($x)*);
            node
        } else {
            markup_arguments!(last; $($x)*);
            let nodes = nodes.iter_mut().fold(last.clone(), move |l, r| {
                r.add_child(l);
                r.clone()
            });
            nodes
        }
    }};
    ($($x:tt)*) => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        use crate::process::data::*;
        
        let mut node = HtmlBuild::new_node("div");
        markup_arguments!(node; $($x)*);
        node
    }};
}

pub fn dev() {
    #[derive(Debug, Clone)]
    pub enum AppMsg {
        NoOp,
    }
    
    let x: HtmlBuild<AppMsg> = markup!(nav.ul|
        display: "flex"
        @media [min_width: "900px"] (
            
        )
        .click(move |_| AppMsg::NoOp)
        li(
            :hover (
                
            )
            display: "flex"
            color: "#000"
            a(
                href="/"
                text("Hello World")
            )
        )
        li(
            display: "flex"
            color: "#000"
            a(
                href="/"
                text("Hello World")
            )
        )
    );
}


