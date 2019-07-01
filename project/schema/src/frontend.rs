use std::collections::*;
use either::{Either, Either::*};
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use uuid::Uuid;
use chrono::prelude::*;

pub use super::common::*;


///////////////////////////////////////////////////////////////////////////////
// ACCOUNT
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub ts: Timestamp,
    pub name: String,
    pub master: User,
    pub users: HashMap<UserName, User>,
    pub sources: HashMap<InputName, Source>
}

impl Account {
    pub fn new(account_name: &str) -> Self {
        let id = Uuid::new_v4();
        let ts = Timestamp::new();
        let name = String::from(account_name);
        let master = User {
            id: Uuid::new_v4(),
            ts: ts.clone(),
            name: name.clone(),
        };
        let users = HashMap::new();
        let sources = HashMap::new();
        Account{id, ts, name, master, users, sources}
    }
}

///////////////////////////////////////////////////////////////////////////////
// USERS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub ts: Timestamp,
    pub name: UserName,
}

