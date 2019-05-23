#[macro_export]
macro_rules! to_expr {
    ($x:expr) => {$x};
}

#[macro_export]
macro_rules! arguments {
    ($ctx:expr;) => {()};
    ($ctx:expr; $tag:ident {$($x:tt)*} $($rest:tt)*) => {{
        let mut node = View::new_tag(stringify!($tag));
        arguments!(node; $($x)*);
        $ctx.append(node);
        arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; $key:ident = $value:expr; $($rest:tt)*) => {{
        let value: AttributeValue = internal_normalize_attribute_value($value);
        let mut key = String::from(stringify!($key));
        key.replace("_", "-");
        $ctx.append((key, value));
        arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; $prop:ident: $value:expr; $($rest:tt)*) => {{
        let value: &str = $value;
        let value: String = String::from(value);
        let mut property = String::from(stringify!($prop));
        property.replace("_", "-");
        $ctx.append(Style::Raw{
            property,
            value,
        });
        arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; $viewable:expr; $($rest:tt)*) => {{
        $ctx.append($viewable);
        arguments!($ctx; $($rest)*);
    }};
}

#[macro_export]
macro_rules! view {
    ($($x:tt)*) => {{
        let mut node = View::new_tag("div");
        arguments!(node; $($x)*);
        node
    }};
}

pub fn dev() {
    use ss_css_types::internal::*;
    use crate::*;
    use crate::attributes::*;
    use crate::events::*;
    
    #[derive(Debug, PartialEq)]
    enum Msg {
        NoOp,
    }
    pub struct Model {}
    fn view() -> View<Msg> {
        view!{
            h1{
                "hello world";
            }
            main {
                
            }
        }
    }
}





