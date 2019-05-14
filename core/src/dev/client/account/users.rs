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
pub struct UsersSpec {
    pub session: Reactive<Option<Session>>,
    pub page: Reactive<UsersPage>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
    Session(Option<Session>),
    UrlRequest(UsersPage),
    UrlChanged(UsersPage),
    ToggleEditMode,
    SubmitNewUser,
    NewNameInput(String),
    NewPasswordInput(String),
    NewPasswordConfirmInput(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Model {
    session: Option<Session>,
    page: UsersPage,
    edit_node: bool,
    new_user_server_errors: Vec<String>,
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
            session: None,
            edit_node: false,
            page: Default::default(),
            new_user_server_errors: Vec::new(),
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

impl Spec for UsersSpec {
    type Model = Model;
    type Msg = Msg;
    
    fn init(&self, loaded: InitArgs<Self::Model>, key: &InitKey) -> Init<Self::Model, Self::Msg> {
        Init {
            model: Model {
                session: self.session.unlock(key),
                page: self.page.unlock(key),
                ..Default::default()
            },
            subs: subscriptions!(
                on(self.session -> new_value) -> Msg {
                    Msg::Session(new_value)
                }
                on(self.page -> new_value) -> Msg {
                    Msg::UrlChanged(new_value)
                }
            ),
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd) {
        use crate::dev::client::login::{
            run_check_for,
            new_name_checks,
            new_password_checks,
            new_password_confirm_checks,
        };
        match msg {
            Msg::NoOp => (),
            Msg::Session(session) => {
                model.session = session;
            }
            Msg::UrlChanged(page) => {
                model.page = page;
            },
            Msg::UrlRequest(page) => cmd.broadcast(
                NewPage(Page::Account(AccountPage::Users(page)))
            ),
            Msg::ToggleEditMode => {
                model.edit_node = !model.edit_node;
            }
            Msg::NewNameInput(text) => {
                model.new_name_errors = run_check_for(
                    new_name_checks(),
                    text.clone(),
                );
                model.new_name = text;
            },
            Msg::NewPasswordInput(text) => {
                model.new_password_errors = run_check_for(
                    new_password_checks(),
                    text.clone(),
                );
                model.new_password = PasswordString(text);
            },
            Msg::NewPasswordConfirmInput(text) => {
                model.new_password_confirm_errors = run_check_for(
                    new_password_confirm_checks(
                        model.new_password.0.clone()
                    ),
                    text.clone(),
                );
                model.new_password_confirm = PasswordString(text);
            },
            Msg::SubmitNewUser => {
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
                    model.new_user_server_errors = Vec::new();
                    model.new_name = String::new();
                    model.new_password = PasswordString(String::new());
                    model.new_password_confirm = PasswordString(String::new());
                    model.new_name_errors = Vec::new();
                    model.new_password_errors = Vec::new();
                    model.new_password_confirm_errors = Vec::new();
                }
            },
        }
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        if model.session.is_none() {
            html!()
        } else {
            match &model.page {
                UsersPage::Index => users_overview(model),
                UsersPage::AddUser => add_user(model),
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// APP VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn users_overview(model: &Model) -> Html<Msg> {
    let user = || html!(
        display: "flex"
        flex_direction: "row"
        background_color: "#f9f9f9"
        color: "#5a5a5a"
        font_family: "'Source Sans Pro', sans-serif"
        border: "1px solid #dcdcdc"
        border_radius: "3px"
        div(
            width: "100%"
            height: "100%"
            padding: "12px"
            span(
                text("User Name")
            )
        )
        div(
            if (model.edit_node)(
                display: "flex"
            )
            if (!model.edit_node)(
                display: "none"
            )
            align_items: "center"
            justify_content: "center"
            padding_left: "8px"
            padding_right: "8px"
            border_left: "1px solid #dcdcdc"
            button(
                - mix::button();
                color: "#c52a2a"
                padding: "4px 8px"
                i(class = "fas fa-user-minus")
            )
        )
    );
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
            text("Users overview")
        )
        hr(
            border: "1px solid #eaeaea"
        )
        header(
            margin_bottom: "8px"
            display: "flex"
            align_items: "center"
            justify_content: "space-between"
            button(
                - mix::button();
                .click(|_| Msg::UrlRequest(UsersPage::AddUser))
                padding: "0"
                font_size: "0.8em"
                display: "flex"
                align_items: "center"
                i(
                    class = "fas fa-user-plus"
                    padding: "4px 6px"
                    padding_top: "6px"
                    border_right: "1px solid #a0a0a0"
                )
                span(
                    padding_left: "6px"
                    padding_right: "6px"
                    text("Add User")
                )
            )
            button(
                - mix::button();
                .click(|_| Msg::ToggleEditMode)
                if(model.edit_node)(
                    box_shadow: "0 0 2px 0px #9c9c9c"
                )
                padding: "0"
                font_size: "0.8em"
                display: "flex"
                align_items: "center"
                i(
                    class = "fas fa-user-edit"
                    padding: "4px 6px"
                    padding_top: "6px"
                    border_right: "1px solid #a0a0a0"
                )
                span(
                    padding_left: "6px"
                    padding_right: "6px"
                    text("Edit Users")
                )
            )
        )
        div(
            @media [min_width: "900px", max_width: "1100px"] (
                grid_template_columns: "repeat(3, 1fr)"
            )
            @media [min_width: "1100px"] (
                grid_template_columns: "repeat(4, 1fr)"
            )
            grid_template_columns: "repeat(2, 1fr)"
            grid_template_rows: "repeat(4, 1fr)"
            display: "grid"
            grid_gap: "10px"
            grid_auto_rows: "100px"
            [
                user(),
                user(),
                user(),
                user(),
                user(),
                user(),
                user(),
                user(),
            ]
        )
    )
}

pub fn add_user(model: &Model) -> Html<Msg> {
    html!(
        @media [min_width: "1100px"] (
            padding_right: "100px"
        )
        padding_right: "20px"
        {breadcrumbs(vec![
            Breadcrumb {
                value: "users overview",
                on_click: Msg::UrlRequest(UsersPage::Index),
                active: model.page.is_index(),
            },
            Breadcrumb {
                value: "add user",
                on_click: Msg::UrlRequest(UsersPage::AddUser),
                active: model.page.is_add_user(),
            },
        ])}
        hr(
            border: "1px solid #eaeaea"
        )
        [new_user_form(model)]
    )
}

#[derive(Clone, Debug)]
pub struct Breadcrumb {
    pub value: &'static str,
    pub on_click: Msg,
    pub active: bool,
}

pub fn breadcrumbs(xs: Vec<Breadcrumb>) -> Html<Msg> {
    use itertools::{Itertools, Position};
    
    let breadcrumb = |is_last: bool, x: Breadcrumb| html!(li|
        color: "#5a5a5a"
        font_family: "'Source Sans Pro', sans-serif"
        if (!is_last)(
            :after(
                color: "#adadad"
                content: "\"/\""
                padding_left: ".5em"
                padding_right: ".5em"
            )
        )
        a(
            if(!x.active)(
                :hover(
                    border_bottom: "1px solid #000"
                )
            )
            if(x.active)(
                font_weight: "600"
            )
            .click({
                let x = x.clone();
                move |_| x.on_click.clone()
            })
            text(x.value)
        )
    );
    html!(ol|
        list_style: "none"
        display: "flex"
        padding: "0"
        margin: "0"
        self.append(
            xs  .into_iter()
                .with_position()
                .map(|x| {
                    match x {
                        Position::Last(x) => breadcrumb(true, x),
                        _ => breadcrumb(false, x.into_inner()),
                    }
                })
                .collect::<Vec<Html<Msg>>>()
        )
    )
}

pub fn new_user_form(model: &Model) -> Html<Msg> {
    html!(form|
        border_radius: "3px"
        self.append(&[
            form_field(
                FormMeta {
                    value: model.new_name.clone(),
                    errors: model.new_name_errors.clone(),
                    name: "User Name",
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
            form_submit(move |ref event| Msg::SubmitNewUser)
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


