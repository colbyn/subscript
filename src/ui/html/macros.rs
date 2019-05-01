use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use serde::{self, Serialize, Deserialize};
use std::collections::HashMap;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use either::Either;
use wasm_bindgen::JsValue;

use crate::browser::{self, Browser, Callback, console, DomRef};
use crate::ui::html::*;
use crate::ui::html::css::CssValue;


///////////////////////////////////////////////////////////////////////////////
// INTERNAL
///////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! attribute_value {
    ($key:ident, true) => {
        Attribute::Toggle{
            key: String::from(stringify!($key)),
            value: true,
        }
    };
    ($key:ident, false) => {
        Attribute::Toggle{
            key: String::from(stringify!($key)),
            value: false,
        }
    };
    ($key:ident, $val:expr) => {
        Attribute::Pair{
            key: String::from(stringify!($key)),
            value: $val.clone().to_owned(),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! view_argument {
    ///////////////////////////////////////////////////////////////////////////
    // EVENT HANDLER
    ///////////////////////////////////////////////////////////////////////////
    ($node:expr, on . $key:ident = $value:expr) => {
        $node.add_event_handler(
            String::from(stringify!($key)),
            Rc::new($value),
        );
    };
    
    ///////////////////////////////////////////////////////////////////////////
    // ATTRIBUTE
    ///////////////////////////////////////////////////////////////////////////
    ($node:expr, $key:ident = $val:tt) => {
        $node.add_attribute(
            attribute_value!($key, $val).clone()
        );
    };

    ///////////////////////////////////////////////////////////////////////////
    // STYLE
    ///////////////////////////////////////////////////////////////////////////
    // CSS RULE
    ($node:expr, $key:ident : $val:expr) => {
        $node.add_style(Style::Style{
            property: String::from(stringify!($key)),
            value: $val.stringify(),
        });
    };
    // EMPTY PSEUDO-CLASS
    ($node:expr, : $key:ident ()) => {
        $node.add_style(Style::PseudoClass(
            String::from(stringify!($key)),
            Vec::new()
        ));
    };
    // PSEUDO-CLASS
    ($node:expr, : $key:ident $val:tt) => {{
        let mut body: Vec<Style> = Vec::new();
        style_properties_only_arguments!(body, $val);
        $node.add_style(Style::PseudoClass(
            String::from(stringify!($key)),
            body
        ));
    }};
    
    ///////////////////////////////////////////////////////////////////////////
    // CHILDREN
    ///////////////////////////////////////////////////////////////////////////
    // TEXT NODE
    ($node:expr, text $value:expr) => {
        $node.add_child(
            Html::new_text($value.to_owned())
        );
    };
    // EMPTY NODE
    ($node:expr, $key:ident ()) => {
        $node.add_child(
            Html::new_node(String::from(stringify!($key)))
        );
    };
    // NODE
    ($node:expr, $key:ident ($($body:tt)*)) => {{
        let inner = view!($key| $($body)*);
        $node.add_child(inner);
    }};
    ///////////////////////////////////////////////////////////////////////////
    // EXPRESSION
    ///////////////////////////////////////////////////////////////////////////
    ($node:expr, $value:expr) => {
        $node.add_child($value.clone());
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! style_properties_only_arguments {
    ///////////////////////////////////////////////////////////////////////////
    // MANY
    ///////////////////////////////////////////////////////////////////////////
    // CSS RULE
    ($list:expr, $key:ident : $val:expr, $($rest:tt)*) => {
        $list.push(
            Style::Style {
                property: String::from(stringify!($key)),
                value: $val.stringify(),
            }
        );
        style_properties_only_arguments!(
            $list,
            $($rest)*
        );
    };
    
    ///////////////////////////////////////////////////////////////////////////
    // SINGLE
    ///////////////////////////////////////////////////////////////////////////
    // CSS RULE
    ($list:expr, $key:ident : $val:expr) => {
        $list.push(Style::Style {
            property: String::from(stringify!($key)),
            value: $val.stringify(),
        });
    };
    
    ///////////////////////////////////////////////////////////////////////////
    // INTERNAL - UNWRAP NESTED PARENS
    ///////////////////////////////////////////////////////////////////////////
    ($node:expr, ($($x:tt)*)) => {
        style_properties_only_arguments!(
            $node,
            $($x)*
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! view_arguments {
    ///////////////////////////////////////////////////////////////////////////
    // MANY - EVENT HANDLER
    ///////////////////////////////////////////////////////////////////////////
    ($node:expr, on . $key:ident = $value:expr, $($rest:tt)*) => {
        view_argument!($node, on . $key = $value);
        view_arguments!(
            $node,
            $($rest)*
        );
    };
    ///////////////////////////////////////////////////////////////////////////
    // MANY - ATTRIBUTE
    ///////////////////////////////////////////////////////////////////////////
    ($node:expr, $key:ident = $val:tt, $($rest:tt)*) => {
        view_argument!($node, $key = $val);
        view_arguments!(
            $node,
            $($rest)*
        );
    };
    ///////////////////////////////////////////////////////////////////////////
    // MANY - CSS
    ///////////////////////////////////////////////////////////////////////////
    // CSS RULE
    ($node:expr, $key:ident : $val:expr, $($rest:tt)*) => {
        view_argument!($node, $key : $val);
        view_arguments!(
            $node,
            $($rest)*
        );
    };
    // CSS PSEUDO-CLASS
    ($node:expr, : $key:ident $val:tt, $($rest:tt)*) => {
        view_argument!($node, : $key $val);
        view_arguments!(
            $node,
            $($rest)*
        );
    };
    
    ///////////////////////////////////////////////////////////////////////////
    // MANY - CHILDREN
    ///////////////////////////////////////////////////////////////////////////
    ($node:expr, $key:ident $val:tt, $($rest:tt)*) => {
        view_argument!($node, $key $val);
        view_arguments!(
            $node,
            $($rest)*
        );
    };
    
    ///////////////////////////////////////////////////////////////////////////
    // MANY - EXPRESSION
    ///////////////////////////////////////////////////////////////////////////
    ($node:expr, $value:expr, $($rest:tt)*) => {
        view_argument!($node, $value);
        view_arguments!(
            $node,
            $($rest)*
        );
    };
    
    ///////////////////////////////////////////////////////////////////////////
    // SINGLE
    ///////////////////////////////////////////////////////////////////////////
    ($node:expr, $($rest:tt)*) => {
        view_argument!(
            $node,
            $($rest)*
        );
    };
}


///////////////////////////////////////////////////////////////////////////////
// EXTERNAL
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! view {
    // EMPTY
    () => {{
        use crate::ui::html::*;
        use crate::ui::html::attributes::*;
        use crate::ui::html::css::*;
        use crate::ui::html::events::*;
        Html::new_node(String::from("div"))
    }};
    // WITH DEFAULT TAG NAME
    ($($tag:ident).*| $($x:tt)*) => {{
        use crate::ui::html::*;
        use crate::ui::html::attributes::*;
        use crate::ui::html::css::*;
        use crate::ui::html::events::*;
        use crate::ui::html::macros::parse_path;
        if let Some((top, bottom)) = parse_path(stringify!($($tag)*)) {
            view_arguments!(bottom, $($x)*);
            top
        } else {
            let tag = String::from(stringify!($($tag)*).trim());
            let node = Html::new_node(tag);
            view_arguments!(node, $($x)*);
            node
        }
    }};
    ($tag:ident| $($x:tt)*) => {{
        use crate::ui::html::*;
        use crate::ui::html::attributes::*;
        use crate::ui::html::css::*;
        use crate::ui::html::events::*;
        let node = Html::new_node(String::from(stringify!($tag)));
        view_arguments!(node, $($x)*);
        node
    }};
    // DEFAULT
    ($($x:tt)*) => {{
        use crate::ui::html::*;
        use crate::ui::html::attributes::*;
        use crate::ui::html::css::*;
        use crate::ui::html::events::*;
        let node = Html::new_node(String::from("div"));
        view_arguments!(node, $($x)*);
        node
    }};
}

///////////////////////////////////////////////////////////////////////////////
// INTERNAL FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

pub fn parse_path<Msg: Debug + Clone + 'static>(tags: &str) -> Option<(Html<Msg>, Html<Msg>)> {
    let (last, nodes): (Html<Msg>, Vec<Html<Msg>>) = {
        let mut nodes: Vec<Html<Msg>> = Vec::new();
        for tag in tags.split(" ") {
            nodes.push(Html::new_node(String::from(tag)));
        }
        let last = nodes.pop();
        nodes.reverse();
        (last.expect("must not be empty"), nodes)
    };
    if nodes.is_empty() {
        None
    } else {
        let nodes: Html<Msg> = nodes.into_iter().fold(last.clone(), move |l, r| {
            r.add_child(l);
            r
        });
        Some(
            (nodes, last)
        )
    }
}

