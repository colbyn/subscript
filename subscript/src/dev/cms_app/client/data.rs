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
// CACHE
///////////////////////////////////////////////////////////////////////////////

pub static CACHE_LOGIN_NAME_KEY: &'static str = "cms.login.name";


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

#[derive(Clone)]
pub struct UrlRequest(pub Page);


///////////////////////////////////////////////////////////////////////////////
// PAGE
///////////////////////////////////////////////////////////////////////////////

use crate::program_sys::spec::UrlString;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Page {
    Homepage,
    Content,
    Analytics,
    Account(AccountPage),
    Login(LoginPage),
    NotFound,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LoginPage {
    Login,
    Signup,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountPage {
    Password,
    Users(UsersPage),
    Billing,
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
        match self {
            Page::Login(_) => true,
            _ => false
        }
    }
    pub fn is_account(&self) -> bool {
        self.get_account().is_some()
    }
    pub fn get_account(&self) -> Option<AccountPage> {
        match self {
            Page::Account(x) => Some(x.clone()),
            _ => None
        }
    }
}
impl AccountPage {
    pub fn is_password(&self) -> bool {self == &AccountPage::Password}
    pub fn is_billing(&self) -> bool {self == &AccountPage::Billing}
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
impl LoginPage {
    pub fn login(&self) -> bool {
        self == &LoginPage::Login
    }
    pub fn signup(&self) -> bool {
        self == &LoginPage::Signup
    }
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
impl Default for LoginPage {
    fn default() -> Self {LoginPage::Signup}
}

impl UrlString for Page {
    fn url_string(&self) -> String {
        let str = match self {
            Page::Homepage =>
                "/",
            Page::Content =>
                "/content",
            Page::Analytics =>
                "/analytics",
            Page::Account(AccountPage::Password) =>
                "/account/password",
            Page::Account(AccountPage::Users(UsersPage::Index)) =>
                "/account/users",
            Page::Account(AccountPage::Users(UsersPage::AddUser)) =>
                "/account/users/add-user",
            Page::Account(AccountPage::Billing) =>
                "/account/billing",
            Page::Login(LoginPage::Login) =>
                "/login",
            Page::Login(LoginPage::Signup) =>
                 "/signup",
            Page::NotFound =>
                "not-found",
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

impl Session {
    pub fn new(account: &Account) -> Self {
        let account = account.clone();
        let user_id = account.account_master.user_id.clone();
        let user_name = account.account_master.user_name.clone();
        let encoded_token = String::from("");
        Session{account, user_id, user_name, encoded_token}
    }
}

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

impl Account {
    pub fn new(account_name: &str) -> Self {
        let account_id = Uuid::new_v4();
        let account_ts = Timestamp::new();
        let account_name = String::from(account_name);
        let account_master = User {
            user_id: Uuid::new_v4(),
            user_ts: account_ts.clone(),
            user_name: account_name.clone(),
        };
        let account_users = HashMap::new();
        Account{account_id, account_ts, account_name, account_master, account_users}
    }
}