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
            if &self.visible.map(|x| {
                console!("TodoEntry - display check");
                !x
            }) => {
                display: "none";
            };
            form {
                input {
                    event.check[id@self.id] => move |value: bool| {
                        Msg::Completed(id, value)
                    };
                    checked = &self.completed;
                    type = "checkbox";
                };
                label {
                    &self.value;
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
        // OPERATIONS
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
                id: rand::random::<EntryId>(),
                visible,
                completed,
                value,
            });
            model.new_todo.set(String::new());
        }
        fn entry_completed(model: &mut Model, id: EntryId, is_completed: IsEntryCompleted) {
            let pred = |x: &TodoEntry| x.id == id;
            let update = |x: &mut TodoEntry| {
                assert!(x.id == id);
                x.completed.set(is_completed);
            };
            model.entries.update_by(pred, update);
        }
        // GO
        match msg {
            Msg::NoOp => {}
            Msg::Display(display) => {
                // set_display(model, display);
                model.display.set(display);
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
                entry_completed(model, id, is_completed);
            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        h1 {
            color: "#999";
            "Todo";
        };
        create_todo(model);
        ul {
            list_style: "none";
            padding: "0";
            margin: "0";
            &model.entries;
        };
        footer(model);
    }}
}

pub fn create_todo(model: &Model) -> View<Msg> {v1!{
    form {
        event.submit[] => move || {
            Msg::SubmitTodo
        };
        input {
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
        button {
            event.click[] => move || {
                Msg::Display(Display::All)
            };
            "All";
        };
        button {
            event.click[] => move || {
                Msg::Display(Display::Active)
            };
            "Active";
        };
        button {
            event.click[] => move || {
                Msg::Display(Display::Completed)
            };
            "Completed";
        };
    };
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





