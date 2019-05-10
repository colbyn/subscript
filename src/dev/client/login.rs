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

use crate::browser::*;
use crate::tree::offline::data::*;
use crate::tree::offline::api::*;
use crate::tree::online::data::*;
use crate::dev::client::utils;
use crate::dev::client::data::*;
use crate::dev::server::data::*;
use crate::extras::*;

use crate::process::app::*;
use crate::process::basics::*;
use crate::process::online::*;


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct LoginSpec {}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
    LoginNameInput(String),
    LoginPasswordInput(String),
    NewNameInput(String),
    NewPasswordInput(String),
    NewPasswordConfirmInput(String),
    SubmitLogIn,
    SubmitCreateAccount,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Model {
    server_login_errors: Vec<String>,
    server_create_account_errors: Vec<String>,
    login_name: String,
    login_password: PasswordString,
    new_name: String,
    new_password: PasswordString,
    new_password_confirm: PasswordString,
    new_name_errors: Vec<String>,
    new_password_errors: Vec<String>,
    new_password_confirm_errors: Vec<String>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            server_login_errors: Vec::new(),
            server_create_account_errors: Vec::new(),
            login_name: String::new(),
            login_password: PasswordString(String::new()),
            new_name: String::new(),
            new_password: PasswordString(String::new()),
            new_password_confirm: PasswordString(String::new()),
            new_name_errors: Vec::new(),
            new_password_errors: Vec::new(),
            new_password_confirm_errors: Vec::new(),
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl Spec for LoginSpec {
    type Model = Model;
    type Msg = Msg;
    
    fn init(&self, loaded: InitArgs<Self::Model>, key: &InitKey) -> Init<Self::Model, Self::Msg> {
        Init {
            model: Default::default(),
            subs: Default::default(),
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd) {
        match msg {
            Msg::LoginNameInput(text) => {
                model.login_name = text;
                cmd.update_view();
            },
            Msg::LoginPasswordInput(text) => {
                model.login_password = PasswordString(text);
                cmd.update_view();
            },
            Msg::NewNameInput(text) => {
                model.new_name_errors = run_check_for(
                    new_name_checks(),
                    text.clone(),
                );
                model.new_name = text;
                cmd.update_view();
            },
            Msg::NewPasswordInput(text) => {
                model.new_password_errors = run_check_for(
                    new_password_checks(),
                    text.clone(),
                );
                model.new_password = PasswordString(text);
                cmd.update_view();
            },
            Msg::NewPasswordConfirmInput(text) => {
                model.new_password_confirm_errors = run_check_for(
                    new_password_confirm_checks(
                        model.new_password.0.clone()
                    ),
                    text.clone(),
                );
                model.new_password_confirm = PasswordString(text);
                cmd.update_view();
            },
            Msg::SubmitLogIn => {
                
            },
            Msg::SubmitCreateAccount => {
                let not_empty = {
                    (!model.new_name.is_empty()) &&
                    (!model.new_password.0.is_empty()) &&
                    (!model.new_password_confirm.0.is_empty())
                };
                let no_errors = {
                    model.new_name_errors.is_empty() &&
                    model.new_password_errors.is_empty() &&
                    model.new_password_confirm_errors.is_empty()
                };
                if not_empty && no_errors {
                    let account = Account::new(model.new_name.as_str());
                    let session = Session {
                        user_id: account.account_master.user_id.clone(),
                        user_name: account.account_master.user_name.clone(),
                        encoded_token: String::from(""),
                        account: account,
                    };
                    *model = Default::default();
                    cmd.update_view();
                    cmd.broadcast(NewSession(session));
                }
            },
            Msg::NoOp => (),
        }
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        let panel = |title: &str, form: Html<Msg>| -> Html<Msg> {markup!(
            width: "100%"
            background_color: "#fff"
            max_width: "400px"
            padding: "12px"
            @media [min_width: "900px"] (
                margin: "0 auto"
                margin_top: "60px"
            )
            @media [max_width: "900px"] (
                margin: "0 auto"
                margin_top: "0"
            )
            h1(
                color: "#5a5a5a"
                font_family: "'Source Sans Pro', sans-serif"
                text_align: "center"
                margin: "0"
                padding_bottom: "20px"
                text(title)
            )
            self.append(vec![form])
        )};
        let user_login = panel("Log In", markup!(form|
            border_radius: "3px"
            self.append(&[
                form_field(
                    FormMeta {
                        value: model.login_name.clone(),
                        errors: vec![],
                        name: "Account Name",
                        placeholder: "Name",
                        type_: "text",
                    },
                    move |ref event| -> Msg {
                        Msg::LoginNameInput(
                            utils::event::get_oninput_value(event)
                        )
                    }
                ),
                form_field(
                    FormMeta {
                        value: model.login_password.0.clone(),
                        errors: vec![],
                        name: "Password",
                        placeholder: "Password",
                        type_: "password",
                    },
                    move |ref event| -> Msg {
                        Msg::LoginPasswordInput(
                            utils::event::get_oninput_value(event)
                        )
                    }
                ),
                form_submit(move |event| Msg::SubmitLogIn),
            ])
        ));
        
        
        let create_account = panel("Create Account", markup!(form|
            border_radius: "3px"
            self.append(&[
                form_field(
                    FormMeta {
                        value: model.new_name.clone(),
                        errors: model.new_name_errors.clone(),
                        name: "Account Name",
                        placeholder: "Name",
                        type_: "text",
                    },
                    move |ref event| -> Msg {
                        Msg::NewNameInput(
                            utils::event::get_oninput_value(event)
                        )
                    }
                ),
                form_field(
                    FormMeta {
                        value: model.new_password.0.clone(),
                        errors: model.new_password_errors.clone(),
                        name: "Password",
                        placeholder: "Password",
                        type_: "password",
                    },
                    move |ref event| -> Msg {
                        Msg::NewPasswordInput(
                            utils::event::get_oninput_value(event)
                        )
                    }
                ),
                form_field(
                    FormMeta {
                        value: model.new_password_confirm.0.clone(),
                        errors: model.new_password_confirm_errors.clone(),
                        name: "Re-Enter Password",
                        placeholder: "Password",
                        type_: "password",
                    },
                    move |ref event| -> Msg {
                        Msg::NewPasswordConfirmInput(
                            utils::event::get_oninput_value(event)
                        )
                    }
                ),
                form_submit(move |ref event| Msg::SubmitCreateAccount)
            ])
        ));
        
        markup!(
            background_color: "#ececec"
            width: "100%"
            height: "100%"
            @media [min_width: "900px"] (
                display: "grid"
                grid_template_columns: "0.5fr 1fr"
                grid_column_gap: "20px"
            )
            @media [max_width: "900px"] (
                display: "grid"
                grid_template_columns: "1fr"
                grid_row_gap: "20px"
            )
            div(
                width: "100%"
                height: "100%"
                background_color: "#fff"
                {user_login}
            )
            div(
                width: "100%"
                height: "100%"
                background_color: "#fff"
                {create_account}
            )
        )
    }
}


///////////////////////////////////////////////////////////////////////////////
// APP VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct FormMeta
{
    value: String,
    errors: Vec<String>,
    name: &'static str,
    placeholder: &'static str,
    type_: &'static str,
}


fn form_field(
    meta: FormMeta,
    on_input: impl Fn(JsValue) -> Msg + 'static
) -> Html<Msg>
{
    let ref input_id: String = format!("{}", rand::random::<u16>());
    markup!(
        margin_bottom: "18px"
        label(
            text_transform: "uppercase"
            font_family: "'Source Sans Pro', sans-serif"
            font_size: "1em"
            color: "#656565"
            for = {input_id}
            text(meta.name)
        )
        input(
            .input(on_input)
            ::placeholder (
                color: "#666"
            )
            font_family: "'Source Sans Pro', sans-serif"
            font_size: "1em"
            width: "100%"
            outline: "none"
            border: "1px solid #b1b1b1"
            border_radius: "3px"
            padding_left: "8px"
            placeholder={meta.placeholder}
            font_size: "1.1em"
            padding: "2px"
            padding_left: "6px"
            id = {input_id}
            type = {meta.type_}
            value = {meta.value}
        )
        {field_error_messages(&meta.errors)}
    )
}

fn form_submit(on_submit: impl Fn(JsValue) -> Msg + 'static) -> Html<Msg> {
    let ref input_id: String = format!("{}", rand::random::<u16>());
    markup!(
        input(
            ::placeholder (
                color: "#666"
            )
            margin_top: "12px"
            color: "#5a5a5a"
            text_transform: "lowercase"
            font_family: "'Source Sans Pro', sans-serif"
            font_size: "1em"
            width: "100%"
            outline: "none"
            border: "1px solid #b1b1b1"
            border_radius: "3px"
            font_size: "1.1em"
            padding: "2px"
            padding_left: "6px"
            font_size: "1.2em"
            text_transform: "uppercase"
            type = "submit"
            .click(move |event| {
                utils::event::prevent_default(&event);
                on_submit(event)
            })
            text("Submit")
        )
    )
}

pub fn field_error_messages(errors: &Vec<String>) -> Html<Msg> {
    markup!(ul|
        padding: "0"
        margin: "0"
        margin_left: "34px"
        margin_top: "6px"
        font_family: "'Source Sans Pro', sans-serif"
        text_transform: "uppercase"
        font_size: "0.9em"
        color: "#505050"
        self.append({
            errors
                .iter()
                .map(|msg| {
                    markup!(li|
                        text({msg})
                    )
                })
                .collect::<Vec<Html<Msg>>>()
        })
    )
}



///////////////////////////////////////////////////////////////////////////////
// APP FORM DATA HELPERS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Check {
    pub validate: Rc<Fn(String)->bool>,
    pub error_msg: &'static str,
}

pub fn run_check_for(checks: Vec<Check>, value: String) -> Vec<String> {
    if value.is_empty() {
        Vec::new()
    } else {
        checks
            .into_iter()
            .filter_map(|check| {
                if check.validate.as_ref()(value.clone()) {
                    None
                } else {
                    Some(String::from(check.error_msg))
                }
            })
            .collect::<Vec<String>>()
    }
}

pub fn new_name_checks() -> Vec<Check> {
    vec![
        Check {
            error_msg: "Must be ASCII",
            validate: Rc::new(|value| {
                value.is_ascii()
            })
        },
        Check {
            error_msg: "Invalid length",
            validate: Rc::new(|value| {
                value.len() >= 3 && 
                value.len() <= 100
            })
        },
        Check {
            error_msg: "Must not contain spaces",
            validate: Rc::new(|value| {
                !value.contains(" ")
            })
        },
        Check {
            error_msg: "Must be all lowercase",
            validate: Rc::new(|value| {
                !value.contains(char::is_uppercase)
            })
        },
    ]
}

pub fn new_password_checks() -> Vec<Check> {
    vec![
        Check {
            error_msg: "Must be ASCII",
            validate: Rc::new(|value| {
                value.is_ascii()
            })
        },
        Check {
            error_msg: "Invalid length",
            validate: Rc::new(|value| {
                value.len() >= 4 && 
                value.len() <= 100
            })
        },
    ]
}

pub fn new_password_confirm_checks(new_password: String) -> Vec<Check> {
    vec![
        Check {
            error_msg: "Passwords do not match",
            validate: Rc::new(move |value| {
                value == new_password
            })
        },
    ]
}
