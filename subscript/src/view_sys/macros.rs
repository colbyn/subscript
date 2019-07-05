#[macro_export]
#[doc(hidden)]
macro_rules! s1_impl_commas {
    () => {StyleList::new()};
    ($($prop:ident : $value:expr),*) => {{
        let mut style_list = StyleList::new();
        $({
            style_list.push(Style::new(&rewrite_ident(stringify!($prop)), $value));
        })*
        style_list
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! s1_impl {
    () => {StyleList::new()};
    ($($prop:ident : $value:expr;)*) => {{
        let mut style_list = StyleList::new();
        $({
            style_list.push(Style::new(&rewrite_ident(stringify!($prop)), $value));
        })*
        style_list
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! animation_intervals {
    ($xs:expr;) => {};
    ($xs:expr; from => $value:expr; $($rest:tt)*) => {{
        $xs.push(AnimationInterval::new("from", $value));
        animation_intervals!($xs; $($rest)*);
    }};
    ($xs:expr; to => $value:expr; $($rest:tt)*) => {{
        $xs.push(AnimationInterval::new("to", $value));
        animation_intervals!($xs; $($rest)*);
    }};
    ($xs:expr; $x:expr => $value:expr; $($rest:tt)*) => {{
        let value = format!("{}", $x);
        $xs.push(AnimationInterval::new(value.as_str(), $value));
        animation_intervals!($xs; $($rest)*);
    }};
}

#[macro_export]
/// Style list maco.
///
/// ```
/// let ss: StyleList = s1!{
///     display: "flex";
/// };
/// ```
///
/// Syntax:
/// ```
/// property_name: "value";
/// ```
///
/// In contrast to the view macro (`v1!{…}`), this only supports css-properties.
///
/// Technically this should be `s0!`.
macro_rules! s1 {
    ($($x:tt)*) => {{
        use $crate::view_sys::dsl::*;
        use $crate::view_sys::shared::*;
        use $crate::view_sys::macros::*;
        use $crate::view_sys::adapters::*;

        s1_impl!($($x)*)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! to_expr {
    ($x:expr) => {$x}
}

#[macro_export]
#[doc(hidden)]
macro_rules! v1_impl {
    ($env:expr;) => {};
    
    ///////////////////////////////////////////////////////////////////////////
    // CONTROL-FLOW
    ///////////////////////////////////////////////////////////////////////////
    ($env:expr; const if $pred:expr => {$($x:tt)*}; $($rest:tt)*) => {{
        if $pred {
            let mut mixin = View::new_mixin();
            if let Some(mut inner_env) = mixin.get_env() {
                v1_impl!(&mut inner_env; $($x)*);
            }
            else {panic!()}
            $env.children.push(mixin);
        }
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; if $pred:expr => {$($x:tt)*}; $($rest:tt)*) => {{
        let mut mixin = View::new_mixin();
        if let Some(mut inner_env) = mixin.get_env() {
            v1_impl!(&mut inner_env; $($x)*);
        }
        else {panic!()}
        $env.children.push(View::new_toggle_control($pred, mixin));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; bind[$($vars:tt)*] $x:expr => $f:expr; $($rest:tt)*) => {{
        $env.children.push(View::new_dynamic_control($x, {
            clone_ident_arguments_outer!($($vars)*);
            move |new| {
                clone_ident_arguments_inner!($($vars)*);
                ($f)(new)
            }
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; bind $x:expr => $f:expr; $($rest:tt)*) => {{
        $env.children.push(View::new_dynamic_control($x, $f));
        v1_impl!($env; $($rest)*);
    }};

    ///////////////////////////////////////////////////////////////////////////
    // EVENTS
    ///////////////////////////////////////////////////////////////////////////
    ($env:expr; event.click[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_click(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.mouse_down[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_mouse_down(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.mouse_up[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_mouse_up(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.mouse_enter[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_mouse_enter(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.mouse_leave[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_mouse_leave(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.mouse_over[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_mouse_over(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.mouse_out[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_mouse_out(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.input[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_input(move |txt| {
            clone_ident_arguments_inner!($($x)*);
            ($expr)(txt)
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.check[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_check(move |toggle| {
            clone_ident_arguments_inner!($($x)*);
            ($expr)(toggle)
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.submit[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_submit(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.blur[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_blur(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; event.focus[$($x:tt)*] => $expr:expr; $($rest:tt)*) => {{
        clone_ident_arguments_outer!($($x)*);
        $env.events.push(EventHandler::new_on_focus(move || {
            clone_ident_arguments_inner!($($x)*);
            ($expr)()
        }));
        v1_impl!($env; $($rest)*);
    }};
    ///////////////////////////////////////////////////////////////////////////
    // CSS
    ///////////////////////////////////////////////////////////////////////////
    ($env:expr; css.media[$($x:tt)*] => $body:expr; $($rest:tt)*) => {{
        let condition = s1_impl_commas!($($x)*);
        $env.styling.add_media(condition, $body);
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.animation => {$($intervals:tt)*}; $($rest:tt)*) => {{
       let mut intervals: Vec<AnimationInterval> = Vec::new(); 
       animation_intervals!(intervals; $($intervals)*);
       $env.styling.add_animation(intervals);
       v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.active => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_active($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.after => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_after($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.before => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_before($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.checked => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_checked($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.disabled => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_disabled($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.empty => $body:expr; $($rest:tt)*) => {{
       $env.styling.add_state(StateSelector::new_empty($body));
       v1_impl!($env; $($rest)*); 
    }};
    ($env:expr; css.enabled => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_enabled($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.first_child => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_first_child($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.first_letter => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_first_letter($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.first_line => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_first_line($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.focus => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_focus($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.hover => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_hover($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.last_child => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_last_child($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.only_child => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_only_child($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.link => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_link($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.visited => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_visited($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.spelling_error => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_spelling_error($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.grammar_error => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_grammar_error($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.selection => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_selection($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.placeholder => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_placeholder($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.marker => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_marker($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.cue => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_cue($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; css.backdrop => $body:expr; $($rest:tt)*) => {{
        $env.styling.add_state(StateSelector::new_backdrop($body));
        v1_impl!($env; $($rest)*);
    }};
    ($env:expr; $prop:ident : $value:expr; $($rest:tt)*) => {{
        $env.styling.add_style(Style::new(&rewrite_ident(stringify!($prop)), $value));
        v1_impl!($env; $($rest)*);
    }};
    ///////////////////////////////////////////////////////////////////////////
    // HTML ATTRIBUTES
    ///////////////////////////////////////////////////////////////////////////
    ($env:expr; $key:ident = $value:expr; $($rest:tt)*) => {{
        $env.attributes.insert(
            rewrite_ident(stringify!($key)),
            normalize_attribute_value($value),
        );
        v1_impl!($env; $($rest)*);
    }};
    ///////////////////////////////////////////////////////////////////////////
    // HTML TAGS
    ///////////////////////////////////////////////////////////////////////////
    ($env:expr; $tag:ident !{$($x:tt)*}; $($rest:tt)*) => {{
        let mut new_element = View::new_element(stringify!($tag));
        if let Some(mut inner_env) = new_element.get_env() {
            v1_impl!(&mut inner_env; $($x)*);
        }
        else {panic!()}
        $env.children.push(new_element);
        v1_impl!($env; $($rest)*);
    }};
    ///////////////////////////////////////////////////////////////////////////
    // VIEWABLE EXPRESSIONS
    ///////////////////////////////////////////////////////////////////////////
    ($env:expr; $value:expr; $($rest:tt)*) => {{
        run_view_extendable($env, $value);
        v1_impl!($env; $($rest)*);
    }};
}

/// View macro.
/// ```
/// let v: View<Msg> = v1!{
///     h1 !{
///         color: "#777";
///         "Hello World";
///     };
/// };
/// ```
///
/// Syntax:
/// ```
/// // CONTROL
/// if &bool_signal => {...};
/// const if value => {...};
/// bind any_value => move |new_value| -> v1!{...};
/// bind[<scope-helper>] any_value => move |new_value| -> v1!{...};
/// bind[name@model.name] any_value => move |new_value| -> v1!{...};
/// 
/// // CSS PROPERTY
/// property_name: "value";
/// 
/// // CSS MEDIA
/// css.media[property: value, ...] => s1!{...};
/// 
/// // CSS ANIMATION
/// css.animation => {
///     from => s1!{...};
///     to => s1!{...};
/// };
/// css.animation => {
///     0 => s1!{...};
///     ...
///     100 => s1!{...};
/// };
/// 
/// // CSS PSEUDO-SELECTORS
/// css.hover => s1!{...};
/// css.active => s1!{...};
/// css.after => s1!{...};
/// css.before => s1!{...};
/// css.checked => s1!{...};
/// css.disabled => s1!{...};
/// css.empty => s1!{...};
/// css.enabled => s1!{...};
/// css.first_child => s1!{...};
/// css.first_letter => s1!{...};
/// css.first_line => s1!{...};
/// css.focus => s1!{...};
/// css.last_child => s1!{...};
/// css.only_child => s1!{...};
/// css.link => s1!{...};
/// css.visited => s1!{...};
/// css.spelling_error => s1!{...};
/// css.grammar_error => s1!{...};
/// css.selection => s1!{...};
/// css.placeholder => s1!{...};
/// css.marker => s1!{...};
/// css.cue => s1!{...};
/// css.backdrop => s1!{...};
/// 
/// // EVENTS FORMAT
/// event.name[<scope-helper>] => closure_expression;
/// event.name[name@model.name, ...] => closure_expression;
///
/// // DOM-EVENTS
/// event.click[] => move || Msg::Value;
/// event.mouse_down[] => move || Msg::Value;
/// event.mouse_up[] => move || Msg::Value;
/// event.mouse_enter[] => move || Msg::Value;
/// event.mouse_leave[] => move || Msg::Value;
/// event.mouse_over[] => move || Msg::Value;
/// event.mouse_out[] => move || Msg::Value;
/// event.input[] => move |x: String| Msg::Value;
/// event.check[] => move |x: bool| Msg::Value;
/// event.submit[] => move || Msg::Value;
/// event.blur[] => move || Msg::Value;
/// event.focus[] => move || Msg::Value;
///
/// 
/// // HTML ATTRIBUTES
/// key_name = "string";
/// key_name = bool;
/// key_name = signal;
/// 
/// // HTML TAGS
/// tag_name !{...};
/// 
/// // VIEW-EXT EXPRESSIONS
/// expression;
/// ```
///
/// Technically this should be `v0!`.
#[macro_export]
macro_rules! v1 {
    ($($x:tt)*) => {{
        use $crate::view_sys::dsl::*;
        use $crate::view_sys::shared::*;
        use $crate::view_sys::macros::*;
        use $crate::view_sys::adapters::*;
        use ::either::{Either, Either::*};

        let mut mixin = View::new_mixin();
        if let Some(mut env) = mixin.get_env() {
            v1_impl!(&mut env; $($x)*);
        }
        mixin
    }}
}


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS INTERNAL HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn rewrite_ident(x: &str) -> String {
    x.replace("_", "-")
}

#[macro_export]
#[doc(hidden)]
macro_rules! extend_callback_env {
    ($inner:expr;) => {
        {move || {$inner}}
    };
    ($inner:expr; $name:ident @ $value:expr) => {{
        let $name = $value.clone();
        {move || {$inner}}
    }};
    ($inner:expr; $name:ident) => {{
        let $name = $name.clone();
        {$inner}
    }};
    ($inner:expr; $name:ident @ $value:expr, $($rest:tt)*) => {{
        let $name = $value.clone();
        extend_callback_env!($inner; $($rest)*)
    }};
    ($inner:expr; $name:ident, $($rest:tt)*) => {{
        let $name = $name.clone();
        extend_callback_env!($inner; $($rest)*)
    }};
}


#[macro_export]
#[doc(hidden)]
macro_rules! clone_ident_arguments_outer {
    () => {};
    ($name:ident @ $value:expr) => {
        let $name = $value.clone();
    };
    ($name:ident) => {
        let $name = $name.clone();
    };
    ($name:ident @ $value:expr, $($rest:tt)*) => {
        let $name = $value.clone();
        clone_ident_arguments_outer!($($rest)*)
    };
    ($name:ident, $($rest:tt)*) => {
        let $name = $name.clone();
        clone_ident_arguments_outer!($($rest)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! clone_ident_arguments_inner {
    () => {};
    ($name:ident @ $value:expr) => {
        let $name = $name.clone();
    };
    ($name:ident) => {
        let $name = $name.clone();
    };
    ($name:ident @ $value:expr, $($rest:tt)*) => {
        let $name = $name.clone();
        clone_ident_arguments_inner!($($rest)*)
    };
    ($name:ident, $($rest:tt)*) => {
        let $name = $name.clone();
        clone_ident_arguments_inner!($($rest)*)
    };
}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

// pub mod dev {
// use crate::view_sys::dsl::View;
//     use crate::reactive_sys::*;

//     pub enum Msg {
//         NoOP,
//         Value(String),
//     }

//     pub struct Model {
//         value: Signal<String>,
//         display: Signal<bool>
//     }

//     pub fn dev(model: &Model) -> View<Msg> {v1!{
//         div !{
//             if &model.display => {
//                 h1 !{
//                     "Hello World";
//                 };
//             };

//             css.media[max_width: "900px"] => s1!{
//                 background_color: "red";
//             };
//             css.hover => s1!{
//                 color: "blue";
//             };
//             css.animation => {
//                 from => s1!{
//                     color: "#fff";
//                 };
//                 to => s1!{
//                     color: "#000";
//                 };
//             };
//         };
//     }}
// }


