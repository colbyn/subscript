#![allow(dead_code, unused, unused_variables)]
pub mod spec;
pub mod process;

use std::any::*;
use std::rc::*;
use std::sync::*;
use std::cell::*;
use wasm_bindgen::JsValue;
use ss_web_utils::{dom, js, js::console};
use ss_view_tree::components::*;
pub use spec::*;
pub use process::*;
use ss_cssom_tree::GLOBAL_CSS_REGISTRY;


#[derive(Clone)]
pub struct Program<S: Spec> {
    window: dom::Window,
    global_tick_registry: Rc<GlobalTickRegistry>,
    js_tick_callback: Rc<RefCell<Option<js::VoidCallback>>>,
    root_component: Rc<Component<S>>,
}

impl<S: 'static +   Spec> Program<S> {
    pub fn from_component(root_component: Component<S>) -> Self {
        let window = dom::window();
        let js_tick_callback = Rc::new(RefCell::new(None));
        let root_component = Rc::new(root_component);
        let global_tick_registry = Rc::new(GlobalTickRegistry::default());
        Program {window, js_tick_callback, root_component, global_tick_registry}
    }
    pub fn tick(&self) {
        if self.window.document.ready_state().is_dom_ready() {
            assert!(self.global_tick_registry.components.borrow().is_empty());
            self.global_tick_registry.components.borrow_mut().push(
                self.root_component.tick(&self.global_tick_registry)
            );
            PROCESS_REGISTRY.with(move |proc_reg| {
                proc_reg.borrow_mut().prune_and_tick_global_events(&self.global_tick_registry);
            });
            self.global_tick_registry.components.borrow_mut().clear();
            GLOBAL_CSS_REGISTRY.with(move |css_reg| {
                css_reg.borrow_mut().sync();
            });
        }
    }
    pub fn start(self) {
        let window = dom::window();
        let handler: js::VoidCallback = js::VoidCallback::new({
            let this = self.clone();
            move |_| {
                this.tick();
                dom::window().request_animation_frame(
                    this.js_tick_callback
                        .borrow()
                        .as_ref()
                        .expect("failed to tick")
                );
            }
        });
        self.js_tick_callback.replace(Some(handler.clone()));
        dom::window().request_animation_frame(&handler);
        std::mem::forget(self);
    }
}



