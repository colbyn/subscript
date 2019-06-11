use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi};
use crate::model_sys::{incremental::*, reactive::*};
use crate::view_sys::runtime::common::ElementEnv;
use crate::view_sys::shared::*;
use crate::view_sys::{dom, dsl, runtime, dom::{Dom, Element}, dsl::{View, Dsl}};

#[derive(Debug, Default)]
pub struct Model {

}

#[derive(Debug)]
pub enum Msg {
	NoOp
}

pub fn view(model: &Model) -> View<Msg> {v1!{
    h1 {
    	"Hello World";
    }
}}


///////////////////////////////////////////////////////////////////////////////
// PROGRAM
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub(crate) static ROOT_PROCESS: std::cell::RefCell<Option<Process<Msg>>> = {
    	std::cell::RefCell::new(None)
    };
}

pub struct Process<Msg> {
	dom: std::cell::RefCell<Dom<Msg>>,
}

impl<Msg> Process<Msg> {
	fn new(view: View<Msg>) -> Self {
		let dom = std::cell::RefCell::new(view.build_root());
		Process{dom}
	}
	fn tick(&self) {
		self.dom.borrow_mut().unsafe_tick_root();
	}
}



///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

pub fn setup() {
	let model = Model::default();
	let process = Process::new(view(&model));
	ROOT_PROCESS.with(|cell| {
		cell.replace(Some(process));
	});
}

pub fn tick() {
	ROOT_PROCESS.with(|cell| {
		let inner: &Option<Process<Msg>> = &cell.borrow();
		if let Some(process) = inner {
			process.tick();
		}
	});
}





