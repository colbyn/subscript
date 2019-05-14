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
use serde::ser::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::convert::*;


///////////////////////////////////////////////////////////////////////////////
// PASSWORD-STRING NEWTYPE
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PasswordString(pub String);

impl Serialize for PasswordString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("")
    }
}


