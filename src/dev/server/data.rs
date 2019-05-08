use std::cell::{self, Cell, RefCell};
use std::str::FromStr;
use serde::de::{value, IntoDeserializer};
use serde::{self, Serialize, Deserialize};
use uuid::Uuid;
use chrono::prelude::*;



///////////////////////////////////////////////////////////////////////////////
// PRIMITIVES - MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn new() -> Self {
        use chrono::prelude::*;
        use crate::browser::console;
        
        let raw: String = From::from(js_sys::Date::new_0().to_iso_string());
        let ts: chrono::format::ParseResult<DateTime<Utc>> = DateTime::from_str(
            raw.as_str()
        );
        Timestamp(
            ts.expect("new timestamp failed")
        )
    }
}


///////////////////////////////////////////////////////////////////////////////
// DOMAIN LOGIC
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_id: Uuid,
    pub account_ts: Timestamp,
    pub account_name: String,
    pub account_master: User,
    pub account_users: Users,
}

impl Account {
    pub fn new(name: &str) -> Self {
        let ts = Timestamp::new();
        let id = Uuid::new_v4();
        
        Account {
            account_id: id.clone(),
            account_ts: ts.clone(),
            account_name: name.to_owned(),
            account_master: User {
                user_id: id,
                user_ts: ts,
                user_name: name.to_owned(),
                user_password: None
            },
            account_users: Users::new(),
        }
    }
}


pub type Users = std::collections::HashMap<UserName, User>;

pub type UserName = String;
pub type AccountName = String;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub user_ts: Timestamp,
    pub user_name: UserName,
    pub user_password: Option<EncodedPassword>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EncodedPassword {
    pub password_hash: String,
    pub password_salt: String,
    pub password_protocol: PasswordProtocol,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PasswordProtocol {
    ARGON2ID
}



