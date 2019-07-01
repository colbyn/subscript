pub mod data;
pub mod login;
pub mod signup;
pub mod account;
pub mod insight;
pub mod input;
pub mod ui_utils;

use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};
use subscript::prelude::*;

use data::*;
use login::LoginSpec;
use signup::SignupSpec;
use account::AccountSpec;
use insight::InsightSpec;
use input::InputSpec;
use ui_utils::{text_theme};


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
            ["input"] => {
                Page::Input
            }
            ["insight"] => {
                Page::Insight(InsightPage::Overview)
            }
            ["insight", "health"] => {
                Page::Insight(InsightPage::Health)
            }
            ["insight", "traffic"] => {
                Page::Insight(InsightPage::Traffic)
            }
            ["insight", "bandwidth"] => {
                Page::Insight(InsightPage::Bandwidth)
            }
            ["insight", "cache"] => {
                Page::Insight(InsightPage::Cache)
            }
            ["insight", "storage"] => {
                Page::Insight(InsightPage::Storage)
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
        // if &model.page.map(move |x| x.is_login()) => {
        //     background_color: "hsl(0, 0%, 86%) !important";
        // };
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
                        model.page.map(|x| x.is_input()),
                        "Inputs",
                        Page::Input,
                    );
                    nav_link(
                        model.page.map(|x| x.is_insight()),
                        "Insights",
                        Page::Insight(InsightPage::default()),
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
            (Some(session), Page::Input) => v1!{
                Component::singleton(InputSpec {
                    session: session.clone(),
                });
            },
            (Some(session), Page::Insight(insight_page)) => v1!{
                Component::singleton(InsightSpec {
                    page: insight_page.clone(),
                    session: session.clone(),
                });
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
    subscript::prelude::on_request_animation_frame();
}
