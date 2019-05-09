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

#[derive(Clone)]
pub struct AccountSpec {
    
}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
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
    
    fn init(&self, loaded: InitArgs<Self::Model>) -> Init<Self::Model, Self::Msg> {
        Init {
            model: match loaded.saved_model {
                Some(saved_model) => saved_model,
                None => Default::default(),
            },
            subs: subscriptions!()
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd) {
        match msg {
            Msg::NoOp => (),
        }
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        markup!(
            width: "100%"
            height: "100%"
            display: "grid"
            grid_template_columns: "300px 1fr"
            self.append(&[
                navigation(),
                body()
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
    text: &'static str,
}

pub fn navigation() -> Html<Msg> {
    let link = |link: Link| -> Html<Msg> {markup!(
        li(
            text(link.text)
        )
    )};
    let section = |info: NavSestion| -> Html<Msg> {markup!(nav|
        h3(
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
                        text: "Password"
                    },
                    Link {
                        text: "Email"
                    },
                ],
            }),
            section(NavSestion {
                title: "Organization settings",
                links: vec![
                    Link {
                        text: "Users"
                    },
                    Link {
                        text: "Billing"
                    },
                ],
            }),
        ])
    )
}

pub fn body() -> Html<Msg> {
    markup!(main|
        
    )
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


