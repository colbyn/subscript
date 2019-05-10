pub mod data;
pub mod login;
pub mod account;
pub mod analytics;
pub mod utils;

use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::any::*;
use std::marker::Sized;
use either::Either;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use uuid::Uuid;

use crate::browser::*;
use crate::effect::url::{self, Url};
use crate::tree::offline::data::*;
use crate::tree::offline::api::*;
use crate::tree::online::data::*;
use crate::dev::client::login::LoginSpec;
use crate::dev::client::account::AccountSpec;
use crate::dev::client::analytics::AnalyticsSpec;
use crate::dev::server::data::*;
use crate::dev::client::data::*;

use crate::process::app::*;
use crate::process::basics::*;
use crate::process::online::*;



///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppSpec {
    account_view: Reactive<AccountPage>,
    url: Reactive<Url>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
    UrlChanged(Page),
    UrlRequest(Page),
    NewSession(Session),
    Logout
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Model {
    page: Page,
    session: Option<Session>
}

impl Default for Model {
    fn default() -> Self {
        Model {
            page: Page::Homepage,
            session: None,
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl Spec for AppSpec {
    type Model = Model;
    type Msg = Msg;
    
    fn init(&self, loaded: InitArgs<Self::Model>, key: &InitKey) -> Init<Self::Model, Self::Msg> {
        use crate::effect::url::*;
        
        let url_parser: UrlParser<Page> = match_path!(
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
                Page::Account(Default::default())
            }
            ["account", "password"] => {
                Page::Account(AccountPage::Password)
            }
            ["account", "email"] => {
                Page::Account(AccountPage::Email)
            }
            ["account", "users"] => {
                Page::Account(AccountPage::Users)
            }
            ["account", "billing"] => {
                Page::Account(AccountPage::Billing)
            }
            _ => {
                Page::NotFound
            }
        );
        
        let initial_page = {
            let url = self.url.unlock(key);
            let page = url_parser(url).unwrap_or(Page::NotFound);
            page
        };
        
        Init {
            model: match loaded.saved_model {
                Some(saved_model) => Model {page: initial_page, ..saved_model},
                None => Model {page: initial_page, ..Default::default()},
            },
            subs: subscriptions!(
                bind(self.url -> value) -> Msg {
                    let new_page = url_parser(value).unwrap_or(Page::NotFound);
                    Msg::UrlChanged(new_page)
                }
                on(msg: NewPage) -> Msg {
                    Msg::UrlRequest(msg.0)
                }
                on(msg: NewSession) -> Msg {
                    Msg::NewSession(msg.0)
                }
            )
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd) {
        match msg {
            Msg::NoOp => (),
            Msg::NewSession(new_session) => {
                model.session = Some(new_session);
                cmd.save();
                cmd.update_view();
            }
            Msg::Logout => {
                model.session = None;
                cmd.save();
                cmd.update_view();
            }
            Msg::UrlChanged(page) => {
                model.page = page;
                cmd.save();
                cmd.update_view();
            }
            Msg::UrlRequest(page) => {
                match &page {
                    Page::Homepage => cmd.navigate("/"),
                    Page::Content => cmd.navigate("/content"),
                    Page::Analytics => cmd.navigate("/analytics"),
                    Page::Account(_) => cmd.navigate("/account"),
                    Page::NotFound => cmd.navigate("/not-found"),
                }
                model.page = page;
                cmd.save();
                cmd.update_view();
            }
        }
        cmd.update_view();
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        let nav_link = move |text: &str, active: bool, on_click: Msg| -> Html<Msg> {markup!(li|
            width: "100%"
            text_align: "center"
            padding: "8px"
            user_select: "none"
            if(!active)(
                font_weight: "300"
            )
            .click(move |_| {
                on_click.clone()
            })
            a(text(text))
        )};
        let navigation: Html<Msg> = markup!(nav.ul|
            z_index: "2"
            position: "relative"
            margin: "0"
            padding: "0"
            list_style: "none"
            display: "flex"
            width: "100%"
            justify_content: "space-around"
            font_size: "0.9em"
            font_family: "'Source Sans Pro', sans-serif"
            text_transform: "uppercase"
            color: "#fff"
            background_color: "#1b1b1b"
            li(
                width: "100%"
                text_align: "center"
                padding: "8px"
                color: "#fff"
                user_select: "none"
                font_weight: "300"
                .click(move |_| Msg::UrlRequest(Page::Homepage))
                a(text("LOGO.IO"))
            )
            self.append(&[
                nav_link(
                    "Content",
                    model.page.is_content(),
                    Msg::UrlRequest(Page::Content)
                ),
                nav_link(
                    "Analytics",
                    model.page.is_analytics(),
                    Msg::UrlRequest(Page::Analytics)
                ),
                nav_link(
                    "Account",
                    model.page.is_account(),
                    Msg::UrlRequest(Page::Account(Default::default()))
                ),
            ])
            li(
                width: "300px"
                text_align: "center"
                padding: "8px"
                font_weight: "300"
                color: "#eaeaea"
                border_left: "1px solid #3c3c3c"
                user_select: "none"
                .click(move |_| Msg::Logout)
                a(text("Logout"))
            )
        );
        let root_page = move |content: Html<Self::Msg>| {
            markup!(
                {navigation}
                {content}
            )
        };
        let homepage = root_page(markup!(
            h1(text("Homepage"))
        ));
        let content = root_page(markup!(
            h1(text("Content"))
        ));
        let analytics = root_page(
            HtmlBuild::new_component(AnalyticsSpec {})
        );
        let account = |subpage| {
            root_page(
                HtmlBuild::new_component(AccountSpec {
                    page: self.account_view.set(subpage)
                })
            )
        };
        let not_found = root_page(markup!(
            h1(text("Not Found"))
        ));
        
        markup!(
            height: "100%"
            {
                if model.session.is_none() {
                    HtmlBuild::new_component(LoginSpec {})
                } else {
                    match &model.page {
                        Page::Homepage => homepage,
                        Page::Content => content,
                        Page::Analytics => analytics,
                        Page::Account(subpage) => account(subpage.clone()),
                        Page::NotFound => not_found,
                    }
                }
            }
        )
    }
}

pub fn main() {
    let app_spec = AppSpec {
        account_view: Reactive::from_value(Default::default()),
        url: url::mk_reactive(),
    };
    let app = Application::from_spec(app_spec);
    app.start();
}


