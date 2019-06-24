pub mod data;
pub mod login;

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
use crate::dev::cms_app::client::login::LoginSpec;


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
                Page::Login(LoginPage::Login)
            }
            ["signup"] => {
                Page::Login(LoginPage::Signup)
            }
            _ => {
                Page::NotFound
            }
        };
        let session: Signal<Option<Session>> = Signal::new(None);
        let mut page = Signal::new(url_parser.parse(&startup.current_url));
        if session.get_copy().is_none() && !page.get().is_login() {
            page.set(Page::Login(LoginPage::default()));
        }
        let model = Model {page,session};
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
        if &model.page.map(move |x| x.is_login()) => {
            background_color: "hsl(0, 0%, 86%) !important";
        };
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
            display: "block";
            margin_right: "16px";
            padding_top: "3px";
            bind[page] &model.page => move |active| {
                if active == &page {v1!{
                    border_bottom: "3px solid #0089ff";
                }}
                else {v1!{
                    border_bottom: "3px solid transparent";
                }}
            };
            a {
                user_select: "none";
                display: "block";
                color: "#fff";
                font_weight: "500";
                padding: "8px";
                bind[page] &model.page => move |active| {
                    if active == &page {v1!{

                    }}
                    else {v1!{
                        css.hover => s1!{
                            color: "#777";
                        };
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
            background_color: "hsl(0, 0%, 24%)";
            display: "flex";
            justify_content: "space-between";
            align_items: "center";
            span {
                user_select: "none";
                margin_left: "16px";
                color: "#fff";
                "LOGO.IO";
            };
            ul {
                list_style: "none";
                display: "flex";
                padding: "0";
                margin: "0";
                if &model.session.map(|x| x.is_some()) => {
                    nav_link("Content", Page::Content);
                    nav_link("Analytics", Page::Analytics);
                    nav_link("Account", Page::Account(AccountPage::default()));
                };
                if &model.session.map(|x| x.is_none()) => {
                    nav_link("Signup", Page::Login(LoginPage::Signup));
                    nav_link("Login", Page::Login(LoginPage::Login));
                };
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
            Page::Login(login_page) => v1!{
                {
                    // let session = model.session.clone();
                    View::new_component("login", LoginSpec {
                        page: login_page.clone(),
                    })
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
