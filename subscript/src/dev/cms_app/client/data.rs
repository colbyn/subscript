//! Client side specific data types. 
//!
//! Some things like an Account type may have two seemingly redundant definitions,
//! this may be due to the need of excluding certain properties or fields from
//! being publicly exposed in any way. Since apart from perhaps security, any form of
//! publicly exposed data creates an implicit interface where the slightest of changes
//! may brake fragile parsers from integrating end-users.
use std::collections::*;
use either::{Either, Either::*};
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use uuid::Uuid;
use chrono::prelude::*;


///////////////////////////////////////////////////////////////////////////////
// TIME
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn new() -> Self {
        use std::str::FromStr;
        let raw: String = From::from(js_sys::Date::new_0().to_iso_string());
        let ts: chrono::format::ParseResult<DateTime<Utc>> = DateTime::from_str(raw.as_str());
        let ts = ts.expect("new timestamp failed");
        Timestamp(ts)
    }
}


///////////////////////////////////////////////////////////////////////////////
// NAVIGATION
///////////////////////////////////////////////////////////////////////////////
use crate::program_sys::spec::UrlString;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Page {
    Homepage,
    Content,
    Analytics,
    Account(AccountPage),
    Login,
    Signup,
    NotFound,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountPage {
    Password,
    Email,
    Users(UsersPage),
    Billing,
    AccountMaster,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum UsersPage {
    Index,
    AddUser
}

impl Page {
    pub fn is_homepage(&self) -> bool {self == &Page::Homepage}
    pub fn is_content(&self) -> bool {self == &Page::Content}
    pub fn is_analytics(&self) -> bool {self == &Page::Analytics}
    pub fn is_not_found(&self) -> bool {self == &Page::NotFound}
    pub fn is_login(&self) -> bool {
        self == &Page::Login
    }
    pub fn is_signup(&self) -> bool {
        self == &Page::Signup
    }
    pub fn is_account(&self) -> Option<AccountPage> {
        match self {
            Page::Account(x) => Some(x.clone()),
            _ => None
        }
    }
}
impl AccountPage {
    pub fn is_password(&self) -> bool {self == &AccountPage::Password}
    pub fn is_email(&self) -> bool {self == &AccountPage::Email}
    pub fn is_billing(&self) -> bool {self == &AccountPage::Billing}
    pub fn is_account_master(&self) -> bool {self == &AccountPage::AccountMaster}
    pub fn is_users(&self) -> bool {
        match self {
            AccountPage::Users(_) => true,
            _ => false
        }
    }
}
impl UsersPage {
    pub fn is_index(&self) -> bool {self == &UsersPage::Index}
    pub fn is_add_user(&self) -> bool {self == &UsersPage::AddUser}
}
impl Default for Page {
    fn default() -> Self {Page::Homepage}
}
impl Default for AccountPage {
    fn default() -> Self {AccountPage::Password}
}
impl Default for UsersPage {
    fn default() -> Self {UsersPage::Index}
}

impl UrlString for Page {
    fn url_string(&self) -> String {
        let str = match self {
            Page::Homepage => "/",
            Page::Content => "/content",
            Page::Analytics => "/analytics",
            Page::Account(account_page) => "/account",
            Page::Login => "/login",
            Page::Signup => "/signup",
            Page::NotFound => "not-found",
        };
        String::from(str)
    }
}

///////////////////////////////////////////////////////////////////////////////
// SESSION
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


///////////////////////////////////////////////////////////////////////////////
// ACCOUNT
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_id: Uuid,
    pub account_ts: Timestamp,
    pub account_name: String,
    pub account_master: User,
    pub account_users: Users,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub user_ts: Timestamp,
    pub user_name: UserName,
}

pub type Users = std::collections::HashMap<UserName, User>;
pub type UserName = String;
pub type AccountName = String;
