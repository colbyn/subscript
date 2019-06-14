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
use crate::program_sys::instances::Component;
use crate::program_sys::spec::*;
use crate::program_sys::{self, Program};


///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppSpec {

}

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    // entries: VecSignal<TodoItem>
}

#[derive(Debug)]
pub enum Msg {
	NoOp
}

#[derive(Default)]
pub struct TodoItem {
    value: Signal<String>,
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
        
    }
    fn view(&self, model: &Self::Model) -> View<Self::Msg> {v1!{
        h1 {
            "Todo";
        }
        // new_todo(model);
    }}
}

pub fn new_todo(model: &Model) -> View<Msg> {v1!{
    form {
        input {
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





