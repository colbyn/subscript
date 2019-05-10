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
use crate::process::data::*;
use crate::tree::offline::data::*;
use crate::tree::offline::api::*;
use crate::tree::online::data::*;
use crate::dev::server::data::*;
use crate::dev::client::data::*;



///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct AccountSpec {
    pub page: AccountPage,
}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
    Page(AccountPage)
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Model {
    
}

impl Default for Model {
    fn default() -> Self {
        Model {
            
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl Spec for AccountSpec {
    type Model = Model;
    type Msg = Msg;
    
    fn new() -> Self {
        AccountSpec {
            page: Default::default(),
        }
    }
    fn init(&self, loaded: InitArgs<Self::Model>, key: &InitKey) -> Init<Self::Model, Self::Msg> {
        Init {
            model: match loaded.saved_model {
                Some(saved_model) => Model {..saved_model},
                None => Model {..Default::default()},
            },
            subs: subscriptions!(),
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd) {
        match msg {
            Msg::NoOp => (),
            Msg::Page(account_page) => cmd.broadcast(
                NewPage(Page::Account(account_page))
            ),
        }
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        markup!(
            width: "100%"
            height: "100%"
            display: "grid"
            grid_template_columns: "300px 1fr"
            self.append(&[
                navigation(&self.page),
                body(&self.page)
            ])
        )
    }
}

///////////////////////////////////////////////////////////////////////////////
// APP VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct NavSestion {
    title: &'static str,
    links: Vec<Link>,
}

#[derive(Debug, Clone)]
pub struct Link {
    active: bool,
    text: &'static str,
    on_click: Msg,
}

pub fn navigation(page: &AccountPage) -> Html<Msg> {
    let link = move |link: Link| -> Html<Msg> {markup!(li|
        :hover (
            background_color: "#fbfbfb"
        )
        if (link.active)(
            font_weight: "600"
            box_shadow: "0px 0px 2px 0px #b9b9b9"
            position: "relative"
            z_index: "3"
        )
        padding: "5px"
        border_bottom: "1px solid #d4d4d4"
        text(&link.text)
        .click(move |event| {
            link.on_click.clone()
        })
    )};
    let section = |info: NavSestion| -> Html<Msg> {markup!(nav|
        box_shadow: "0px 1px 1px 0px #e2e2e2"
        display: "flex"
        flex_direction: "column"
        border: "1px solid #d4d4d4"
        margin: "12px"
        border_radius: "3px"
        color: "#3e3e3e"
        h3(
            border_top_left_radius: "3px"
            border_top_right_radius: "3px"
            padding: "3px"
            margin: "0"
            border_bottom: "1px solid #d4d4d4"
            background_color: "#f9f9f9"
            font_weight: "inherit"
            text(info.title)
        )
        ul(
            margin: "0"
            padding: "0"
            list_style: "none"
            self.append(
                info.links
                    .into_iter()
                    .map(|x| link(x))
                    .collect::<Vec<Html<Msg>>>()
            )
        )
    )};
    markup!(aside|
        font_size: "0.8em"
        font_family: "'Source Sans Pro', sans-serif"
        font_weight: "400"
        text_transform: "uppercase"
        self.append(&[
            section(NavSestion {
                title: "Personal Settings",
                links: vec![
                    Link {
                        active: page.is_password(),
                        text: "Password",
                        on_click: Msg::Page(AccountPage::Password),
                    },
                    Link {
                        active: page.is_email(),
                        text: "Email",
                        on_click: Msg::Page(AccountPage::Email),
                    },
                ],
            }),
            section(NavSestion {
                title: "Organization settings",
                links: vec![
                    Link {
                        active: page.is_users(),
                        text: "Users",
                        on_click: Msg::Page(AccountPage::Users),
                    },
                    Link {
                        active: page.is_billing(),
                        text: "Billing",
                        on_click: Msg::Page(AccountPage::Billing),
                    },
                ],
            }),
        ])
    )
}

pub fn body(page: &AccountPage) -> Html<Msg> {
    match page {
        AccountPage::Password => markup!(main|
            
        ),
        AccountPage::Email => markup!(main|
            
        ),
        AccountPage::Users => markup!(main|
            
        ),
        AccountPage::Billing => markup!(main|
            
        ),
    }
}

#[derive(Debug, Clone)]
pub enum ChildPos {
    First,
    Middle,
    Last,
}

impl ChildPos {
    pub fn is_first(&self) -> bool {
        match &self {
            ChildPos::First => true,
            _ => false
        }
    }
    pub fn is_middle(&self) -> bool {
        match &self {
            ChildPos::Middle => true,
            _ => false
        }
    }
    pub fn is_last(&self) -> bool {
        match &self {
            ChildPos::Last => true,
            _ => false
        }
    }
}


