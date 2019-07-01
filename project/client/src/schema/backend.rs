use std::collections::*;
use either::{Either, Either::*};
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use uuid::Uuid;
use chrono::prelude::*;
use subscript::prelude::{UrlString};

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

///////////////////////////////////////////////////////////////////////////////
// USER
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub ts: Timestamp,
    pub name: UserName,
    pub password: EncodedPassword,
}

///////////////////////////////////////////////////////////////////////////////
// PASSWORD
///////////////////////////////////////////////////////////////////////////////

fn init_system_argon2id_config<'a>() -> argon2::Config<'a> {
    let config = argon2::Config::default();
    let config = argon2::Config {variant: argon2::Variant::Argon2id, ..config};
    config
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EncodedPassword {
    pub hash: String,
    pub salt: String,
    pub protocol: PasswordProtocol,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PasswordProtocol {
    Argon2id
}

impl EncodedPassword {
    pub fn new(password: &str) -> Self {
        // HELPERS
        fn new_random_salt() -> String {
            use rand::{self, Rng};
            use rand::distributions::Alphanumeric;
            rand::thread_rng().sample_iter(&Alphanumeric).take(16).collect()
        }
        // SETUP
        let protocol = PasswordProtocol::Argon2id;
        let config = init_system_argon2id_config();
        let salt = new_random_salt();
        let hash = argon2::hash_encoded(
            password.as_bytes(),
            salt.as_bytes(),
            &config
        );
        let hash = hash.expect("EncodedPassword.new failed");
        // RESULT
        EncodedPassword {hash, salt, protocol}
    }
    pub fn valid(&self, password: &str) -> bool {
        let config = init_system_argon2id_config();
        let result = argon2::verify_raw(
            password.as_bytes(),
            self.salt.as_bytes(),
            self.hash.as_bytes(),
            &config,
        );
        result.expect("EncodedPassword.valid failed")
    }
}

