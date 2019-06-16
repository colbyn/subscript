use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi};
use crate::reactive_sys::*;
use crate::view_sys::runtime::common::ElementEnv;
use crate::view_sys::shared::*;
use crate::view_sys::{dom, dsl, runtime, dom::{Dom, Element}, dsl::{View, Dsl}};
use crate::view_sys::adapters::*;
use crate::program_sys::instances::Component;
use crate::program_sys::spec::*;
use crate::program_sys::{self, Program};


pub struct Checkbox<'a, Msg, F> where F: Fn(bool) -> Msg + 'static + std::marker::Sized {
    pub checked: &'a UnitSignal<bool>,
    pub label: &'a str,
    pub on_click: F,
}

pub fn mk_checkbox<'a, Msg: 'static>(label: &str) -> View<Msg> {v1!{
    div {
        // display: flex;
        // justify-content: center;
        // align-items: center;
        position: "relative";
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
            checked = true;
            type="checkbox";
        };
        i {
            border_radius: "100%";
            border: "1px solid #000";
            padding: "2px";
            font_size: "0.4em";
            class = "icon fas fa-check";
        };
    };
}}


