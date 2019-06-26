pub mod data;
pub mod login;
pub mod signup;
pub mod account;
pub mod ui_utils;

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
use crate::dev::cms_app::client::signup::SignupSpec;
use crate::dev::cms_app::client::account::AccountSpec;
use crate::dev::cms_app::client::ui_utils::{text_theme};


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
    Logout,
}

#[derive(Default)]
pub struct Model {
    page: Signal<Page>,
    session: Signal<Option<Session>>,
}

///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub static CACHE_SESSION_KEY: &'static str = "cmd.session";


///////////////////////////////////////////////////////////////////////////////
// VIEWABLE DATA TYPES
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Spec for AppSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, sh: &Shell<Self>) -> Init<Self> {
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
            ["account", "billing"] => {
                Page::Account(AccountPage::Billing)
            }
            ["account", "password"] => {
                Page::Account(AccountPage::Password)
            }
            ["account", "users"] => {
                Page::Account(AccountPage::Users(UsersPage::Index))
            }
            ["account", "users", "add-user"] => {
                Page::Account(AccountPage::Users(UsersPage::AddUser))
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
        let session: Option<Session> = sh
            .cache()
            .get(CACHE_SESSION_KEY);
        let session: Signal<Option<Session>> = Signal::new(session);
        let mut page = Signal::new(url_parser.parse(&sh.current_url()));
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
            msg new_session(value: NewSession) -> Msg {
                Msg::NewSession(value.0)
            }
        };
        Init{subs, model, ..Default::default()}
    }
    fn update(&self, model: &mut Model, msg: Msg, sh: &mut Shell<Self>) {
        // HELPERS
        fn set_url(model: &mut Model, sh: &mut Shell<AppSpec>, page: Page) {
            sh.navigate(page.clone());
            model.page.set(page);
        }
        // GO!
        match msg {
            Msg::NoOp => {}
            Msg::UrlChanged(page) => {
                model.page.set(page);
            }
            Msg::UrlRequest(page) => {
                sh.navigate(page);
            }
            Msg::NewSession(session) => {
                sh  .cache()
                    .insert(CACHE_SESSION_KEY, &session);
                model.session.set(Some(session));
                if model.page.get().is_login() {
                    set_url(model, sh, Page::Homepage)
                }
            }
            Msg::Logout => {
                model.session.set(None);
                sh  .cache()
                    .remove(CACHE_SESSION_KEY);
                set_url(model, sh, Page::Login(LoginPage::default()))
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
    let nav_link = |active: Formula<bool>, txt: &str, page: Page| -> View<Msg> {v1!{
        li !{
            display: "block";
            margin_right: "16px";
            padding_top: "3px";
            border_bottom: "3px solid transparent";
            if &active => {
                border_bottom: "3px solid #0089ff !important";
            };
            a !{
                user_select: "none";
                display: "block";
                color: "#fff";
                font_weight: "500";
                padding: "8px";
                if &active.map(|x| !x) => {
                    css.hover => s1!{
                        color: "#777";
                    };
                };
                event.click[page] => move || {
                    Msg::UrlRequest(page)
                };
                txt;
            };
        };
    }};
    let logout_link = || v1!{
        li !{
            display: "block";
            padding_top: "3px";
            padding_left: "12px";
            padding_right: "12px";
            border_bottom: "3px solid transparent";
            background_color: "#272727";
            
            event.click[page] => move || {
                Msg::Logout
            };
            a !{
                user_select: "none";
                display: "block";
                color: "#fff";
                font_weight: "500";
                padding: "8px";
                css.hover => s1!{
                    color: "#777";
                };
                "Logout";
            };
        };
    };
    v1!{
        header !{
            background_color: "hsl(0, 0%, 24%)";
            display: "flex";
            justify_content: "space-between";
            align_items: "center";
            span !{
                user_select: "none";
                margin_left: "16px";
                color: "#fff";
                "LOGO.IO";
            };
            ul !{
                list_style: "none";
                display: "flex";
                padding: "0";
                margin: "0";
                if &model.session.map(|x| x.is_some()) => {
                    nav_link(
                        model.page.map(|x| x.is_content()),
                        "Content",
                        Page::Content,
                    );
                    nav_link(
                        model.page.map(|x| x.is_analytics()),
                        "Analytics",
                        Page::Analytics,
                    );
                    nav_link(
                        model.page.map(|x| x.is_account()),
                        "Account",
                        Page::Account(AccountPage::default()),
                    );
                    logout_link();
                };
                if &model.session.map(|x| x.is_none()) => {
                    nav_link(
                        model.page.map(|x| x == &Page::Login(LoginPage::Signup)),
                        "Signup",
                        Page::Login(LoginPage::Signup),
                    );
                    nav_link(
                        model.page.map(|x| x == &Page::Login(LoginPage::Login)),
                        "Login",
                        Page::Login(LoginPage::Login),
                    );
                };
            };
        };
    }
}

pub fn page(model: &Model) -> View<Msg> {v1!{
    bind &model.page.zip(&model.session) => move |(page, session)| {
        match (session, page) {
            (Some(session), Page::Homepage) => v1!{
                h1 !{
                    "Homepage";
                };
            },
            (Some(session), Page::Content) => v1!{
                h1 !{
                    "Content";
                };
            },
            (Some(session), Page::Analytics) => v1!{
                h1 !{
                    "Analytics";
                };
            },
            (Some(session), Page::Account(accunt_page)) => v1!{
                Component {
                    name: String::from("account"),
                    spec: AccountSpec {
                        page: accunt_page.clone(),
                        session: session.clone(),
                    }
                };
            },
            (_, Page::NotFound) => v1!{
                h1 !{
                    "NotFound";
                };
            },
            (_, Page::Login(LoginPage::Signup)) => v1!{
                Component {
                    name: String::from("signup"),
                    spec: SignupSpec {}
                };
            },
            (_, _) => v1!{
                Component {
                    name: String::from("login"),
                    spec: LoginSpec {}
                };
            },
        }
    };
}}



///////////////////////////////////////////////////////////////////////////////
// VIEW AGNOSTIC UTILS
///////////////////////////////////////////////////////////////////////////////




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
