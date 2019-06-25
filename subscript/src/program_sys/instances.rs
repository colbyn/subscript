use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use wasm_bindgen::JsValue;

use crate::backend::browser;
use crate::reactive_sys::*;
use crate::view_sys::dom::Dom;
use crate::program_sys::spec::*;
use crate::program_sys::shell::*;
use crate::program_sys::effect::nav::*;


///////////////////////////////////////////////////////////////////////////////
// COMPONENTS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub(crate) struct SubComponent(pub(crate) Rc<SubComponentImpl>);
pub(crate) trait SubComponentImpl {
    fn build(&self) -> SubProcess;
}

#[derive(Clone)]
pub struct Component<S: Spec> {
    pub name: String,
    pub spec: S
}

impl<S: Spec> Component<S> {
    pub fn new(name: &str, spec: S) -> Self {
        let name = String::from(name);
        Component {name, spec}
    }
}


impl SubComponent {
    pub(crate) fn build(&self) -> SubProcess {
        self.0.as_ref().build()
    }
}
impl std::fmt::Debug for SubComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SubComponent")
    }
}
impl<S: 'static +  Spec> SubComponentImpl for Component<S> {
    fn build(&self) -> SubProcess {
        SubProcess(Box::new(self.build_impl(false)))
    }
}

impl<S: Spec + 'static> Component<S> {
    pub(crate) fn build_impl(&self, is_root: bool) -> Process<S> {
        let window = browser::window();
        let component = self.clone();
        let mut sub_systems = Shell {
            instance_name: component.name.clone(),
            commands: RefCell::new(VecDeque::new()),
            mark: PhantomData,
        };
        let init = component.spec.init(&sub_systems);
        assert!(sub_systems.commands.try_borrow().is_ok());
        assert!(sub_systems.commands.borrow().is_empty());
        let model = init.model;
        let view = component.spec.view(&model);
        let dom = view.build_component(is_root);
        process_system_requests(&component.name, &model, &mut sub_systems);
        Process {
            name: component.name,
            spec: component.spec,
            subs: init.subs,
            model,
            sub_systems,
            dom: Some(dom),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// PROCESS
///////////////////////////////////////////////////////////////////////////////

pub(crate) struct SubProcess(pub(crate) Box<SubProcessImpl>);
pub(crate) trait SubProcessImpl {
    fn dom_ref(&self) -> browser::Element;
    fn tick(&mut self, messages: &Vec<SystemMessage>);
}

pub(crate) struct Process<S: Spec> {
    name: String,
    spec: S,
    subs: Subscriptions<S::Msg>,
    model: S::Model,
    dom: Option<Dom<S::Msg>>,
    sub_systems: Shell<S>,
}


impl<S: Spec> Process<S> {
    fn get_dom_mut(&mut self) -> &mut Dom<S::Msg> {
        match &mut self.dom {
            Some(dom) => dom,
            None => panic!()
        }
    }
}
impl std::fmt::Debug for SubProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SubProcess")
    }
}
impl<S: Spec> Drop for Process<S> {
    fn drop(&mut self) {
        console!("Process.Drop");
        match self.dom.take() {
            Some(dom) => {
                dom.unsafe_remove_root();
            }
            _ => panic!()
        }
    }
}

impl<S: Spec + 'static> SubProcessImpl for Process<S> {
    fn dom_ref(&self) -> browser::Element {
        match &self.dom {
            Some(dom) => dom.unsafe_get_element().dom_ref.clone(),
            None => panic!()
        }
    }
    fn tick(&mut self, messages: &Vec<SystemMessage>) {self.tick_impl(messages);}
}


///////////////////////////////////////////////////////////////////////////////
// PROCESS TICK
///////////////////////////////////////////////////////////////////////////////

pub(crate) struct TickEnv<'a, Msg> {
    pub system_messages: &'a Vec<SystemMessage>,
    pub local_messages: &'a mut Vec<Msg>,
}

impl<S: Spec + 'static> Process<S> {
    fn tick_impl(&mut self, system_messages: &Vec<SystemMessage>) {
        // TICK
        let local_messages = {
            let mut local_messages = Vec::new();
            let mut tick_env = TickEnv {
                system_messages,
                local_messages: &mut local_messages,
            };
            // TICK
            self.get_dom_mut().unsafe_tick_root(&mut tick_env);
            self.subs.tick::<S>(self.name.as_str(), &mut tick_env);
            local_messages
        };
        // UPDATE
        for msg in local_messages {
            self.spec.update(&mut self.model, msg, &mut self.sub_systems);
        }
        process_system_requests(&self.name, &self.model, &mut self.sub_systems);
    }
}
