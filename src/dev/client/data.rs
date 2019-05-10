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


///////////////////////////////////////////////////////////////////////////////
// NAVIGATION
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewPage(pub Page);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Page {
    Homepage,
    Content,
    Analytics,
    Account(AccountPage),
    NotFound,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountPage {
    Password,
    Email,
    Users,
    Billing,
}

impl Page {
    pub fn is_homepage(&self) -> bool {
        match self {
            Page::Homepage => true,
            _ => false
        }
    }
    pub fn is_content(&self) -> bool {
        match self {
            Page::Content => true,
            _ => false
        }
    }
    pub fn is_analytics(&self) -> bool {
        match self {
            Page::Analytics => true,
            _ => false
        }
    }
    pub fn is_account(&self) -> bool {
        match self {
            Page::Account(_) => true,
            _ => false
        }
    }
    pub fn is_not_found(&self) -> bool {
        match self {
            Page::NotFound => true,
            _ => false
        }
    }
}

impl AccountPage {
    pub fn is_password(&self) -> bool {
        match self {
            AccountPage::Password => true,
            _ => false
        }
    }
    pub fn is_email(&self) -> bool {
        match self {
            AccountPage::Email => true,
            _ => false
        }
    }
    pub fn is_users(&self) -> bool {
        match self {
            AccountPage::Users => true,
            _ => false
        }
    }
    pub fn is_billing(&self) -> bool {
        match self {
            AccountPage::Billing => true,
            _ => false
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::Homepage
    }
}

impl Default for AccountPage {
    fn default() -> Self {
        AccountPage::Password
    }
}

///////////////////////////////////////////////////////////////////////////////
// ACCOUNT
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Session {
    pub account: Account,
    pub user_id: Uuid,
    pub user_name: String,
    pub encoded_token: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewSession(pub Session);

