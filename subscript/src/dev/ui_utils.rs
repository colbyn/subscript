use std::marker::Sized;
use std::cell::*;
use serde::{Serialize, Deserialize};

use ss_web_utils::js::console;

use ss_view_tree::{View, Mixin, Viewable, Env};
use ss_view_tree::attributes::*;
use ss_view_tree::styling::*;
use ss_view_tree::styling::selectors::*;
use ss_view_tree::events::EventHandler;
use ss_view_tree::events::*;
use ss_program::Subscriptions;
use ss_program::StartupInfo;
use ss_program::Init;
use ss_program::SubSystems;
use ss_program::Component;
use ss_program::Program;
use ss_program::Spec;


///////////////////////////////////////////////////////////////////////////////
// VIEW - SPEC/COMPONENT AGNOSTIC REUSABLE UTILS
///////////////////////////////////////////////////////////////////////////////

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
        // on_check(Box::new(data.on_click));
        type="checkbox";
    }
    div {
        box_sizing: "border-box";
        position: "relative";
        display: "inline-block";
        margin_right: "1em";
        white_space: "nowrap";
        line_height: "1";
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
            class="icon fas fa-check";
            // if (data.checked) {
            //     class="icon fas fa-check";
            // };
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
            data.label;
        }
    }
}}