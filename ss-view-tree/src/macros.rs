


///////////////////////////////////////////////////////////////////////////////
// VIEW CONSTRUCTION MACROS
///////////////////////////////////////////////////////////////////////////////
#[macro_export]
macro_rules! styles {
    ($ctx:expr;) => {{}};
    ($ctx:expr; $prop:ident: $value:expr; $($rest:tt)*) => {{
        let value: &str = $value;
        let value: String = String::from(value);
        let property = String::from(stringify!($prop));
        let property = property.replace("_", "-");
        $ctx.push(Style::Untyped(Untyped{
            property,
            value,
        }));
        styles!($ctx; $($rest)*);
    }};
}
#[macro_export]
macro_rules! media_types {
    ($ctx:expr;) => {{}};
    ($ctx:expr;) => {{}};
    ($ctx:expr; $prop:ident: $value:expr) => {{
        let value: &str = $value;
        let value: String = String::from(value);
        let property = String::from(stringify!($prop));
        let property = property.replace("_", "-");
        $ctx.push(Style::Untyped(Untyped{
            property,
            value,
        }));
    }};
    ($ctx:expr; $prop:ident: $value:expr, $($rest:tt)*) => {{
        let value: &str = $value;
        let value: String = String::from(value);
        let property = String::from(stringify!($prop));
        let property = property.replace("_", "-");
        $ctx.push(Style::Untyped(Untyped{
            property,
            value,
        }));
        media_types!($ctx; $($rest)*);
    }};
}

#[macro_export]
macro_rules! state_selector {
    ($ctx:expr; :active {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(active(body));
    }};
    ($ctx:expr; :after {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(after(body));
    }};
    ($ctx:expr; :before {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(before(body));
    }};
    ($ctx:expr; :checked {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(checked(body));
    }};
    ($ctx:expr; :disabled {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(disabled(body));
    }};
    ($ctx:expr; :empty {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(empty(body));
    }};
    ($ctx:expr; :enabled {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(enabled(body));
    }};
    ($ctx:expr; :first-child {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(first_child(body));
    }};
    ($ctx:expr; :first-letter {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(first_letter(body));
    }};
    ($ctx:expr; :first-line {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(first_line(body));
    }};
    ($ctx:expr; :focus {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(focus(body));
    }};
    ($ctx:expr; :hover {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(hover(body));
    }};
    ($ctx:expr; :last-child {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(last_child(body));
    }};
    ($ctx:expr; :only-child {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(only_child(body));
    }};
    ($ctx:expr; :link {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(link(body));
    }};
    ($ctx:expr; :visited {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(visited(body));
    }};
    ($ctx:expr; :spelling-error {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(spelling_error(body));
    }};
    ($ctx:expr; :grammar-error {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(grammar_error(body));
    }};
    ($ctx:expr; :selection {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(selection(body));
    }};
    ($ctx:expr; :placeholder {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(placeholder(body));
    }};
    ($ctx:expr; :marker {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(marker(body));
    }};
    ($ctx:expr; :cue {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(cue(body));
    }};
    ($ctx:expr; :backdrop {$($x:tt)*}) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.extend(backdrop(body));
    }};
}


#[macro_export]
macro_rules! view_arguments {
    ($ctx:expr;) => {()};
    ($ctx:expr; if ($pred:expr) {$($x:tt)*}; $($rest:tt)*) => {{
        if ($pred) {
            view_arguments!($ctx; $($x)*);
        }
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; @media [$($xs:tt)*] {$($ys:tt)*}; $($rest:tt)*) => {{
        let mut selector: Vec<Style> = Vec::new();
        let mut body: Vec<Style> = Vec::new();
        media_types!(selector; $($xs)*);
        // styles!(body; $($ys)*);
        $ctx.extend(media(selector, body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :active {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :active {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :after {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :after {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :before {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :before {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :checked {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :checked {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :disabled {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :disabled {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :empty {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :empty {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :enabled {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :enabled {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :first-child {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :first-child {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :first_child {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :first-child {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :first-letter {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :first-letter {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :first_letter {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :first-letter {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :first-line {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :first-line {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :first_line {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :first-line {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :focus {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :focus {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :hover {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :hover {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :last-child {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :last-child {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :last_child {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :last-child {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :only-child {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :only-child {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :only_child {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :only-child {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :link {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :link {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :visited {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :visited {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :spelling-error {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :spelling-error {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :spelling_error {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :spelling-error {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :grammar-error {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :grammar-error {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :grammar_error {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :grammar-error {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :selection {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :selection {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :placeholder {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :placeholder {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :marker {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :marker {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :cue {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :cue {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :backdrop {$($x:tt)*}; $($rest:tt)*) => {{
        state_selector!($ctx; :backdrop {$($x)*});
        view_arguments!($ctx; $($rest)*);
    }};
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
        $ctx.extend(Style::Untyped(Untyped{
            property,
            value,
        }));
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
macro_rules! extend_ident_arguments {
    () => {{}};
    ($name:ident @ $value:expr) => {
        let $name = $value.clone();
    };
    ($name:ident) => {
        let $name = $name.clone();
    };
    ($name:ident @ $value:expr, $($rest:tt)*) => {
        let $name = $value.clone();
        extend_ident_arguments!($($rest)*);
    };
    ($name:ident, $($rest:tt)*) => {
        let $name = $name.clone();
        extend_ident_arguments!($($rest)*);
    };
}

#[macro_export]
macro_rules! extend {
    ($fn_name:ident, [], $body:expr) => {{
        $fn_name(($body))
    }};
    ($fn_name:ident, [$($x:tt)*], $body:expr) => {{
        extend_ident_arguments!($($x)*);
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
    use crate::styling::*;
    use crate::styling::selectors::*;
    
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
                @media [min_width: "100px"] {
                    display: "flex";
                    display: "flex";
                };
                :first_line {
                    display: "flex";
                };
                display: "flex";
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





