use std::collections::*;
use either::{Either, Either::*};
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use uuid::Uuid;
use chrono::prelude::*;
use subscript::prelude::{UrlString};

pub use super::common::*;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewUserForm {
    name: String,
    password: String,
}
