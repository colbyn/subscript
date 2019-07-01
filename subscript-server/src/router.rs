#[macro_export]
macro_rules! operations {
    ($($x:tt)*) => {};
}

operations!{
    account::new
    account::get
    account::is_taken
    account::user::token::new
    account::user::new
    account.user::delete
    account.user.password::set
    account::delete
}

operations!{
    account {
        fn new(input: AccountForm) -> Result<Account, ApiError>;
        fn get(token: Token) -> Result<Account, ApiError>;
        fn is_taken(name: String) -> Result<bool, ApiError>;
        fn delete(token: Token) -> Result<(), ApiError>;
        user {
            fn new(token: Token, input: UserForm) -> Result<(), ApiError>;
            fn delete(token: Token) -> Result<(), ApiError>;
            token {
                fn new(input: UserForm) -> Result<Token, ApiError>;
            }
            password {
                fn set(token: Token, input: String) -> Result<(), ApiError>;
            }
        }
    }
}

fn handlers() {
    operations!{
        mod account {
            fn new(input: AccountForm) -> Result<Account, ApiError> {
                unimplemented!()
            }
            fn get(token: Token) -> Result<Account, ApiError> {
                unimplemented!()
            }
            fn delete(token: Token) -> Result<(), ApiError> {
                unimplemented!()
            }
            fn is_taken(name: String) -> Result<bool, ApiError> {
                unimplemented!()
            }
            mod user {
                fn new(token: Token, input: UserForm) -> Result<(), ApiError> {
                    unimplemented!()
                }
                fn delete(token: Token, id: UserId) -> Result<(), ApiError> {
                    unimplemented!()
                }
                mod token {
                    fn new(input: UserForm) -> Result<Token, ApiError> {
                        unimplemented!()
                    }
                    fn delete(id: TokenId) -> Result<Token, ApiError> {
                        unimplemented!()
                    }
                }
                mod password {
                    fn set(token: Token, input: String) -> Result<(), ApiError> {
                        unimplemented!()
                    }
                }
            }
        }
    }
    operations!{
        post ["account"](body: AccountForm) => {

        };
        get ["account"](body: Auth<()>) => {

        };
        post ["account", "is-taken"](body: String) => {

        };
        post ["account", "user", "token"](body: AccountForm) => {

        };
        post ["account", "user"](body: Auth<UserForm>) => {

        };
        delete ["account", "user"](body: Auth<UserId>) => {

        };
        put ["account", "user", "password"](body: Auth<String>) => {

        };
        delete ["account"] => {

        };
    };
}

