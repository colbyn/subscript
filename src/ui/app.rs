use std::fmt;
use std::fmt::Debug;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cell::{self, Cell, RefCell};
use std::sync::Once;
use std::sync::RwLock;
use std::rc::Rc;
use either::Either;
use serde::{self, Serialize, Deserialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::closure;
use wasm_bindgen::closure::Closure;


use crate::browser::{self, Browser, Callback, console};
use crate::ui::html::Html;
use crate::ui::dom::style_mount::*;
use crate::ui::dom;


#[derive(Clone)]
pub struct ApplicationSpec<Model, Msg>
where
    Model: Debug + Clone,
    Msg: Debug + Clone
{
    pub init: Rc<Fn()->(Model, Rc<Fn(String)->Option<Msg>>)>,
    pub update: Rc<Fn(&mut Model, Msg, &Cmd)>,
    pub view: Rc<Fn(&Model)->Html<Msg>>,
}


#[derive(Clone, Debug)]
pub struct Cmd {
    browser: Browser,
}

impl Cmd {
    pub fn navigate(&self, route: &str) {
        self.browser.window
            .history()
            .expect("history failed")
            .push_state_with_url(
                &JsValue::null(),
                "",
                Some(route)
            )
            .expect("pushState failed");
    }
}


#[derive(Clone)]
pub struct AppState<Model, Msg>
where
    Model: Debug + Clone,
    Msg: Debug + Clone,
{
    model: Rc<RefCell<Model>>,
    html: Rc<RefCell<Html<Msg>>>,
}


#[derive(Clone)]
pub struct Application<Model, Msg>
where
    Model: Debug + Clone,
    Msg: Debug + Clone,
{
    browser: Browser,
    js_tick_callback: Rc<RefCell<Option<Callback<()>>>>,
    js_nav_callback: Callback<()>,
    style_mount: StyleMount,
    view_mount: browser::DomRef,
    spec: ApplicationSpec<Model, Msg>,
    state: AppState<Model, Msg>,
    navigation: Rc<Fn(String)->Option<Msg>>,
    app_event_queue: Rc<RefCell<VecDeque<Msg>>>,
}


impl<Model, Msg> Application<Model, Msg>
where
    Model: Debug + Clone + 'static,
    Msg: Debug + Clone + 'static
{
    pub fn new(spec: ApplicationSpec<Model, Msg>) -> Self {
        let browser = Browser::new();
        let view_mount = browser.body.clone();
        let style_mount = StyleMount::new();
        let app_event_queue = Rc::new(RefCell::new(VecDeque::new()));
        let (model, navigation) = spec.init.as_ref()();
        let active_node = spec.view.as_ref()(&model);
        active_node.init(&style_mount);
        view_mount.append_child(
            &active_node
                .get_node()
                .expect("should be a node")
                .dom_ref
        );
        Application {
            js_tick_callback: Rc::new(RefCell::new(None)),
            js_nav_callback: {
                // SET INITIAL NAV MSG
                let initial_path: String = browser.window
                    .location()
                    .pathname()
                    .expect("pathname failed");
                match navigation.as_ref()(initial_path) {
                    Some(msg) => app_event_queue.borrow_mut().push_back(msg),
                    None => (),
                }
                // ON-POP-STATE CALLBACK
                let cb = Callback::new(Rc::new({
                    let app_event_queue = app_event_queue.clone();
                    let navigation = navigation.clone();
                    let browser = browser.clone();
                    move |event| -> Option<()> {
                        let path: String = browser.window
                            .location()
                            .pathname()
                            .expect("pathname failed");
                        match navigation.as_ref()(path) {
                            Some(msg) => app_event_queue.borrow_mut().push_back(msg),
                            None => (),
                        }
                        None
                    }
                }));
                browser.window.set_onpopstate(Some(&cb.js_function));
                cb
            },
            browser: browser,
            style_mount: style_mount,
            view_mount: view_mount,
            spec: spec,
            state: AppState {
                model: Rc::new(RefCell::new(model)),
                html: Rc::new(RefCell::new(active_node)),
            },
            navigation: navigation,
            app_event_queue: app_event_queue,
        }
    }
    pub fn tick(&self) {
        // INIT NEW MODEL >>= APPLY UPDATES >>= UPDATE MODEL
        let mut new_model: Model = self.state.model.borrow().clone();
        let ref cmd = Cmd {
            browser: self.browser.clone()
        };
        for msg in self.app_event_queue.borrow_mut().drain(..) {
            self.spec.update.as_ref()(&mut new_model, msg, cmd);
        }
        for msg in self.state.html.borrow().tick() {
            self.spec.update.as_ref()(&mut new_model, msg, cmd);
        }
        self.state.model.replace(new_model);
        // INIT & SYNC NEW VIEW
        let new_view = self.spec.view.as_ref()(&self.state.model.borrow());
        self.state.html.borrow().sync(&new_view, &self.style_mount);
    }
    pub fn start(&self) {
        let handler: Rc<Fn(JsValue)->Option<()> > = Rc::new({
            let this = self.clone();
            move |_| {
                match this.js_tick_callback.borrow().as_ref() {
                    Some(handler) => {
                        this.tick();
                        this.browser.window.request_animation_frame(
                            &handler.js_function
                        );
                    },
                    None => (),
                }
                None
            }
        });
        let handler: Callback<()> = Callback::new(handler.clone());
        self.js_tick_callback.replace(Some(handler.clone()));
        self.browser.window.request_animation_frame(
            &handler.js_function
        );
    }
}




