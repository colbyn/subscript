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

use crate::dev::server::data::*;

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
    pub account: Account,
    pub user_id: Uuid,
    pub user_name: String,
    pub encoded_token: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewSession(pub Session);

