use std::collections::*;
use either::{Either, Either::*};
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use uuid::Uuid;
use chrono::prelude::*;

pub use super::common::*;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewUserForm {
    name: String,
    password: String,
}
