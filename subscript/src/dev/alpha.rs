use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi};
use crate::model::{incremental::*, reactive::*};
use crate::view::runtime::common::ElementEnv;
use crate::view::{dom, dsl, runtime, dom::{Dom, Element}, dsl::{View, Dsl}};

#[derive(Debug)]
pub enum Msg {
	NoOp
}

pub fn view() -> View<Msg> {
	let mut root = View::new_element("main");
	let toggle_signal = Signal::new(false);
	let mut toggle_pane = View::new_element("section");
	toggle_pane.tag("h1", |h1| {
		h1.text("Hello World")
	});
	root.push_child(View::new_toggle_control(&toggle_signal, toggle_pane));
	std::mem::forget(toggle_signal.clone());
	let timeout = browser::window().set_timeout(3000, {
		let mut toggle_signal = toggle_signal;
		move || {
			console!("timeout");
			toggle_signal.set(true);
		}
	});
	std::mem::forget(timeout);
	root
}


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
	let process = Process::new(view());
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





