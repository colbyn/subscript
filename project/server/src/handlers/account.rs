use uuid::Uuid;

use schema::forms::{AccountForm, UserForm};
use schema::backend::*;
use schema::common::*;
use super::common::*;


pub fn new(input: AccountForm) -> Result<Account, ApiError> {
    unimplemented!()
}

pub fn get(token: Token) -> Result<Account, ApiError> {
    unimplemented!()
}

pub fn is_taken(name: String) -> Result<bool, ApiError> {
    unimplemented!()
}

pub fn delete(token: Token) -> Result<(), ApiError> {
    unimplemented!()
}

pub mod user {
    use super::*;

    pub fn new(token: Token, input: UserForm) -> Result<(), ApiError> {
        unimplemented!()
    }

    pub fn delete(token: Token, user_id: Uuid) -> Result<(), ApiError> {
        unimplemented!()
    }

    pub mod token {
        use super::*;

        pub fn new(input: UserForm) -> Result<Token, ApiError> {
            unimplemented!()
        }
    }

    pub mod password {
        use super::*;

        pub fn set(token: Token, value: String) -> Result<(), ApiError> {
            unimplemented!()
        }
    }
}

