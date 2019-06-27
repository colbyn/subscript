use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};
use subscript::prelude::*;



///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppSpec {

}

pub enum Msg {
    NoOp,
    Increment,
    Decrement,
}

#[derive(Default)]
pub struct Model {
    counter: Signal<i32>,
}



///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
// VIEWABLE DATA TYPES
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Spec for AppSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, startup: &Shell<Self>) -> Init<Self> {
        Init{
            ..Default::default()
        }
    }
    fn update(&self, model: &mut Model, msg: Msg, sys: &mut Shell<Self>) {
        match msg {
            Msg::NoOp => {}
            Msg::Increment => {
                let current = model.counter.get_copy();
                model.counter.set(current + 1);
            }
            Msg::Decrement => {
                let current = model.counter.get_copy();
                model.counter.set(current - 1);
            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        display: "flex";
        flex_direction: "column";
        display: "flex";
        flex_direction: "column";
        max_width: "600px";
        margin: "0 auto";
        padding_top: "30px";

        css.media[max_width: "600px"] => s1!{
            padding: "0 10px";
        };

        h1 !{
            text_theme();
            margin: "0";
            text_align: "center";
            font_size: "6em";
            margin_bottom: "10px";
            color: "#777";
            font_weight: "700";
            transition: "1s";
            css.hover => s1!{
                font_size: "8em";
                color: "#00fdde";
            };
            model.counter.map(|x| format!("{}", x));
        };
        button !{
            text_theme();
            outline: "none";
            user_select: "none";
            padding: "4px";
            font_size: "2em";
            border: "none";
            border_radius: "3px";
            background_color: "#565656";
            color: "#fff";
            margin_bottom: "10px";
            event.click[] => move || Msg::Increment;
            "Increment";
        };
        button !{
            text_theme();
            outline: "none";
            user_select: "none";
            padding: "4px";
            font_size: "2em";
            border: "none";
            border_radius: "3px";
            background_color: "#565656";
            color: "#fff";
            event.click[] => move || Msg::Decrement;
            "Decrement";
        };
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////
// VIEW AGNOSTIC UTILS
///////////////////////////////////////////////////////////////////////////////

pub fn text_theme<Msg: 'static>() -> View<Msg> {v1!{
    font_family: "'Source Sans Pro', sans-serif";
    font_weight: "200";
}}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

pub fn setup() {
    Program::run_spec(AppSpec{

    });
}

pub fn tick() {
    subscript::prelude::on_request_animation_frame();
}
