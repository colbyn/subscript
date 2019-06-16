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
    visible: Option<SignalOutput<bool>>,
    completed: Signal<bool>,
    value: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Display {
    All,
    Active,
    Completed,
}

type EntryCompleted = bool;

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
        form {
            label {
                &self.value;
            }
        }
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
                visible: Some(visible),
                completed,
                value,
            });
            model.new_todo.set(String::new());
        }
        // GO
        match msg {
            Msg::NoOp => {}
            Msg::Display(display) => {
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
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        h1 {
            "Todo";
        }
        new_todo(model);
        &model.entries;
        footer(model);
    }}
}

pub fn new_todo(model: &Model) -> View<Msg> {v1!{
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
        }
    }
}}

pub fn footer(model: &Model) -> View<Msg> {v1!{
    button {
        event.click[] => || {
            Msg::Display(Display::All)
        };
        "All";
    }
    button {
        event.click[] => || {
            Msg::Display(Display::Active)
        };
        "Active";
    }
    button {
        event.click[] => || {
            Msg::Display(Display::Completed)
        };
        "Completed";
    }
    // if &model.display.map(|display: &Display| display == &Display::All) => {

    // };
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





