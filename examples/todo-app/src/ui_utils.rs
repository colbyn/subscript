use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};
use subscript::prelude::*;


pub struct Checkbox<'a, Msg> {
    pub checked: &'a Signal<bool>,
    pub label: &'a str,
    pub mixin: View<Msg>,
}

impl<'b, Msg: 'static> ViewExt<Msg> for Checkbox<'b, Msg> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(mk_checkbox(self.label, self.checked, self.mixin));
    }
}

fn mk_checkbox<'a, Msg: 'static>(label: &str, checked: &Signal<bool>, mixin: View<Msg>) -> View<Msg> {v1!{
    span !{
        width: "28px";
        height: "26px";
        position: "relative";
        display: "flex";
        justify_content: "center";
        align_items: "center";
        transform: "scale(0.9)";
        border: "1px solid #dadada";
        border_radius: "100%";
        padding: "2px";
        margin: "1px";
        i !{
            display: "block";
            position: "absolute";
            transform: "scale(0.8)";
            if checked => {
                class = "icon fas fa-check";
            };
        };
        mixin;
    };
}}


