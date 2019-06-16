pub mod ui_utils;

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


///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppSpec {

}

pub enum Msg {
    NoOp,
    Display(Display),
    NewTodo(String),
    Completed(EntryId, IsEntryCompleted),
    MouseHovering(EntryId, bool),
    SubmitTodo
}

#[derive(Default)]
pub struct Model {
    display: Signal<Display>,
    new_todo: Signal<String>,
    entries: VecSignal<TodoEntry>
}

#[derive(Default)]
pub struct TodoEntry {
    id: EntryId,
    value: String,
    completed: Signal<bool>,
    visible: SignalOutput<bool>,
    mouse_hovering: Signal<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Display {
    All,
    Active,
    Completed,
}

type EntryId = u32;
type IsEntryCompleted = bool;

///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////


impl Default for Display {
    fn default() -> Self {
        Display::All
    }
}


///////////////////////////////////////////////////////////////////////////////
// VIEWABLE DATA TYPES
///////////////////////////////////////////////////////////////////////////////

impl Viewable<Msg> for TodoEntry {
    fn view(&self) -> View<Msg> {v1!{
        li {
            width: "100%";
            event.mouse_enter[id@self.id] => move || {
                Msg::MouseHovering(id, true)
            };
            event.mouse_leave[id@self.id] => move || {
                Msg::MouseHovering(id, false)
            };
            if &self.visible.map(|x| !x) => {
                display: "none";
            };
            form {
                display: "flex";
                align_items: "center";
                padding: "6px";
                background_color: "#ececec";
                font_size: "1.1em";
                justify_content: "space-between";
                ui_utils::mk_checkbox("hello world");
                // input {
                //     event.check[id@self.id] => move |value: bool| {
                //         Msg::Completed(id, value)
                //     };
                //     checked = &self.completed;
                //     type = "checkbox";
                // };
                label {
                    width: "100%";
                    &self.value;
                };
                button {
                    outline: "none";
                    border: "none";
                    display: "flex";
                    background: "transparent";
                    if &self.mouse_hovering.map(|x| !x) => {
                        visibility: "hidden";
                    };
                    span {
                        color: "#d60000";
                        font_size: "1.4em";
                        "x";
                    };
                };
            };
        };
    }}
}


///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Spec for AppSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, startup: StartupInfo<Self>) -> Init<Self> {
        Init{
            ..Default::default()
        }
    }
    fn update(&self, model: &mut Model, msg: Msg, sys: &mut SubSystems<Self>) {
        // OPERATION HELPERS
        fn set_display(model: &mut Model, display: Display) {
            let is_empty = model.entries.inspect(|x| x.is_empty());
            if is_empty {
                model.display.set(Display::All);
            } else {
                model.display.set(display);
            }
        }
        fn new_todo(model: &mut Model) {
            let value = model.new_todo.get_copy();
            let completed: Signal<bool> = Signal::new(false);
            let visible: SignalOutput<bool> = model.display
                .zip(&completed)
                .map(move |(display, completed)| -> bool {
                    match display {
                        Display::All => true,
                        Display::Active => {
                            !completed.clone()
                        }
                        Display::Completed => {
                            completed.clone()
                        }
                    }
                });
            model.entries.push(TodoEntry {
                mouse_hovering: Signal::new(false),
                id: rand::random::<EntryId>(),
                visible,
                completed,
                value,
            });
            model.new_todo.set(String::new());
        }
        fn update_entry(model: &mut Model, id: EntryId, f: impl Fn(&mut TodoEntry)) {
            let pred = |x: &TodoEntry| x.id == id;
            let update = |x: &mut TodoEntry| {
                assert!(x.id == id);
                f(x);
            };
            model.entries.update_by(pred, update);
        }
        // GO
        match msg {
            Msg::NoOp => {}
            Msg::Display(display) => {
                set_display(model, display);
            }
            Msg::SubmitTodo => {
                if !model.new_todo.get().is_empty() {
                    new_todo(model);
                }
            }
            Msg::NewTodo(x) => {
                model.new_todo.set(x);
            }
            Msg::Completed(id, is_completed) => {
                update_entry(model, id, move |entry: &mut TodoEntry| {
                    entry.completed.set(is_completed);
                });
            }
            Msg::MouseHovering(id, is_hovering) => {
                update_entry(model, id, move |entry: &mut TodoEntry| {
                    entry.mouse_hovering.set(is_hovering);
                });
            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        text_theme();
        max_width: "500px";
        margin: "0 auto";
        display: "flex";
        flex_direction: "column";
        justify_content: "center";
        align_items: "center";
        h1 {
            text_transform: "uppercase";
            margin: "0";
            "Todo";
        };
        create_todo(model);
        ul {
            width: "100%";
            list_style: "none";
            padding: "0";
            margin: "0";
            &model.entries;
        };
        footer(model);
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn create_todo(model: &Model) -> View<Msg> {v1!{
    form {
        width: "100%";
        event.submit[] => move || {
            Msg::SubmitTodo
        };
        input {
            width: "100%";
            outline: "none";
            value = &model.new_todo;
            event.input[] => move |txt: String| {
                Msg::NewTodo(txt)
            };
            type = "text";
        };
    };
}}

pub fn footer(model: &Model) -> View<Msg> {v1!{
    footer {
        width: "100%";
        display: "flex";
        justify_content: "space-around";
        align_items: "center";
        max_width: "200px";
        padding: "10px";
        button {
            text_theme();
            event.click[] => move || {
                Msg::Display(Display::All)
            };
            "All";
        };
        button {
            text_theme();
            event.click[] => move || {
                Msg::Display(Display::Active)
            };
            "Active";
        };
        button {
            text_theme();
            event.click[] => move || {
                Msg::Display(Display::Completed)
            };
            "Completed";
        };
    };
}}

///////////////////////////////////////////////////////////////////////////////
// VIEW AGNOSTIC UTILS
///////////////////////////////////////////////////////////////////////////////

pub fn text_theme<Msg: 'static>() -> View<Msg> {v1!{
    font_family: "'Source Sans Pro', sans-serif";
}}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

pub fn setup() {
	Program::run_spec(AppSpec{

    });
}

pub fn tick() {
    program_sys::on_request_animation_frame();
}





