pub mod data;

use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi};
use crate::reactive_sys::*;
use crate::view_sys::runtime::common::ElementEnv;
use crate::view_sys::shared::*;
use crate::view_sys::{dom, dsl, runtime, dom::{Dom, Element}, dsl::{View, Dsl}};
use crate::view_sys::adapters::*;
use crate::program_sys::instances::Component;
use crate::program_sys::spec::*;
use crate::program_sys::{self, Program};

use crate::dev::cms_app::client::data::*;

///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppSpec {

}

pub enum Msg {
    NoOp,
    UrlChanged(Page),
    UrlRequest(Page),
    NewSession(Session),
}

#[derive(Default)]
pub struct Model {
    page: Signal<Page>,
    session: Signal<Option<Session>>,
}

#[derive(Clone)]
pub struct UrlRequest(Page);

///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////
// VIEWABLE DATA TYPES
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Spec for AppSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, startup: StartupInfo<Self>) -> Init<Self> {
        let url_parser: UrlParser<Page> = url_parser!{
            [] => {
                Page::Homepage
            }
            ["content"] => {
                Page::Content
            }
            ["analytics"] => {
                Page::Analytics
            }
            ["account"] => {
                Page::Account(AccountPage::default())
            }
            ["login"] => {
                Page::Login
            }
            ["signup"] => {
                Page::Signup
            }
            _ => {
                Page::NotFound
            }
        };
        let model = Model {
            page: Signal::new(url_parser.parse(&startup.current_url)),
            session: Signal::new(None),
        };
        let subs = subs!{
            msg url_changed(value: UrlChanged) -> Msg {
                Msg::UrlChanged(
                    url_parser
                        .clone()
                        .parse(&value.url())
                )
            }
            msg url_request(value: UrlRequest) -> Msg {
                Msg::UrlRequest(value.0)
            }
        };
        Init{subs, model, ..Default::default()}
    }
    fn update(&self, model: &mut Model, msg: Msg, sh: &mut Shell<Self>) {
        match msg {
            Msg::NoOp => {}
            Msg::UrlChanged(page) => {
                console!("Msg::UrlChanged");
                model.page.set(page);
            }
            Msg::UrlRequest(page) => {
                sh.navigate(page);
            }
            Msg::NewSession(session) => {

            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        text_theme();
        overflow: "auto";
        width: "100%";
        height: "100%";
        background_color: "#efefef";
        display: "flex";
        flex_direction: "column";
        navigation(model);
        page(model);
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn navigation(model: &Model) -> View<Msg> {
    let nav_link = |txt: &str, page: Page| -> View<Msg> {v1!{
        li {
            margin_right: "10px";
            css.last_child => s1!{
                margin_right: "0";
            };
            a {
                bind[page] &model.page => move |active| {
                    if active == &page {v1!{
                        color: "#fff";
                    }}
                    else {v1!{
                        color: "#000";
                    }}
                };
                event.click[page] => move || {
                    Msg::UrlRequest(page)
                };
                txt;
            };
        };
    }};
    v1!{
        header {
            padding: "8px";
            background_color: "#000";
            display: "flex";
            justify_content: "space-between";
            span {
                "LOGO.IO";
            };
            ul {
                list_style: "none";
                display: "flex";
                padding: "0";
                margin: "0";

                nav_link("Content", Page::Content);
                nav_link("Analytics", Page::Analytics);
                nav_link("Account", Page::Account(AccountPage::default()));
                nav_link("Signup", Page::Signup);
                nav_link("Login", Page::Login);
                // if &model.session.map(|x| x.is_some()) => {
                //     nav_link("Content", Page::Content);
                //     nav_link("Analytics", Page::Analytics);
                //     nav_link("Account", Page::Account(AccountPage::default()));
                // };
                // if &model.session.map(|x| x.is_none()) => {
                //     nav_link("Signup", Page::Signup);
                //     nav_link("Login", Page::Login);
                // };
            };
        };
    }
}

pub fn page(model: &Model) -> View<Msg> {v1!{
    bind &model.page => move |page| {
        match page {
            Page::Homepage => v1!{
                h1 {
                    "Homepage";
                };
            },
            Page::Content => v1!{
                h1 {
                    "Content";
                };
            },
            Page::Analytics => v1!{
                h1 {
                    "Analytics";
                };
            },
            Page::Account(accunt_page) => v1!{
                h1 {
                    "Account";
                };
            },
            Page::Login => v1!{
                h1 {
                    "Login";
                };
            },
            Page::Signup => v1!{
                h1 {
                    "Signup";
                };
            },
            Page::NotFound => v1!{
                h1 {
                    "NotFound";
                };
            },
        }
    };
}}


///////////////////////////////////////////////////////////////////////////////
// VIEW AGNOSTIC UTILS
///////////////////////////////////////////////////////////////////////////////

pub fn text_theme<Msg: 'static>() -> View<Msg> {v1!{
    font_family: "'Source Sans Pro', sans-serif";
    color: "#777";
    font_weight: "200";
}}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

pub fn setup() {
    Program::run_spec(AppSpec{

    });
}

pub fn tick() {
    program_sys::on_request_animation_frame();
}
