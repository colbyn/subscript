use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use wasm_bindgen::JsValue;

use crate::signals_sys::*;
use crate::program_sys::spec::Spec;
use crate::view_sys::shared::*;
use crate::program_sys::instances::*;

///////////////////////////////////////////////////////////////////////////////
// VIEW
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct View<Msg>(pub(crate) Dsl<Msg>);

impl<Msg: 'static> View<Msg> {
    pub fn new_text(value: &str) -> Self {
        View(Dsl::Text(Text(Value::Static(String::from(value)))))
    }
    pub fn new_text_signal(cell: &Reactive<String>) -> Self
    {
        let observer: CellObserver<String> = CellObserver::new(cell);
        let value = match cell.get_either_val_or_ref() {
            Left(x) => x,
            Right(x) => x.clone(),
        };
        View(Dsl::Text(Text(Value::Dynamic(DynamicValue {
            observer,
            current: RefCell::new(value),
        }))))
    }
    pub fn new_element(tag: &str) -> Self {
        View(Dsl::Element(Element {
            tag: String::from(tag),
            styling: Styling::default(),
            attributes: HashMap::new(),
            events: Vec::new(),
            children: Vec::new(),
        }))
    }
    pub fn new_mixin() -> Self {
        View(Dsl::Mixin(Mixin {
            styling: Styling::default(),
            attributes: HashMap::new(),
            events: Vec::new(),
            children: Vec::new(),
        }))
    }
    pub fn new_linked_control<T: 'static>(vec: &VecSignal<T>, init: impl Fn(&T)->View<Msg> + 'static) -> Self {
        let observer = ViewVecObserver::new(&vec, init);
        View(Dsl::Control(Control::Linked(observer)))
    }
    pub fn new_toggle_control(pred: &Reactive<bool>, value: View<Msg>) -> Self {
        let pred = CellObserver::new(pred);
        View(Dsl::Control(Control::Toggle {
            pred,
            value: Rc::new(value),
        }))
    }
    pub fn new_component<S: Spec + 'static >(name: &str, spec: S) -> Self {
        View(Dsl::Component(SubComponent(Rc::new(Component {
            name: String::from(name),
            spec,
        }))))
    }
    pub fn text(&mut self, value: &str) {
        self.push_child(View::new_text(value));
    }
    pub fn text_cell(&mut self, value: &Reactive<String>) {
        self.push_child(View::new_text_signal(value));
    }
    pub fn tag(&mut self, tag: &str, inner: impl FnMut(&mut View<Msg>)) {
        if let Some(env) = self.get_env() {
            let mut new_element = View::new_element(tag);
            let mut inner = Box::new(inner);
            inner(&mut new_element);
            env.children.push(new_element);
        }
    }
    pub fn push_child(&mut self, child: View<Msg>) {
        if let Some(env) = self.get_env() {
            env.children.push(child);
        }
    }
    pub fn add_styling(&mut self, new: Styling) {
        if let Some(env) = self.get_env() {
            env.styling.extend(new);
        }
    }
    pub fn get_env<'a>(&'a mut self) -> Option<ViewEnv<'a, Msg>> {
        match &mut self.0 {
            Dsl::Element(element) => {
                Some(ViewEnv {
                    styling: &mut element.styling,
                    attributes: &mut element.attributes,
                    events: &mut element.events,
                    children: &mut element.children,
                })
            }
            Dsl::Mixin(mixin) => {
                Some(ViewEnv {
                    styling: &mut mixin.styling,
                    attributes: &mut mixin.attributes,
                    events: &mut mixin.events,
                    children: &mut mixin.children,
                })
            }
            Dsl::Control(Control::Toggle{pred, value}) => {
                // Should this be None?
                unimplemented!()
            }
            Dsl::Control(Control::Linked(sub)) => {
                // Should this be None?
                unimplemented!()
            }
            Dsl::Component(component) => None,
            Dsl::Text(text) => None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// VIEW - INTERNAL
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(crate) enum Dsl<Msg> {
    Component(SubComponent),
    Text(Text),
    Element(Element<Msg>),
    Mixin(Mixin<Msg>),
    Control(Control<Msg>),
}

#[derive(Debug)]
pub(crate) struct Text(pub Value<String>);

#[derive(Debug)]
pub(crate) struct Element<Msg> {
    pub(crate) tag: String,
    pub(crate) styling: Styling,
    pub(crate) attributes: HashMap<String, Either<Value<String>, Value<bool>>>,
    pub(crate) events: Vec<EventHandler<Msg>>,
    pub(crate) children: Vec<View<Msg>>,
}

#[derive(Debug)]
pub(crate) struct Mixin<Msg> {
    pub(crate) styling: Styling,
    pub(crate) attributes: HashMap<String, Either<Value<String>, Value<bool>>>,
    pub(crate) events: Vec<EventHandler<Msg>>,
    pub(crate) children: Vec<View<Msg>>,
}

#[derive(Debug)]
pub(crate) enum Control<Msg> {
    Linked(ViewVecObserver<Msg>),
    Toggle {
        pred: CellObserver<bool>,
        value: Rc<View<Msg>>,
    },
}


impl std::fmt::Debug for SubComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SubComponent")
    }
}


pub struct ViewEnv<'a, Msg> {
    pub(crate) styling: &'a mut Styling,
    pub(crate) attributes: &'a mut HashMap<String, Either<Value<String>, Value<bool>>>,
    pub(crate) events: &'a mut Vec<EventHandler<Msg>>,
    pub(crate) children: &'a mut Vec<View<Msg>>,
}


