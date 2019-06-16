pub mod spec;
pub mod instances;
#[macro_use]
pub mod macros;

use std::collections::*;
use std::rc::*;
use std::any::*;
use std::cell::*;
use wasm_bindgen::prelude::*;

use crate::view_sys::dsl::View;
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


impl ProgramImpl for Program {
    fn tick(&mut self) {
        let messages: Vec<SystemMessage> = GLOABL_MESSAGE_REGISTRY.with(|reg| {
            reg.borrow_mut().drain(..).collect::<Vec<SystemMessage>>()
        });
        self.process.0.tick(&messages);
    }
}

///////////////////////////////////////////////////////////////////////////////
// PROGRAM
///////////////////////////////////////////////////////////////////////////////

pub struct Program {
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
        let root_component = Component {
            name: String::from("Root Component"),
            spec,
        };
        let process = SubProcess(Box::new(root_component.build_impl()));
        let program: Program = Program{
            process,
        };
        ROOT_PROCESS.with(move |cell| {
            let old = cell.replace(Some(Box::new(program)));
            assert!(old.is_none());
        });
    }
}

// impl<Model, Msg> Program<ImmediateModeApp<Model, Msg>> {
//     pub fn run(args: ImmediateMode<impl Fn(Option<Model>)-> Model, impl Fn(&mut Model, Msg), impl Fn(&Model) -> View<Msg>>) {
//         unimplemented!()
//     }
// }



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
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, sys: &mut SubSystems<Self>) {
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

