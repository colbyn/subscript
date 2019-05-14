pub mod mixins;

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
use crate::process::app::*;
use crate::process::basics::*;
use crate::process::online::*;


pub fn button<Msg: Clone + Debug>(content: Mixin<Msg>) -> Html<Msg> {
    html!(button|
        color: "#212121"
        border: "1px solid #a2a2a2"
        background: "linear-gradient(to bottom, rgb(249, 249, 249) 0%,rgb(214, 214, 214) 100%)"
        border_radius: "3px"
        outline: "none"
        mixin(content)
    )
}

