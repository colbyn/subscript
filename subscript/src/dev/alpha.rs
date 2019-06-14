use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi};
use crate::signals_sys::*;
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

#[derive(Debug)]
pub enum Msg {
    NoOp,
    NewTodo(String),
    SubmitTodo
}

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    display: Signal<Display>,
    new_todo: Signal<String>,
    entries: VecSignal<TodoEntry>
}

#[derive(Serialize, Deserialize, Default)]
pub struct TodoEntry {
    display: Signal<bool>,
    completed: bool,
    value: Signal<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Display {
    All,
    Active,
    Completed,
}

impl Default for Display {
    fn default() -> Self {
        Display::All
    }
}

///////////////////////////////////////////////////////////////////////////////
// METHOD UTILS
///////////////////////////////////////////////////////////////////////////////

impl TodoEntry {
    pub fn set_display(&mut self, current: Display) {
        match current {
            Display::All => {
                self.display.set(true);
            }
            Display::Active => {
                self.display.set(!self.completed);
            }
            Display::Completed => {
                self.display.set(self.completed);
            }
        }
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
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, sys: &mut SubSystems<Self>) {
        match msg {
            Msg::NoOp => {}
            Msg::SubmitTodo => {
                if !model.new_todo.get().is_empty() {
                    let text = model.new_todo.map_mut(|x| x.drain(..).collect::<String>());
                    model.entries.push(TodoEntry {
                        display: Signal::new(true),
                        completed: false,
                        value: Signal::new(text),
                    });
                }
            }
            Msg::NewTodo(x) => {
                model.new_todo.set(x);
            }
        }
    }
    fn view(&self, model: &Self::Model) -> View<Self::Msg> {v1!{
        h1 {
            "Todo";
        }
        new_todo(model);
        &model.entries;
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


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

pub fn setup() {
	let program = Program::from_component(Component {
        name: String::from("root"),
        spec: AppSpec{},
    });
    program.start();
}

pub fn tick() {
    program_sys::on_request_animation_frame();
}





