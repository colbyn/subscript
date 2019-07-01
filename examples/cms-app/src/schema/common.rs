use std::collections::*;
use either::{Either, Either::*};
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use uuid::Uuid;
use chrono::prelude::*;
use subscript::prelude::{UrlString};


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type UserName = String;
pub type AccountName = String;
pub type InputName = String;


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
// SOURCE
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Uuid,
    pub ts: Timestamp,
    pub name: String,
    pub driver: SourceDriver,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SourceDriver {
    Http {
        address: String,
    },
    AwsS3 {

    },
    GoogleStorage {

    },
}


impl SourceDriver {
    pub fn is_http(&self) -> bool {
        match self {
            SourceDriver::Http{..} => true,
            _ => false,
        }
    }
    pub fn is_aws_s3(&self) -> bool {
        match self {
            SourceDriver::AwsS3{..} => true,
            _ => false,
        }
    }
    pub fn is_google_storage(&self) -> bool {
        match self {
            SourceDriver::GoogleStorage{..} => true,
            _ => false,
        }
    }
}