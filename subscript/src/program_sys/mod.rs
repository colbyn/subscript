pub mod spec;

use std::collections::*;
use std::rc::*;
use std::any::*;
use std::cell::*;

use crate::program_sys::spec::*;
use crate::view_sys::shared::*;

///////////////////////////////////////////////////////////////////////////////
// GLOABL MESSAGES
///////////////////////////////////////////////////////////////////////////////
thread_local! {
    pub(crate) static GLOABL_MESSAGES: RefCell<VecDeque<Message>> = {
        RefCell::new(VecDeque::new())
    };
}

pub enum Message {
    Public(Rc<Any>),
    Private {
        tid: TypeId,
        value: Rc<Any>,
    },
}

///////////////////////////////////////////////////////////////////////////////
// ROOT-PROCESS GLOBAL
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub(crate) static ROOT_PROCESS: RefCell<Option<Box<ProgramImpl>>> = {
        RefCell::new(None)
    };
}

pub trait ProgramImpl {
    fn tick(&mut self);
}


impl<S: Spec> ProgramImpl for Program<S> {
    fn tick(&mut self) {
        self.process.tick();
    }
}

///////////////////////////////////////////////////////////////////////////////
// PROGRAM
///////////////////////////////////////////////////////////////////////////////

pub struct Program<S: Spec> {
    process: Process<S>,
}


