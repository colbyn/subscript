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
use crate::tree::online::data::*;
use crate::process::basics::*;
use crate::process::online::*;
use crate::process::registry::*;


pub trait OfflineProcessApi {
    fn spec_type_id(&self) -> TypeId;
    fn box_clone(&self) -> Box<OfflineProcessApi>;
    fn spawn(&self) -> Box<ProcessHandle>;
}

impl Clone for Box<OfflineProcessApi>
{
    fn clone(&self) -> Box<OfflineProcessApi> {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct OfflineProcess<S: Spec> {
    pub spec: S,
}

impl<S: Spec> OfflineProcess<S> {
    pub fn from_spec(spec: S) -> Self {
        OfflineProcess {
            spec: spec
        }
    }
}

impl<S: Spec> OfflineProcessApi for OfflineProcess<S> {
    fn spec_type_id(&self) -> TypeId {
        TypeId::of::<S>()
    }
    fn box_clone(&self) -> Box<OfflineProcessApi> {
        Box::new(self.clone())
    }
    fn spawn(&self) -> Box<ProcessHandle> {
        GLOBAL_REGISTRY.with(|reg| {
            match reg.get_copy_of_process(&self.spec_type_id()) {
                None => Box::new(Process::from_spec(self.spec.clone())),
                Some(process) => process
            }
        })
    }
}


