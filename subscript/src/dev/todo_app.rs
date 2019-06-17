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
    RemoveTodo(EntryId),
    Completed(EntryId, IsEntryCompleted),
    ToggleCompleted(EntryId),
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
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
            border_bottom: "1px solid #f1f1f1";
            css.last_child => s1!{
                border_bottom: "none";
            };
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
                font_size: "1.1em";
                justify_content: "space-between";
                ui_utils::Checkbox {
                    label: "hello world",
                    checked: &self.completed,
                    mixin: v1!{
                        event.click[id@self.id] => || {
                            Msg::ToggleCompleted(id)
                        };
                    },
                };
                label {
                    width: "100%";
                    margin_left: "8px";
                    user_select: "none";
                    font_size: "1.4em";
                    event.click[id@self.id] => || {
                        Msg::ToggleCompleted(id)
                    };
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
                    event.click[id@self.id] => move || {
                        Msg::RemoveTodo(id)
                    };
                    span {
                        user_select: "none";
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
    fn update(&self, model: &mut Model, msg: Msg, sys: &mut Shell<Self>) {
        // OPERATION HELPERS
        fn set_display(model: &mut Model, display: Display) {
            let is_empty = model.entries.get_by(|x| x.is_empty());
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
            model.entries.insert(0, TodoEntry {
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
        fn remove_entry(model: &mut Model, id: EntryId) {
            let pred = |x: &TodoEntry| x.id == id;
            model.entries.remove_by(pred);
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
            Msg::RemoveTodo(id) => {
                remove_entry(model, id);
            }
            Msg::Completed(id, is_completed) => {
                update_entry(model, id, move |entry: &mut TodoEntry| {
                    entry.completed.set(is_completed);
                });
            }
            Msg::ToggleCompleted(id) => {
                update_entry(model, id, move |entry: &mut TodoEntry| {
                    let current = entry.completed.get_copy();
                    entry.completed.set(!current);
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
        overflow: "auto";
        width: "100%";
        height: "100%";
        padding_bottom: "100px";
        background_color: "#efefef";
        display: "flex";
        flex_direction: "column";
        h1 {
            text_transform: "uppercase";
            font_size: "3em";
            font_weight: "300";
            margin: "20px 0";
            text_align: "center";
            "Todo";
        };
        div {
            box_shadow:
                "0 2px 4px 0 rgba(0, 0, 0, 0.2),\
                 0 25px 50px 0 rgba(0, 0, 0, 0.1)";
            width: "100%";
            max_width: "500px";
            margin: "0 auto";
            display: "flex";
            flex_direction: "column";
            justify_content: "center";
            align_items: "center";
            background_color: "#fff";
            create_todo(model);
            ul {
                width: "100%";
                list_style: "none";
                padding: "0";
                margin: "0";
                &model.entries;
            };
            footer(model);
        };
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn create_todo(model: &Model) -> View<Msg> {v1!{
    form {
        width: "100%";
        border_bottom: "1px solid #e6e6e6";
        box_shadow: "inset 0 -2px 1px rgba(0,0,0,0.03)";
        padding: "10px";
        event.submit[] => move || {
            Msg::SubmitTodo
        };
        input {
            outline: "none";
            border: "none";
            background: "transparent";
            width: "100%";
            value = &model.new_todo;
            font_size: "1.5em";
            font_weight: "100";
            css.placeholder => s1!{
                font_size: "1em";
                font_variant: "petite-caps";
            };
            event.input[] => move |txt: String| {
                Msg::NewTodo(txt)
            };
            placeholder = "What needs to be done?";
            type = "text";
        };
    };
}}

pub fn footer(model: &Model) -> View<Msg> {
    let set_display_button = |name: &str, display: Display| v1!{
        button {
            text_theme();
            outline: "none";
            user_select: "none";
            padding: "4px 7px";
            border_radius: "3px";
            margin_left: "14px";
            margin_right: "14px";
            text_transform: "uppercase";
            font_size: "0.8em";
            font_weight: "400";
            if &model.display.map(move |x| x == &display) => {
                border: "1px solid #ef87c1";
            };
            if &model.display.map(move |x| x != &display) => {
                border: "1px solid transparent";
                css.hover => s1!{
                    border: "1px solid #fbb5c2";
                };
            };
            event.click[display] => move || {
                Msg::Display(display)
            };
            name;
        };
    };
    v1!{
        footer {
            border_top: "1px solid #e6e6e6";
            box_shadow:
                "0 1px 1px rgba(0, 0, 0, 0.2), 0 8px 0 -3px #f6f6f6, \
                 0 9px 1px -3px rgba(0, 0, 0, 0.2), \
                 0 16px 0 -6px #f6f6f6, \
                 0 17px 2px -6px rgba(0, 0, 0, 0.2)";
            padding: "10px";
            width: "100%";
            align_items: "center";
            display: "grid";
            grid_template_columns: "100px 1fr 100px";
            div {
                span {
                    user_select: "none";
                    model.entries.reduce_to(|xs| {
                        format!("{} items left", xs.len())
                    });
                };
            };
            div {
                display: "flex";
                justify_content: "space-around";
                if &model.entries.reduce_to(|xs| !xs.is_empty()) => {
                    set_display_button("All", Display::All);
                    set_display_button("Active", Display::Active);
                    set_display_button("Completed", Display::Completed);
                };
            };
            div {

            };
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
// VIEW AGNOSTIC UTILS
///////////////////////////////////////////////////////////////////////////////

pub fn text_theme<Msg: 'static>() -> View<Msg> {v1!{
    font_family: "'Source Sans Pro', sans-serif";
    color: "#777";
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
    program_sys::on_request_animation_frame();
}





