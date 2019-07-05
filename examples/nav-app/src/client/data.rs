use std::collections::*;
use either::{Either, Either::*};
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use uuid::Uuid;
use chrono::prelude::*;
use subscript::prelude::{UrlString};

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


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Page {
    Homepage,
    Content,
    Input,
    Insight(InsightPage),
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum InsightPage {
    Overview,
    Health,
    Traffic,
    Bandwidth,
    Cache,
    Storage,
}

impl Page {
    pub fn is_homepage(&self) -> bool {self == &Page::Homepage}
    pub fn is_content(&self) -> bool {self == &Page::Content}
    pub fn is_input(&self) -> bool {self == &Page::Input}
    pub fn is_not_found(&self) -> bool {self == &Page::NotFound}
    pub fn is_insight(&self) -> bool {
        match self {
            Page::Insight(_) => true,
            _ => false
        }
    }
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
impl Default for InsightPage {
    fn default() -> Self {InsightPage::Overview}
}

impl UrlString for Page {
    fn url_string(&self) -> String {
        let str = match self {
            Page::Homepage =>
                "/",
            Page::Content =>
                "/content",
            Page::Input =>
                "/input",
            Page::Insight(InsightPage::Overview) =>
                "/insight",
            Page::Insight(InsightPage::Health) =>
                "/insight/health",
            Page::Insight(InsightPage::Traffic) =>
                "/insight/traffic",
            Page::Insight(InsightPage::Bandwidth) =>
                "/insight/bandwidth",
            Page::Insight(InsightPage::Cache) =>
                "/insight/cache",
            Page::Insight(InsightPage::Storage) =>
                "/insight/storage",
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

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// pub struct Session {
//     pub account: Account,
//     pub user_id: Uuid,
//     pub name: String,
//     pub encoded_token: String,
// }

// impl Session {
//     pub fn new(account: &Account) -> Self {
//         let account = account.clone();
//         let user_id = account.master.id.clone();
//         let name = account.master.name.clone();
//         let encoded_token = String::from("");
//         Session{account, user_id, name, encoded_token}
//     }
// }

