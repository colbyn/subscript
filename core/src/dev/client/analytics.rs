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
use crate::dev::server::data::*;
use crate::dev::client::data::*;
use crate::dev::client::utils;

use crate::process::app::*;
use crate::process::basics::*;
use crate::process::online::*;



///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct AnalyticsSpec {
    
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

impl Spec for AnalyticsSpec {
    type Model = Model;
    type Msg = Msg;
    
    fn init(&self, loaded: InitArgs<Self::Model>, key: &InitKey) -> Init<Self::Model, Self::Msg> {
        Init {
            model: match loaded.saved_model {
                Some(saved_model) => saved_model,
                None => Default::default(),
            },
            subs: Default::default(),
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd) {
        match msg {
            Msg::NoOp => (),
        }
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        line_chart(model)
    }
}

///////////////////////////////////////////////////////////////////////////////
// APP VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn line_chart(model: &Model) -> Html<Msg> {
    let dataset: Vec<(u32, u32)> = vec![
        (0, 10),
        (10, 40),
        (20, 30),
        (30, 70),
        (40, 0),
    ];
    html!(svg|
        width: "100%"
        height: "auto"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 200 100"
        preserveAspectRatio="none"
        path(
            stroke_width="3"
            d = "M 0,0 L 100,00 L 50,100 z"
        )
    )
}

