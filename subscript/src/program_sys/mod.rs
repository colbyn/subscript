pub mod spec;
pub mod instances;
#[macro_use]
pub mod macros;

use std::collections::*;
use std::rc::*;
use std::any::*;
use std::cell::*;

use wasm_bindgen::prelude::*;

use crate::program_sys::spec::*;
use crate::program_sys::instances::*;
use crate::view_sys::shared::*;



///////////////////////////////////////////////////////////////////////////////
// ROOT-PROCESS TICK - EXTERNAL API
///////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen]
pub fn on_request_animation_frame() {
    ROOT_PROCESS.with(|cell| {
        let inner: Option<Box<ProgramImpl>> = cell.replace(None);
        if let Some(mut process) = inner {
            process.tick();
            assert!(cell.replace(Some(process)).is_none());
        }
    });
}


///////////////////////////////////////////////////////////////////////////////
// ROOT-PROCESS TICK - INTERNAL
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub(crate) static ROOT_PROCESS: RefCell<Option<Box<ProgramImpl>>> = {
        RefCell::new(None)
    };
}

pub(crate) trait ProgramImpl {
    fn tick(&mut self);
}


impl<S: Spec + 'static> ProgramImpl for Program<S> {
    fn tick(&mut self) {
        let messages: Vec<SystemMessage> = GLOABL_MESSAGE_REGISTRY.with(|reg| {
            reg.borrow_mut().drain(..).collect::<Vec<SystemMessage>>()
        });
        self.process.tick(&messages);
    }
}

///////////////////////////////////////////////////////////////////////////////
// PROGRAM
///////////////////////////////////////////////////////////////////////////////

pub struct Program<S: Spec> {
    process: Process<S>,
}


impl<S: Spec + 'static> Program<S> {
    pub fn from_component(root_component: Component<S>) -> Self {
        let process = root_component.build_impl();
        Program {process}
    }
    pub fn start(self) {
        ROOT_PROCESS.with(move |cell| {
            let old = cell.replace(Some(Box::new(self)));
            assert!(old.is_none());
        });
    }
}

