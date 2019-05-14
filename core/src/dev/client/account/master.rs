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

use crate::toolkit;
use crate::toolkit::mixins as mix;

use crate::process::app::*;
use crate::process::basics::*;
use crate::process::online::*;



///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AccountMasterSpec {
    pub session: Reactive<Option<Session>>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
    Session(Option<Session>),
    AccountNameInput(String),
    SubmitDeleteAccount
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Model {
    session: Option<Session>,
    account_name: String,
    account_name_errors: Vec<String>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            session: None,
            account_name: String::new(),
            account_name_errors: Vec::new(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl Spec for AccountMasterSpec {
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
        };
        match msg {
            Msg::NoOp => (),
            Msg::Session(session) => {
                model.session = session;
            }
            Msg::AccountNameInput(text) => {
                if let Some(session) = &model.session {
                    model.account_name_errors = run_check_for(
                        delete_account_name_checks(session.account.account_name.clone()),
                        text.clone(),
                    );
                    model.account_name = text;
                }
            }
            Msg::SubmitDeleteAccount => {
                model.account_name = String::new();
            }
        }
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        if model.session.is_none() {
            html!()
        } else {
            settings(model)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// APP VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn settings(model: &Model) -> Html<Msg> {
    html!(
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
            text("Delete Account")
        )
        hr(
            border: "1px solid #eaeaea"
        )
        [delete_account(model)]
    )
}

pub fn delete_account(model: &Model) -> Html<Msg> {
    html!(form|
        border_radius: "3px"
        self.append(&[
            form_field(
                FormMeta {
                    value: model.account_name.clone(),
                    errors: model.account_name_errors.clone(),
                    name: "Account Name",
                    placeholder: "Account Name",
                    type_: "text",
                },
                move |ref event| -> Msg {
                    Msg::AccountNameInput(
                        utils::event::get_oninput_value(event)
                    )
                }
            ),
            form_submit(move |ref event| Msg::SubmitDeleteAccount)
        ])
    )
}

///////////////////////////////////////////////////////////////////////////////
// NEW-USER FORM
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
    html!(
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

pub fn delete_account_name_checks(account_name: String) -> Vec<crate::dev::client::login::Check> {
    vec![
        crate::dev::client::login::Check {
            error_msg: "Name does not match",
            validate: Rc::new(move |value| {
                value == account_name
            })
        },
    ]
}


