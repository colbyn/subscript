use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::any::*;
use std::marker::Sized;
use either::Either;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use wasm_bindgen::convert::*;

use crate::browser::*;
use crate::tree::offline::data::*;
use crate::tree::online::data::*;

use crate::process::basics::*;
use crate::process::offline::*;
use crate::process::online::*;


///////////////////////////////////////////////////////////////////////////////
// INTERNAL GLOBAL REGISTRY
///////////////////////////////////////////////////////////////////////////////

type SpecTypeId = TypeId;

thread_local! {
    pub static GLOBAL_REGISTRY: GlobalRegistry = {
        GlobalRegistry {
            events: RefCell::new(Vec::new()),
            processes: RefCell::new(HashMap::new()),
        }
    };
}

pub struct GlobalRegistry {
    pub events: RefCell<Vec<Rc<Any>>>,
    pub processes: RefCell<HashMap<SpecTypeId, Box<ProcessHandle>>>,
}

impl GlobalRegistry {
    pub fn drain_events(&self) -> Vec<Rc<Any>> {
        self.events
            .borrow_mut()
            .drain(..)
            .collect::<Vec<Rc<Any>>>()
    }
    pub fn add_event(&self, event: Rc<Any>) {
        self.events.borrow_mut().push(event);
    }
    pub fn add_process<S: Spec>(&self, process: Process<S>) {
        if self.process_exists(&process.spec_type_id()) {
            panic!("Process for spec already exists");
        } else {
            self.processes
                .borrow_mut()
                .insert(process.spec_type_id(), Box::new(process));
        }
    }
    pub fn process_exists(&self, id: &SpecTypeId) -> bool {
        self.processes
            .borrow()
            .contains_key(id)
    }
    pub fn delete_process(&self, id: &SpecTypeId) {
        self.processes
            .borrow_mut()
            .remove(id);
    }
    pub fn get_copy_of_process(&self, id: &SpecTypeId) -> Option<Box<ProcessHandle>> {
        match self.processes.borrow().get(&id) {
            None => None,
            Some(ph) => Some(ph.box_clone()),
        }
    }
}



