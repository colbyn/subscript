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
use crate::tree::offline::data::*;
use crate::tree::offline::api::*;
use crate::tree::online::data::*;
use crate::process::data::*;
use crate::dev::client::login::LoginSpec;
use crate::dev::client::account::AccountSpec;
use crate::dev::client::analytics::AnalyticsSpec;
use crate::dev::server::data::*;
use crate::dev::client::data::*;



///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppSpec {
    login: Process<LoginSpec>,
    account: Process<AccountSpec>,
    analytics: Process<AnalyticsSpec>,
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
    page: Option<Page>,
    session: Option<Session>
}

impl Default for Model {
    fn default() -> Self {
        Model {
            page: None,
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
    
    fn init(&self, loaded: InitArgs<Self::Model>) -> Init<Self::Model, Self::Msg> {
        use crate::effect::nav::UrlChange;
        use crate::effect::nav::router::*;
        
        let url_matcher: RouterFn<Self::Msg> = match_path!(
            [] => {
                Msg::UrlChanged(Page::Homepage)
            }
            ["content"] => {
                Msg::UrlChanged(Page::Content)
            }
            ["analytics"] => {
                Msg::UrlChanged(Page::Analytics)
            }
            ["account"] => {
                Msg::UrlChanged(Page::Account)
            }
            _ => {
                Msg::UrlChanged(Page::NotFound)
            }
        );
        
        Init {
            model: match loaded.saved_model {
                Some(saved_model) => saved_model,
                None => Default::default(),
            },
            subs: subscriptions!(
                on(value: NewSession) -> Msg {
                    Msg::NewSession(value.0)
                }
                on(value: UrlChange) -> Msg {
                    url_matcher(value).unwrap_or(Msg::NoOp)
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
                model.page = Some(page);
                cmd.save();
                cmd.update_view();
            }
            Msg::UrlRequest(page) => {
                match &page {
                    Page::Homepage => cmd.navigate("/"),
                    Page::Content => cmd.navigate("/content"),
                    Page::Analytics => cmd.navigate("/analytics"),
                    Page::Account => cmd.navigate("/account"),
                    Page::NotFound => cmd.navigate("/not-found"),
                }
                model.page = Some(page);
                cmd.save();
                cmd.update_view();
            }
        }
        cmd.update_view();
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        let nav_link = move |name: &str, page: Page| -> Html<Msg> {markup!(li|
            width: "100%"
            text_align: "center"
            padding: "8px"
            user_select: "none"
            self.css.append({
                if model.page == Some(page.clone()) {
                    css!()
                } else {
                    css!(font_weight: "300")
                }
            })
            .click(move |_| {
                Msg::UrlRequest(page.clone())
            })
            a(text(name))
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
            text_transform: "uppercase"
            font_family: "'Source Sans Pro', sans-serif"
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
                nav_link("Content", Page::Content),
                nav_link("Analytics", Page::Analytics),
                nav_link("Account", Page::Account),
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
                // display: "flex"
                // flex_direction: "column"
                // min_width: "100%"
                // min_height: "100%"
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
            HtmlBuild::new_component(Box::new(self.analytics.clone()))
        );
        let account = root_page(
            HtmlBuild::new_component(Box::new(self.account.clone()))
        );
        let not_found = root_page(markup!(
            h1(text("Not Found"))
        ));
        let loading = root_page(markup!(
            h1(text("Loading"))
        ));
        
        markup!(
            // display: "flex"
            // flex_direction: "column"
            // min_width: "100%"
            // min_height: "100%"
            {
                if model.session.is_none() {
                    HtmlBuild::new_component(
                        Box::new(self.login.clone())
                    )
                } else {
                    match &model.page {
                        Some(Page::Homepage) => homepage,
                        Some(Page::Content) => content,
                        Some(Page::Analytics) => analytics,
                        Some(Page::Account) => account,
                        Some(Page::NotFound) => not_found,
                        None => loading,
                    }
                }
            }
        )
    }
}

pub fn main() {
    use crate::effect::nav::Navigation;
    
    let app_spec = AppSpec {
        login: Process::from_spec("login", LoginSpec {}),
        account: Process::from_spec("account", AccountSpec {}),
        analytics: Process::from_spec("analytics", AnalyticsSpec {}),
    };
    AppBuilder::from_spec(app_spec)
        .with_effect(Navigation::new())
        .build()
        .start();
}


