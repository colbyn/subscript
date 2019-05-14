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

use crate::process::app::*;
use crate::process::basics::*;
use crate::process::offline::*;
use crate::process::online::*;


///////////////////////////////////////////////////////////////////////////////
// INTERNAL - VIEW MACRO HELPERS
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! markup_argument {
    // INLINE MIXIN EXPRESSIONS
    ($is_svg:expr; $parent:expr; - $mixin:expr ;) => {{
        $parent.merge_mixin($mixin.clone());
    }};
    // EXPRESSION
    ($is_svg:expr; $parent:expr; [$($value:expr),*  $(,)*]) => {{
        let mut xs = Vec::new();
        $(
            xs.push($value.clone());
        )*
        for value in xs {
            $parent.add_child(value);
        }
    }};
    ($is_svg:expr; $parent:expr; {$value:expr}) => {
        $parent.add_child($value.clone());
    };
    // SELF METHODS
    ($is_svg:expr; $parent:expr; self.css.add.if($cond:expr)($style_node:expr)) => {{
        if $cond {
            $parent.merge_style_node($style_node);
        }
    }};
    ($is_svg:expr; $parent:expr; self.add.if($cond:expr)($child:expr)) => {{
        if $cond {
            $parent.add_child($child.clone());
        }
    }};
    ($is_svg:expr; $parent:expr; self.append.if($cond:expr)($children:expr)) => {{
        if $cond {
            for child in &($children).clone() {
                $parent.add_child(child.clone());
            }
        }
    }};
    ($is_svg:expr; $parent:expr; self.css.add ($style_node:expr)) => {{
        $parent.merge_style_node($style_node);
    }};
    ($is_svg:expr; $parent:expr; self.add($child:expr)) => {{
        $parent.add_child($child.clone());
    }};
    ($is_svg:expr; $parent:expr; self.append ($children:expr)) => {{
        for child in &($children).clone() {
            $parent.add_child(child.clone());
        }
    }};
    // CSS
    ($is_svg:expr; $parent:expr; @media [$($prop_cond:ident : $value_cond:expr),*] ($($prop:ident : $value:expr)*)) => {{
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
    ($is_svg:expr; $parent:expr; : $pseudo_name:ident ($($prop:ident : $value:expr)*)) => {{
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
    ($is_svg:expr; $parent:expr; :: $pseudo_name:ident ($($prop:ident : $value:expr)*)) => {{
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
    ($is_svg:expr; $parent:expr; $prop:ident : $value:expr) => {{
        $parent.add_style(Style::SelfRule(Rule {
            property: stringify!($prop).to_string(),
            value: $value.to_owned()
        }));
    }};
    // ATTRIBUTES
    ($is_svg:expr; $parent:expr; $key:ident = true) => {{
        $parent.add_attribute(stringify!($key), Either::Left(true));
    }};
    ($is_svg:expr; $parent:expr; $key:ident = false) => {{
        $parent.add_attribute(stringify!($key), Either::Left(false));
    }};
    ($is_svg:expr; $parent:expr; $key:ident = $value:expr) => {{
        $parent.add_attribute(stringify!($key), $value);
    }};
    // EVENTS
    ($is_svg:expr; $parent:expr; . $event_name:ident ($body:expr)) => {{
        $parent.add_event(stringify!($event_name), Rc::new($body));
    }};
    // CHILDREN
    ($is_svg:expr; $parent:expr; mixin ($($mixins:expr),*)) => {{
        let mut xs = Vec::new();
        $(
            xs.push($mixins);
        )*
        for mixin in xs {
            $parent.merge_mixin(mixin.clone());
        }
    }};
    ($is_svg:expr; $parent:expr; mixin ($mixin:expr)) => {{
        $parent.merge_mixin($mixin.clone());
    }};
    ($is_svg:expr; $parent:expr; component ($value:expr)) => {{
        $parent.add_child(HtmlBuild::new_component($value));
    }};
    ($is_svg:expr; $parent:expr; text ($value:expr)) => {{
        $parent.add_child(HtmlBuild::new_text($value));
    }};
    ($is_svg:expr; $parent:expr; $tag:ident ($($inner_tks:tt)*)) => {{
        let mut child_node = HtmlBuild::new_node(stringify!($tag));
        markup_arguments!($is_svg; child_node; $($inner_tks)*);
        $parent.add_child(child_node);
    }};
}

#[macro_export]
macro_rules! markup_arguments {
    ($is_svg:expr; $parent:expr;) => {};
    // INLINE MIXIN EXPRESSIONS
    ($is_svg:expr; $parent:expr; - $x1:expr;  $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; -$x1;);
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    // CONDITIONALS
    ($is_svg:expr; $parent:expr; if(let $l:pat = $r:expr)($($x:tt)*) $($rest:tt)*) => {{
        if let $l = $r {
            markup_arguments!($is_svg; $parent; $($x)*);
        }
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; if($con:expr)($($x:tt)*) $($rest:tt)*) => {{
        if $con {
            markup_arguments!($is_svg; $parent; $($x)*);
        }
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    // EXPRESSION
    ($is_svg:expr; $parent:expr; [$($args:tt)*] $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; [$($args)*]);
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; {$value:expr} $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; {$value});
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    // SELF METHODS
    ($is_svg:expr; $parent:expr; self.css.add.if($cond:expr)($style_node:expr) $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; self.css.add.if($cond)($style_node));
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; self.add.if($cond:expr)($child:expr) $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; self.add.if($cond)($child));
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; self.append.if($cond:expr)($children:expr) $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; self.append.if($cond)($children));
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; self.css.add ($style_node:expr) $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; self.css.add ($style_node));
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; self.add($child:expr) $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; self.add($child));
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; self.append ($children:expr) $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; self.append ($children));
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    
    // CSS
    ($is_svg:expr; $parent:expr; @media $media_header:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; @media $media_header $body);
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; $prop:tt : $value:tt $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; $prop : $value);
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; : $prop:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; : $prop $body);
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    ($is_svg:expr; $parent:expr; :: $prop:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; :: $prop $body);
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    // ATTRIBUTES
    ($is_svg:expr; $parent:expr; $key:tt = $value:tt $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; $key = $value);
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    // EVENTS
    ($is_svg:expr; $parent:expr; . $event_name:tt $body:tt $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; . $event_name $body);
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
    // CHILDREN
    ($is_svg:expr; $parent:expr; $ident:ident $body:tt $($rest:tt)*) => {{
        markup_argument!($is_svg; $parent; $ident $body);
        markup_arguments!($is_svg; $parent; $($rest)*);
    }};
}


///////////////////////////////////////////////////////////////////////////////
// EXTERNAL - CSS
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! css_impl {
    ($parent:expr;) => {};
    ($parent:expr; @media $media_header:tt $body:tt $($rest:tt)*) => {{
        markup_argument!(false; $parent; @media $media_header $body);
        css_impl!($parent; $($rest)*);
    }};
    ($parent:expr; $prop:tt : $value:tt $($rest:tt)*) => {{
        markup_argument!(false; $parent; $prop : $value);
        css_impl!($parent; $($rest)*);
    }};
    ($parent:expr; : $prop:tt $body:tt $($rest:tt)*) => {{
        markup_argument!(false; $parent; : $prop $body);
        css_impl!($parent; $($rest)*);
    }};
    ($parent:expr; :: $prop:tt $body:tt $($rest:tt)*) => {{
        markup_argument!(false; $parent; :: $prop $body);
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
macro_rules! html {
    () => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
        let mut node = HtmlBuild::new_node("div");
        node
    }};
    ($($path:ident).*|) => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
        let mut node = HtmlBuild::new_node("div");
        node
    }};
    ($($tag:ident).*| $($x:tt)*) => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::macros;
        use crate::tree::offline::data::*;
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
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
            markup_arguments!(false; node; $($x)*);
            node
        } else {
            markup_arguments!(false; last; $($x)*);
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
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
        let mut node = HtmlBuild::new_node("div");
        markup_arguments!(false; node; $($x)*);
        node
    }};
}


///////////////////////////////////////////////////////////////////////////////
// EXTERNAL - MIXINS
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! mixin {
    () => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
        Mixin {
            attributes: BTreeMap::new(),
            events: BTreeMap::new(),
            styling: StyleNode::new(),
            nodes: Vec::new(),
        }
    }};
    ($($x:tt)*) => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
        let mut node = HtmlBuild::new_node("div");
        markup_arguments!(false; node; $($x)*);
        
        if let Some(node) = node.unpack_node_own() {
            Mixin {
                attributes: node.attributes,
                events: node.events,
                styling: node.styling,
                nodes: node.children,
            }
        } else {
            panic!("mixin macro failed")
        }
    }};
}

#[macro_export]
macro_rules! mixin_fn {
    ($name:ident|) => {
        pub fn $name <T: Clone + Debug> () -> Mixin<T> {
            mixin!()
        }
    };
    ($name:ident| $($x:tt)*) => {
        pub fn $name <T: Clone + Debug> () -> Mixin<T> {
            mixin!($($x)*)
        }
    };
}



///////////////////////////////////////////////////////////////////////////////
// EXTERNAL - MARKUP
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! svg {
    () => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
        let mut node = HtmlBuild::new_node("div");
        node
    }};
    ($($path:ident).*|) => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::data::*;
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
        let mut node = HtmlBuild::new_node("div");
        node
    }};
    ($($tag:ident).*| $($x:tt)*) => {{
        use ::either::Either;
        use crate::browser::*;
        use crate::tree::offline::macros;
        use crate::tree::offline::data::*;
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
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
            markup_arguments!(false; node; $($x)*);
            node
        } else {
            markup_arguments!(false; last; $($x)*);
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
        
        use crate::process::app::*;
        use crate::process::basics::*;
        use crate::process::online::*;
        
        let mut node = HtmlBuild::new_node("div");
        markup_arguments!(false; node; $($x)*);
        node
    }};
}





