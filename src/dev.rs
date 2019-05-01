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

use crate::browser::console;
use crate::ui::html::*;
use crate::ui::effect::nav::Navigation;
#[macro_use]
use crate::ui::html::macros;
use crate::ui::app::*;


///////////////////////////////////////////////////////////////////////////////
// APP DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Page {
    Homepage,
    Content,
    Account,
    NotFound
}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
    UrlChanged(Page),
    UrlRequest(Page),
}

#[derive(Debug, Clone)]
pub struct Model {
    page: Option<Page>
}


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION
///////////////////////////////////////////////////////////////////////////////

pub fn init() -> (Model, Navigation<Msg>) {
    let initial_model = Model {
        page: None,
    };
    let navigation = match_path!(
        [] => {
            Msg::UrlChanged(Page::Homepage)
        }
        ["content"] => {
            Msg::UrlChanged(Page::Content)
        }
        ["account"] => {
            Msg::UrlChanged(Page::Account)
        }
        _ => {
            Msg::UrlChanged(Page::NotFound)
        }
    );
    (initial_model, navigation)
}

pub fn update(model: &mut Model, msg: Msg, cmd: &Cmd) {
    match msg {
        Msg::UrlChanged(page) => {
            model.page = Some(page);
        }
        Msg::UrlRequest(page) => {
            match &page {
                Page::Homepage => cmd.navigate("/"),
                Page::Content => cmd.navigate("/content"),
                Page::Account => cmd.navigate("/account"),
                Page::NotFound => cmd.navigate("/not-found"),
            };
            model.page = Some(page);
        }
        Msg::NoOp => {}
    }
}

pub fn view(model: &Model) -> Html<Msg> {
    let navigation = view!(nav.ul|
        li(
            on.click = move |event| {
                Msg::UrlRequest(Page::Homepage)
            },
            a(text("Homepage"))
        ),
        li(
            on.click = move |event| {
                Msg::UrlRequest(Page::Content)
            },
            a(text("Content"))
        ),
        li(
            on.click = move |event| {
                Msg::UrlRequest(Page::Account)
            },
            a(text("Account"))
        )
    );
    let homepage = view!(
        navigation,
        h1(text("Homepage"))
    );
    let content = view!(
        navigation,
        h1(text("Content"))
    );
    let account = view!(
        navigation,
        h1(text("Account"))
    );
    let not_found = view!(
        navigation,
        h1(text("Not Found"))
    );
    let loading = view!(
        navigation,
        h1(text("Loading"))
    );
    match &model.page {
        Some(Page::Homepage) => homepage,
        Some(Page::Content) => content,
        Some(Page::Account) => account,
        Some(Page::NotFound) => not_found,
        None => loading,
    }
}


// GO!
pub fn run() {
    let app = Application::new(ApplicationSpec {
        init: Rc::new(init),
        update: Rc::new(update),
        view: Rc::new(view),
    });
    app.start();
}


