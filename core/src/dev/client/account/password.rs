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
use uuid::Uuid;

use crate::browser::*;
use crate::tree::offline::data::*;
use crate::tree::offline::api::*;
use crate::tree::online::data::*;
use crate::dev::server::data::*;
use crate::dev::client::data::*;
use crate::dev::client::utils;
use crate::extras::*;

use crate::process::app::*;
use crate::process::basics::*;
use crate::process::online::*;



///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct PasswordSpec {
    pub session: Reactive<Option<Session>>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
    Session(Option<Session>),
    OldPasswordInput(String),
    NewPasswordInput(String),
    NewPasswordConfirmInput(String),
    SubmitResetPassword,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Model {
    session: Option<Session>,
    valid_submit: Option<bool>,
    server_reset_password_errors: Vec<String>,
    old_password: PasswordString,
    new_password: PasswordString,
    new_password_confirm: PasswordString,
    old_password_errors: Vec<String>,
    new_password_errors: Vec<String>,
    new_password_confirm_errors: Vec<String>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            session: None,
            valid_submit: None,
            server_reset_password_errors: Vec::new(),
            old_password: PasswordString(String::new()),
            new_password: PasswordString(String::new()),
            new_password_confirm: PasswordString(String::new()),
            old_password_errors: Vec::new(),
            new_password_errors: Vec::new(),
            new_password_confirm_errors: Vec::new(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl Spec for PasswordSpec {
    type Model = Model;
    type Msg = Msg;
    
    fn init(&self, loaded: InitArgs<Self::Model>, key: &InitKey) -> Init<Self::Model, Self::Msg> {
        Init {
            model: Model {
                session: self.session.unlock(key),
                ..Default::default()
            },
            subs: subscriptions!(
                on(self.session -> new_value) -> Msg {
                    Msg::Session(new_value)
                }
            ),
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd) {
        use crate::dev::client::login::{
            run_check_for,
            new_password_checks,
            new_password_confirm_checks,
        };
        match msg {
            Msg::NoOp => (),
            Msg::Session(session) => {
                model.session = session;
            }
            Msg::OldPasswordInput(value) => {
                model.old_password = PasswordString(value);
            }
            Msg::NewPasswordInput(value) => {
                model.new_password_errors = run_check_for(
                    new_password_checks(),
                    value.clone(),
                );
                model.new_password = PasswordString(value);
            }
            Msg::NewPasswordConfirmInput(value) => {
                model.new_password_confirm_errors = run_check_for(
                    new_password_confirm_checks(model.new_password.0.clone()),
                    value.clone(),
                );
                model.new_password_confirm = PasswordString(value);
            }
            Msg::SubmitResetPassword => {
                let not_empty = {
                    (!model.old_password.0.is_empty()) &&
                    (!model.new_password.0.is_empty()) &&
                    (!model.new_password_confirm.0.is_empty())
                };
                let no_errors = {
                    model.old_password_errors.is_empty() &&
                    model.new_password_errors.is_empty() &&
                    model.new_password_confirm_errors.is_empty()
                };
                if not_empty && no_errors {
                    if let Some(session) = &model.session {
                        let account = Account::new(session.user_name.as_str());
                        let session = Session {
                            user_id: account.account_master.user_id.clone(),
                            user_name: account.account_master.user_name.clone(),
                            encoded_token: String::from(""),
                            account: account,
                        };
                        *model = Default::default();
                        cmd.broadcast(NewSession(session));
                    } else {
                        model.valid_submit = Some(false);
                    }
                } else {
                    model.valid_submit = Some(false);
                }
            }
        }
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        if model.session.is_none() {
            html!()
        } else {
            password(model)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// APP VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn password(model: &Model) -> Html<Msg> {
    let ref old_password_id = format!("{}", rand::random::<u16>());
    let ref new_password_id = format!("{}", rand::random::<u16>());
    let ref new_password_confirm_id = format!("{}", rand::random::<u16>());
    html!(main|
        @media [min_width: "1100px"] (
            padding_right: "100px"
        )
        padding_right: "20px"
        h1(
            color: "#5a5a5a"
            font_family: "'Source Sans Pro', sans-serif"
            margin: "0"
            padding: "0"
            padding_bottom: "4px"
            text("Change Password")
        )
        form(
            font_family: "'Source Sans Pro', sans-serif"
            font_size: "1em"
            color: "#656565"
            
            if(let Some(false) = model.valid_submit)(
                h2(
                    text("Submit Failed")
                )
            )
            div(
                display: "flex"
                flex_direction: "column"
                margin_bottom: "12px"
                label(
                    text_transform: "uppercase"
                    for = {old_password_id}
                    text("Old password")
                )
                input(
                    .input(|event| {
                        let value = utils::event::get_oninput_value(&event);
                        Msg::OldPasswordInput(value)
                    })
                    ::placeholder (
                        color: "#666"
                    )
                    outline: "none"
                    font_family: "'Source Sans Pro', sans-serif"
                    font_size: "1em"
                    width: "100%"
                    border: "1px solid #b1b1b1"
                    border_radius: "3px"
                    padding: "2px"
                    padding_left: "8px"
                    id = {old_password_id}
                    placeholder = "Old Password"
                    type = "password"
                    value = {model.old_password.0.as_str()}
                )
                self.append(&[
                    field_error_messages(&model.old_password_errors)
                ])
            )
            div(
                display: "flex"
                flex_direction: "column"
                margin_bottom: "12px"
                label(
                    text_transform: "uppercase"
                    for = {new_password_id}
                    text("New password")
                )
                input(
                    .input(|event| {
                        let value = utils::event::get_oninput_value(&event);
                        Msg::NewPasswordInput(value)
                    })
                    ::placeholder (
                        color: "#666"
                    )
                    outline: "none"
                    font_family: "'Source Sans Pro', sans-serif"
                    font_size: "1em"
                    width: "100%"
                    border: "1px solid #b1b1b1"
                    border_radius: "3px"
                    padding: "2px"
                    padding_left: "8px"
                    id = {new_password_id}
                    placeholder = "New Password"
                    type = "password"
                    value = {model.new_password.0.as_str()}
                )
                self.append(&[
                    field_error_messages(&model.new_password_errors)
                ])
            )
            div(
                display: "flex"
                flex_direction: "column"
                margin_bottom: "12px"
                label(
                    text_transform: "uppercase"
                    for = {new_password_confirm_id}
                    text("Confirm new password")
                )
                input(
                    .input(|event| {
                        let value = utils::event::get_oninput_value(&event);
                        Msg::NewPasswordConfirmInput(value)
                    })
                    ::placeholder (
                        color: "#666"
                    )
                    outline: "none"
                    font_family: "'Source Sans Pro', sans-serif"
                    font_size: "1em"
                    width: "100%"
                    border: "1px solid #b1b1b1"
                    border_radius: "3px"
                    padding: "2px"
                    padding_left: "8px"
                    id = {new_password_confirm_id}
                    placeholder = "New Password"
                    type = "password"
                    value = {model.new_password_confirm.0.as_str()}
                )
                self.append(&[
                    field_error_messages(&model.new_password_confirm_errors)
                ])
            )
            self.append(&[
                form_submit(|event| {
                    Msg::SubmitResetPassword
                })
            ])
        )
    )
}


fn form_submit(on_submit: impl Fn(JsValue) -> Msg + 'static) -> Html<Msg> {
    let ref input_id: String = format!("{}", rand::random::<u16>());
    html!(
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
            value = "Submit"
        )
    )
}


pub fn field_error_messages(errors: &Vec<String>) -> Html<Msg> {
    html!(ul|
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
                    html!(li|
                        text({msg})
                    )
                })
                .collect::<Vec<Html<Msg>>>()
        })
    )
}


