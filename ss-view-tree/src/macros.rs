


///////////////////////////////////////////////////////////////////////////////
// VIEW CONSTRUCTION MACROS
///////////////////////////////////////////////////////////////////////////////
#[macro_export]
macro_rules! styles {
    ($ctx:expr;) => {{}};
    ($ctx:expr; $($prop:ident: $value:expr;)*) => {{
        $({
            let value: &str = $value;
            let value: String = String::from(value);
            let property = String::from(stringify!($prop));
            let property = property.replace("_", "-");
            $ctx.push(Style::Untyped(Untyped{
                property,
                value,
            }));
        })*
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
macro_rules! keyframe_interval {
    (from => {$($x:tt)*}) => {{
        use std::iter::FromIterator;
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        let value = String::from("from");
        keyframe_interval(value, body)
    }};
    (to => {$($x:tt)*}) => {{
        use std::iter::FromIterator;
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        let value = String::from("to");
        keyframe_interval(value, body)
    }};
    ($value:expr => {$($x:tt)*}) => {{
        use std::iter::FromIterator;
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        let value: u8 = $value;
        let value: String = format!("{}", value);
        keyframe_interval(value, body)
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
        $ctx.merge(media(selector, body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; @keyframes {$($val:tt => $body:tt)*}; $($rest:tt)*) => {{
        let mut xs: Vec<KeyframeInterval> = Vec::new();
        $({
            xs.push(keyframe_interval!($val => $body));
        })*
        $ctx.merge(keyframes(xs));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :active {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(active(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :after {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(after(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :before {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(before(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :checked {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(checked(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :disabled {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(disabled(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :empty {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(empty(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :enabled {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(enabled(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :first-child {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(first_child(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :first-letter {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(first_letter(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :first-line {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(first_line(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :focus {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(focus(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :hover {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(hover(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :last-child {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(last_child(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :only-child {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(only_child(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :link {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(link(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :visited {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(visited(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :spelling-error {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(spelling_error(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :grammar-error {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(grammar_error(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :selection {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(selection(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :placeholder {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(placeholder(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :marker {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(marker(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :cue {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(cue(body));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; :backdrop {$($x:tt)*}; $($rest:tt)*) => {{
        let mut body: Vec<Style> = Vec::new();
        styles!(body; $($x)*);
        $ctx.merge(backdrop(body));
        view_arguments!($ctx; $($rest)*);
    }};




    ($ctx:expr; $tag:ident {$($x:tt)*} $($rest:tt)*) => {{
        let mut node = View::new_tag(stringify!($tag));
        view_arguments!(node; $($x)*);
        $ctx.merge(node);
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; $key:ident = $value:expr; $($rest:tt)*) => {{
        let value: AttributeValue = internal_normalize_attribute_value($value);
        let key = String::from(stringify!($key));
        let ket = key.replace("_", "-");
        $ctx.merge((key, value));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; $prop:ident: $value:expr; $($rest:tt)*) => {{
        let value: &str = $value;
        let value: String = String::from(value);
        let property = String::from(stringify!($prop));
        let property = property.replace("_", "-");
        $ctx.merge(Style::Untyped(Untyped{
            property,
            value,
        }));
        view_arguments!($ctx; $($rest)*);
    }};
    ($ctx:expr; $viewable:expr; $($rest:tt)*) => {{
        $ctx.merge($viewable);
        view_arguments!($ctx; $($rest)*);
    }};
}


///////////////////////////////////////////////////////////////////////////////
// PUBLIC VIEW MACROS
///////////////////////////////////////////////////////////////////////////////


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

#[macro_export]
macro_rules! mix {
    ($($x:tt)*) => {{
        let mut mixin: Mixin<_> = Mixin::default();
        view_arguments!(mixin; $($x)*);
        mixin
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
    ([], $body:expr) => {{
        {
            $body
        }
    }};
    ([$($x:tt)*], $body:expr) => {{
        extend_ident_arguments!($($x)*);
        {
            $body
        }
    }};

    ($fn_name:ident, [], $body:expr) => {{
        $fn_name(($body))
    }};
    ($fn_name:ident, [$($x:tt)*], $body:expr) => {{
        extend_ident_arguments!($($x)*);
        $fn_name($body)
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
            @keyframes {
                0 => {
                    // ... (CSS PROPERTIES) ...
                }
                50 => {
                    // ... (CSS PROPERTIES) ...
                }
                80 => {
                    // ... (CSS PROPERTIES) ...
                }
                100 => {
                    // ... (CSS PROPERTIES) ...
                }
            };
            h1{
                @media [min_width: "100px"] {
                    display: "flex";
                    display: "flex";
                };
                :first-line {
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
    fn button() -> Mixin<Msg> {
        mix!{
            button {

            }
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// VIEW - SPEC/COMPONENT AGNOSTIC REUSABLE UTILS
///////////////////////////////////////////////////////////////////////////////


use crate::styling::selectors::*;
use crate::*;

pub struct Checkbox<'a, Msg, F> where F: Fn(bool) -> Msg + 'static + std::marker::Sized {
    pub checked: bool,
    pub label: &'a str,
    pub on_click: F,
}

impl<'a, Msg, F> Viewable<Msg> for Checkbox<'a, Msg, F> where F: Fn(bool) -> Msg + 'static + std::marker::Sized {
    fn extend<'b>(self, env: Env<'b, Msg>) {
        env.children.push(mk_checkbox(self));
    }
}

fn mk_checkbox<'a, Msg>(data: Checkbox<'a, Msg, impl Fn(bool) -> Msg + 'static>) -> View<Msg> {v!{div|
    box_sizing: "border-box";
    position: "relative";
    display: "inline-block";
    margin_right: "1em";
    white_space: "nowrap";
    line_height: "1";

    input {
        box_sizing: "border-box";
        position: "absolute";
        left: "0";
        top: "0";
        min_width: "1em";
        width: "100%";
        height: "100%";
        z_index: "2";
        opacity: "0";
        margin: "0";
        padding: "0";
        cursor: "pointer";
        // checked = data.checked;
        on_check(Box::new(data.on_click));
        type="checkbox";
    }
    div {
        box_sizing: "border-box";
        position: "relative";
        display: "inline-block";
        margin_right: "1em";
        white_space: "nowrap";
        line_height: "1";
        span {
            display: "inline-block";
            margin_right: "1em";
            white_space: "nowrap";
            line_height: "1";
            box_sizing: "border-box";
            position: "absolute";
            font_size: "1em";
            width: "calc(1em + 2px)";
            height: "calc(1em + 2px)";
            left: "0";
            z_index: "1";
            text_align: "center";
            line_height: "normal";
            top: "calc((0% - (100% - 1em)) - 8%)";
            border: "1px solid transparent";
            border_radius: "100%";
            overflow: "hidden";

            :before {
                _webkit_transform: "scale(.8)";
                _ms_transform: "scale(.8)";
                transform: "scale(.8)";
                margin: "0";
                width: "100%";
                height: "100%";
                text_align: "center";
                display: "-webkit-box";
                display: "-ms-flexbox";
                display: "flex";
                _webkit_box_flex: "1";
                _ms_flex: "1";
                flex: "1";
                _webkit_box_pack: "center";
                _ms_flex_pack: "center";
                justify_content: "center";
                _webkit_box_align: "center";
                _ms_flex_align: "center";
                align_items: "center";
                line_height: "1";
            };
            if (data.checked) {
                i {
                    display: "inline-block";
                    margin_right: "1em";
                    white_space: "nowrap";
                    line_height: "1";
                    box_sizing: "border-box";
                    position: "absolute";
                    font_size: "1em";
                    width: "calc(1em + 2px)";
                    height: "calc(1em + 2px)";
                    left: "0";
                    z_index: "1";
                    text_align: "center";
                    line_height: "normal";
                    top: "calc((0% - (100% - 1em)) - 8%)";
                    border: "1px solid transparent";
                    border_radius: "100%";
                    overflow: "hidden";

                    :before {
                        _webkit_transform: "scale(.8)";
                        _ms_transform: "scale(.8)";
                        transform: "scale(.8)";
                        margin: "0";
                        width: "100%";
                        height: "100%";
                        text_align: "center";
                        display: "-webkit-box";
                        display: "-ms-flexbox";
                        display: "flex";
                        // _webkit_box_flex: "1";
                        // _ms_flex: "1";
                        // flex: "1";
                        // _webkit_box_pack: "center";
                        // _ms_flex_pack: "center";
                        // justify_content: "center";
                        // _webkit_box_align: "center";
                        // _ms_flex_align: "center";
                        // align_items: "center";
                        // line_height: "1";
                    };
                    
                    class="icon fas fa-check";
                }
            };
        }
        label {
            box_sizing: "border-box";
            font_weight: "400";
            margin: "0";
            text_indent: "1.5em";
            min_width: "calc(1em + 2px)";
            position: "relative";
            display: "inline-block";
            margin_right: "1em";
            white_space: "nowrap";
            line_height: "1";
            text_align: "-webkit-match-parent";
            :before {
                content: "''";
                width: "calc(1em + 2px)";
                height: "calc(1em + 2px)";
                display: "block";
                box_sizing: "border-box";
                border: "1px solid transparent";
                z_index: "0";
                position: "absolute";
                left: "0";
                top: "calc((0% - (100% - 1em)) - 8%)";
                background_color: "transparent";
                border_color: "#5a656b";
                border_radius: "100%";
            };
            "Click Me";
        }
    }
}}
