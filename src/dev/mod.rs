pub mod login;
pub mod data;
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
use crate::dev::login::*;
use crate::dev::data::*;


///////////////////////////////////////////////////////////////////////////////
// DOMAIN LOGIC MISCELLANEOUS - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Page {
    Homepage,
    Content,
    Analytics,
    Account,
    NotFound
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Session {
    account: Account,
    user_id: Uuid,
    user_name: String,
    encoded_token: String,
}


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppSpec {
    login: Process<LoginSpec>
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    NoOp,
    UrlChanged(Page),
    UrlRequest(Page),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AppModel {
    page: Option<Page>,
    session: Option<Session>
}


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl Spec for AppSpec {
    type Model = AppModel;
    type Msg = AppMsg;
    
    fn init(&self, loaded: InitArgs<Self::Model>) -> Init<Self::Model, Self::Msg> {
        use crate::effect::nav::UrlChange;
        use crate::effect::nav::router::*;
        
        let url_matcher: RouterFn<Self::Msg> = match_path!(
            [] => {
                AppMsg::UrlChanged(Page::Homepage)
            }
            ["content"] => {
                AppMsg::UrlChanged(Page::Content)
            }
            ["analytics"] => {
                AppMsg::UrlChanged(Page::Analytics)
            }
            ["account"] => {
                AppMsg::UrlChanged(Page::Account)
            }
            _ => {
                AppMsg::UrlChanged(Page::NotFound)
            }
        );
        
        Init {
            model: AppModel {
                page: None,
                session: None,
            },
            subs: subscriptions!(
                on(value: UrlChange) -> AppMsg {
                    url_matcher(value).unwrap_or(AppMsg::NoOp)
                }
            )
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd) {
        match msg {
            AppMsg::NoOp => (),
            AppMsg::UrlChanged(page) => {
                model.page = Some(page);
                cmd.save();
                cmd.update_view();
            }
            AppMsg::UrlRequest(page) => {
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
        let nav_link = move |name: &str, page: Page| -> Html<AppMsg> {markup!(li|
            width: "100%"
            text_align: "center"
            padding: "8px"
            self.css.append({
                if model.page == Some(page.clone()) {
                    css!()
                } else {
                    css!(font_weight: "300")
                }
            })
            .click(move |_| {
                AppMsg::UrlRequest(page.clone())
            })
            a(text(name))
        )};
        let navigation: Html<AppMsg> = markup!(nav.ul|
            margin: "0"
            padding: "0"
            list_style: "none"
            display: "flex"
            width: "100%"
            justify_content: "space-around"
            font_size: "0.9em"
            text_transform: "uppercase"
            font_family: "'Source Sans Pro', sans-serif"
            background_color: "#5d5d5d"
            color: "#fff"
            li(
                width: "300px"
                text_align: "center"
                padding: "8px"
                background_color: "#2d2d2d"
                .click(move |_| AppMsg::UrlRequest(Page::Homepage))
                color: "#fff"
                a(text("LOGO"))
            )
            self.append(&[
                nav_link("Content", Page::Content),
                nav_link("Analytics", Page::Analytics),
                nav_link("Account", Page::Account),
            ])
        );
        let homepage = markup!(
            {navigation}
            h1(text("Homepage"))
        );
        let content = markup!(
            {navigation}
            h1(text("Content"))
        );
        let analytics = markup!(
            {navigation}
            h1(text("Analytics"))
        );
        let account = markup!(
            {navigation}
            h1(text("Account"))
        );
        let not_found = markup!(
            {navigation}
            h1(text("Not Found"))
        );
        let loading = markup!(
            {navigation}
            h1(text("Loading"))
        );
        let active_session = || {
            match &model.page {
                Some(Page::Homepage) => homepage,
                Some(Page::Content) => content,
                Some(Page::Analytics) => analytics,
                Some(Page::Account) => account,
                Some(Page::NotFound) => not_found,
                None => loading,
            }
        };
        match &model.session {
            Some(session) => active_session(),
            None => HtmlBuild::new_component(Rc::new(self.login.clone()))
        }
    }
}

pub fn main() {
    use crate::effect::nav::Navigation;
    
    let app_spec = AppSpec {
        login: Process::from_spec(LoginSpec {
    
        })
    };
    AppBuilder::from_spec(app_spec)
        .with_effect(Navigation::new())
        .build()
        .start();
}


