


///////////////////////////////////////////////////////////////////////////////
// VIEW CONSTRUCTION MACROS
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! view_arguments {
    ($ctx:expr;) => {()};
    ($ctx:expr; $tag:ident {$($x:tt)*} $($rest:tt)*) => {{
        let mut node = View::new_tag(stringify!($tag));
        view_arguments!(node; $($x)*);
        $ctx.extend(node);
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; $key:ident = $value:expr; $($rest:tt)*) => {{
        let value: AttributeValue = internal_normalize_attribute_value($value);
        let key = String::from(stringify!($key));
        let ket = key.replace("_", "-");
        $ctx.extend((key, value));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; $prop:ident: $value:expr; $($rest:tt)*) => {{
        let value: &str = $value;
        let value: String = String::from(value);
        let property = String::from(stringify!($prop));
        let property = property.replace("_", "-");
        $ctx.extend(Style::Raw{
            property,
            value,
        });
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; $viewable:expr; $($rest:tt)*) => {{
        $ctx.extend($viewable);
        view_arguments!($ctx; $($rest)*);
    }};
}

#[macro_export]
macro_rules! v {
    ($tag:ident| $($x:tt)*) => {{
        let mut node = View::new_tag(stringify!($tag));
        view_arguments!(node; $($x)*);
        node
    }};
    ($($x:tt)*) => {{
        let mut node = View::new_tag("div");
        view_arguments!(node; $($x)*);
        node
    }};
}


///////////////////////////////////////////////////////////////////////////////
// PUBLIC VIEW HELPER MACORS
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! extend_ident_argument {
    ($name:ident @ $value:expr) => {
        let $name = $value.clone();
    };
    ($name:ident) => {
        let $name = $name.clone();
    };
}

#[macro_export]
macro_rules! extend {
    ($fn_name:ident, [], $body:expr) => {{
        $fn_name(($body))
    }};
    ($fn_name:ident, [$($($x:tt)*),*], $body:expr) => {{
        $(extend_ident_argument!($($x)*))*;
        $fn_name(($body))
    }};
}



///////////////////////////////////////////////////////////////////////////////
// DEV/TESTING
///////////////////////////////////////////////////////////////////////////////

pub fn dev() {
    use crate::*;
    use crate::attributes::*;
    use crate::events::*;
    
    #[derive(Debug, PartialEq)]
    enum Msg {
        NoOp,
        SetCounter(u32),
    }
    pub struct Model {
        counter: u32
    }

    fn view(model: &Model) -> View<Msg> {
        v!{
            h1{
                "hello world";
            }
            button {
                extend!(on_click, [counter@model.counter], move || {
                    Msg::SetCounter(counter)
                });
            }
            main {
                
            }
        }
    }
}





