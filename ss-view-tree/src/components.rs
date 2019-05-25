use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::any::*;
use std::marker::Sized;
use either::Either::{self, Left, Right};
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};

use crate::attributes::*;
use crate::events::*;
use ss_trees::tree::*;
use ss_trees::tree::map::*;
use ss_css_types::api::*;

pub type InternalComponentId = Any;

pub trait ViewComponent {
    fn box_clone(&self) -> Box<ViewComponent>;
    fn tick(&self, reg: &GlobalTickRegistry) -> Box<InternalComponentId>;
    fn unchanged(&self, other: &Box<Any>) -> bool;
    fn recyclable(&self, other: &Box<Any>) -> bool;
}

impl Clone for Box<ViewComponent>
{
    fn clone(&self) -> Box<ViewComponent> {
        self.box_clone()
    }
}
impl PartialEq for ViewComponent {
    fn eq(&self, other: &ViewComponent) -> bool {
        // self.spec_type_id() == other.spec_type_id()
        unimplemented!()
    }
}
impl Debug for ViewComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "QueueCallback")
    }
}


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////


pub struct GlobalTickRegistry {
    pub components: RefCell<Vec<Box<InternalComponentId>>>,
}

impl Default for GlobalTickRegistry {
    fn default() -> Self {
        let components = RefCell::new(Vec::new());
        GlobalTickRegistry {components}
    }
}

pub struct TickEnv<Msg> {
    pub messages: Vec<Msg>,
}

impl<Msg> Default for TickEnv<Msg> {
    fn default() -> Self {
        let messages = Vec::new();
        TickEnv{messages}
    }
}
