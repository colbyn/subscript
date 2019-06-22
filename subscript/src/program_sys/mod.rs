#[macro_use]
pub mod effect;
pub mod spec;
pub mod instances;
pub mod shell;

use std::collections::*;
use std::rc::*;
use std::any::*;
use std::cell::*;
use wasm_bindgen::prelude::*;

use crate::backend::browser;
use crate::view_sys::dsl::View;
use crate::program_sys::spec::*;
use crate::program_sys::instances::*;
use crate::view_sys::shared::*;
use crate::program_sys::shell::*;
pub use crate::program_sys::effect::nav::{Url};



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
// NAVIGATION
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub(crate) static CURRENT_URL: RefCell<Option<Url>> = {
        RefCell::new(None)
    };
}

fn init_url_callback() {
    let callback_settings = browser::CallbackSettings{
        prevent_default: true,
        ..browser::CallbackSettings::default()
    };
    let callback = browser::VoidCallback::new_with_fn_unset(callback_settings, move |_| {
        CURRENT_URL.with(|cell| {
            let new_url: Url = Url::get_current(&browser::window());
            cell.replace(Some(new_url));
        });
    });
    browser::window().add_event_listener("popstate", &callback);
    std::mem::forget(callback);
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


impl ProgramImpl for Program {
    fn tick(&mut self) {
        let url_state: Option<Url> = CURRENT_URL.with(|url| {
            url.borrow().clone()
        });
        let url_state: Url = if let Some(url_state) = url_state {
            url_state
        } else {
            self.url.clone()
        };
        let url_unchanged = &self.url == &url_state;
        let mut messages: Vec<SystemMessage> = GLOABL_MESSAGE_REGISTRY.with(|reg| {
            reg.borrow_mut().drain(..).collect::<Vec<SystemMessage>>()
        });
        if !url_unchanged {
            CURRENT_URL.with(|cell| {
                let new_url = Url::get_current(&browser::window());
                cell.replace(Some(new_url));
            });
            messages.push({
                let value: Rc<UrlChanged> = Rc::new(UrlChanged(url_state.clone()));
                SystemMessage::Public {
                    from_name: String::from(""),
                    from_tid: TypeId::of::<Program>(),
                    value,
                }
            });
            self.url = url_state;
        };
        self.process.0.tick(&messages);
    }
}

///////////////////////////////////////////////////////////////////////////////
// PROGRAM
///////////////////////////////////////////////////////////////////////////////

pub struct Program {
    url: Url,
    process: SubProcess,
}

impl Program {
    pub fn run_basic<Model, Msg>(
        init: impl Fn(Option<Model>)-> Model + 'static,
        update: impl Fn(&mut Model, Msg) + 'static,
        view: impl Fn(&Model) -> View<Msg> + 'static,
    ) where
        Model: 'static,
        Msg: 'static,
    {
        Program::run_spec(SimpleApp {
            init: Rc::new(init),
            update: Rc::new(update),
            view: Rc::new(view),
        })
    }
    pub fn run_spec(spec: impl Spec + 'static) {
        let window = browser::window();
        let root_component = Component {
            name: String::from("Root Component"),
            spec,
        };
        let process = SubProcess(Box::new(root_component.build_impl()));
        let program: Program = Program{
            url: Url::get_current(&window),
            process,
        };
        ROOT_PROCESS.with(move |cell| {
            let old = cell.replace(Some(Box::new(program)));
            assert!(old.is_none());
        });
    }
}





///////////////////////////////////////////////////////////////////////////////
// IMMEDIATE-MODE-APP
///////////////////////////////////////////////////////////////////////////////

pub(crate) struct SimpleApp<Model, Msg> {
    init: Rc<Fn(Option<Model>)-> Model>,
    update: Rc<Fn(&mut Model, Msg)>,
    view: Rc<Fn(&Model) -> View<Msg>>,
}

impl<Model, Msg> Spec for SimpleApp<Model, Msg> {
    type Msg = Msg;
    type Model = Model;
    
    fn init(&self, startup: StartupInfo<Self>) -> Init<Self> {
        let inner = (self.init)(None);
        Init {
            model: inner,
            subs: Subscriptions::default(),
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, sys: &mut Shell<Self>) {
        (self.update)(model, msg);
    }
    fn view(&self, model: &Self::Model) -> View<Self::Msg> {
        (self.view)(model)
    }
}

impl<Model, Msg> Clone for SimpleApp<Model, Msg> {
    fn clone(&self) -> Self {
        let init = self.init.clone();
        let update = self.update.clone();
        let view = self.view.clone();
        SimpleApp{init,update,view}
    }
}

