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
            {pane(ChildPos::First, Some("Nav"))}
            {pane(ChildPos::Last, Some("Username"))}
        )
    }
}

pub fn pane(pos: ChildPos, header: Option<&str>) -> Html<Msg> {
    markup!(
        display: "flex"
        flex_direction: "column"
        width: "100%"
        height: "100%"
        header(
            width: "100%"
            height: "100px"
            display: "flex"
            justify_content: "center"
            align_items: "center"
            font_family: "'Source Sans Pro', sans-serif"
            text_transform: "uppercase"
            font_size: "0.9em"
            border_bottom: "1px solid #000"
            {
                if let Some(txt) = header {
                    markup!(text(txt))
                } else {
                    markup!()
                }
            }
        )
        div(
            width: "100%"
            height: "100%"
        )
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
            ChildPos::First => true,
            _ => false
        }
    }
    pub fn is_last(&self) -> bool {
        match &self {
            ChildPos::First => true,
            _ => false
        }
    }
}


