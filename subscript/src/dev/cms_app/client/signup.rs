use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi};
use crate::reactive_sys::*;
use crate::view_sys::runtime::common::ElementEnv;
use crate::view_sys::shared::*;
use crate::view_sys::{dom, dsl, runtime, dom::{Dom, Element}, dsl::{View, Dsl}};
use crate::view_sys::adapters::*;
use crate::program_sys::instances::Component;
use crate::program_sys::spec::*;
use crate::program_sys::{self, Program};

use crate::dev::cms_app::client::data::*;
use crate::dev::cms_app::client::ui_utils::{self, text_theme};


///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct SignupSpec {}

pub enum Msg {
    NoOp,
    Input {
        to: FormField,
        value: String,
    },
    Submit
}

#[derive(Default)]
pub struct Model {
    loading: Signal<bool>,
    name: Signal<String>,
    password: Signal<String>,
    password_confirm: Signal<String>,
    name_checks: Vec<Check>,
    password_checks: Vec<Check>,
    password_confirm_checks: Vec<Check>,
}

#[derive(Clone, PartialEq)]
pub enum FormField {
    Name,
    Password,
    PasswordConfirm,
}

///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////


#[derive(Clone)]
pub struct Check {
    pub error_msg: String,
    pub active: Formula<bool>,
    pub valid: Formula<bool>,
}

pub fn name_checks(name: &Signal<String>) -> Vec<Check> {
    let name = name.clone();
    let active = name.map(|value| !value.is_empty());
    let mut xs = Vec::new();
    xs.push(Check {
        error_msg: String::from("Must be ASCII"),
        active: active.clone(),
        valid: name.map(|value| value.is_ascii()),
    });
    xs.push(Check {
        error_msg: String::from("Invalid length"),
        active: active.clone(),
        valid: name.map(|value| {
            value.len() >= 3 && 
            value.len() <= 20
        })
    });
    xs.push(Check {
        error_msg: String::from("Must not contain spaces"),
        active: active.clone(),
        valid: name.map(|value| {
            !value.contains(" ")
        })
    });
    xs.push(Check {
        error_msg: String::from("Must be all lowercase"),
        active: active.clone(),
        valid: name.map(|value| {
            !value.contains(char::is_uppercase)
        })
    });
    xs
}
pub fn password_checks(password: &Signal<String>) -> Vec<Check> {
    let password = password.clone();
    let active = password.map(|value| !value.is_empty());
    vec![
        Check {
            error_msg: String::from("Must be ASCII"),
            active: active.clone(),
            valid: password.map(|value| value.is_ascii()),
        },
        Check {
            error_msg: String::from("Must not contain spaces"),
            active: active.clone(),
            valid: password.map(|value| {
                !value.contains(" ")
            })
        },
        Check {
            error_msg: String::from("Invalid length"),
            active: active.clone(),
            valid: password.map(|value| value.len() >= 4 && value.len() <= 100),
        },
    ]
}
pub fn password_confirm_checks(new_password: &Signal<String>, password_confirm: &Signal<String>) -> Vec<Check> {
    let new_password = new_password.clone();
    let password_confirm = password_confirm.clone();
    let active = new_password.clone().zip(&password_confirm).map(|(pass, confrm)| {
        !pass.is_empty() && !confrm.is_empty()
    });
    let valid = new_password.clone().zip(&password_confirm).map(|(pass, confrm)| {
        pass == confrm
    });
    vec![
        Check {error_msg: String::from("Passwords do not match"), active, valid},
    ]
}

pub fn all_valid(checks: &Vec<Check>) -> bool {
    checks
        .iter()
        .all(|check| check.valid.get_copy())
}

///////////////////////////////////////////////////////////////////////////////
// VIEWABLE DATA TYPES
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Spec for SignupSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, sh: &Shell<Self>) -> Init<Self> {
        let name: Option<String> = sh
            .cache()
            .get(CACHE_LOGIN_NAME_KEY);
        let name = Signal::new(name.unwrap_or(String::new()));
        let password = Signal::new(String::new());
        let password_confirm = Signal::new(String::new());
        let name_checks = name_checks(&name);
        let password_checks = password_checks(&password);
        let password_confirm_checks = password_confirm_checks(&password, &password_confirm);
        let model = Model {
            name,
            password,
            password_confirm,
            name_checks,
            password_checks,
            password_confirm_checks,
            ..Default::default()
        };
        Init {model, ..Default::default()}
    }
    fn update(&self, model: &mut Model, msg: Msg, sh: &mut Shell<Self>) {
        // HELPERS
        fn on_input(model: &mut Model, sh: &mut Shell<SignupSpec>, to: FormField, value: String) {
            match to {
                FormField::Name => {
                    model.name.set(value);
                }
                FormField::Password => {
                    model.password.set(value);
                }
                FormField::PasswordConfirm => {
                    model.password_confirm.set(value);
                }
            }
        }
        fn on_submit(model: &mut Model, sh: &mut Shell<SignupSpec>) {
            let no_errors = {
                all_valid(&model.name_checks) &&
                all_valid(&model.password_checks) &&
                all_valid(&model.password_confirm_checks)
            };
            if no_errors {
                model.loading.set(true);
                sh  .cache()
                    .insert(CACHE_LOGIN_NAME_KEY, model.name.get().as_ref());
                let account = Account::new(model.name.get().as_str());
                let session = Session::new(&account);
                model.loading.set(false);
                model.name.set(String::new());
                model.password.set(String::new());
                model.password_confirm.set(String::new());
                sh.broadcast(NewSession(session));
            }
        }
        // GO!
        match msg {
            Msg::NoOp => {}
            Msg::Input{to, value} => {
                on_input(model, sh, to, value);
            }
            Msg::Submit => {
                on_submit(model, sh);
            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        width: "100%";
        height: "100%";
        display: "flex";
        justify_content: "center";
        align_items: "center";

        div !{
            background_color: "#fff";
            border_radius: "3px";
            margin_top: "-20vh";

            css.media[min_width: "900px"] => s1!{
                width: "50%";
            };
            css.media[max_width: "900px"] => s1!{
                width: "80%";
            };

            h1 !{
                margin: "0";
                padding: "8px";
                text_align: "center";
                background_color: "#0089ff";
                color: "#fff";
                border_top_left_radius: "3px";
                border_top_right_radius: "3px";
                "Create Account";
            };
            form !{
                padding: "8px";
                form_field(
                    FormField::Name,
                    "Name",
                    &model.name,
                    &model.name_checks,
                );
                form_field(
                    FormField::Password,
                    "Password",
                    &model.password,
                    &model.password_checks,
                );
                form_field(
                    FormField::PasswordConfirm,
                    "Confirm Password",
                    &model.password_confirm,
                    &model.password_confirm_checks,
                );
                input !{
                    text_theme();
                    outline: "none";
                    border: "1px solid #d2d2d2";
                    width: "100%";
                    font_size: "1em";
                    padding: "8px";
                    text_transform: "uppercase";
                    font_weight: "400";
                    margin_top: "8px";
                    margin_bottom: "8px";
                    if &model.loading.map(|x| !x) => {
                        css.hover => s1!{
                            box_shadow: "0 0 4px 1px #e0e0e0";
                        };
                    };
                    if &model.loading => {
                        background:
                            "repeating-linear-gradient(
                                -45deg, \
                                hsl(0, 0%, 94%), \
                                hsl(0, 0%, 94%) 11px, \
                                hsl(0, 0%, 88%) 10px, \
                                hsl(0, 0%, 88%) 20px /* determines size */ \
                            )";
                          background_size: "28px 28px";
                          animation: "default-move 1s linear infinite";
                    };
                    css.animation => {
                        from => s1!{
                            background_position: "0 0";
                        };
                        to => s1!{
                            background_position: "28px 0";
                        };
                    };
                    event.click[] => move || {
                        Msg::Submit
                    };
                    type = "submit";
                    value = "Submit";
                };
            };
        };
    }}
}

///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn form_field(
    field: FormField,
    label: &str,
    value: &Signal<String>,
    checks: &Vec<Check>,
) -> View<Msg> {
    let id = format!("id-{}", rand::random::<u16>());
    let input_type = if (field == FormField::Password) || (field == FormField::PasswordConfirm) {
        "password"
    } else {
        "text"
    };
    v1!{
        div !{
            display: "flex";
            flex_direction: "column";
            margin_bottom: "8px";

            css.last_child => s1!{
                margin_bottom: "0";
            };

            label !{
                text_theme();
                font_size: "1em";

                for = id.clone();
                label;
            };
            input !{
                text_theme();
                font_size: "1em";
                outline: "none";
                
                event.input[field] => move |value| {
                    let to = field;
                    Msg::Input{to, value}
                };
                value = value;
                type = input_type;
                id = id;
            };
            render_checks(checks);
        };
    }
}

pub fn render_checks(checks: &Vec<Check>) -> View<Msg> {v1!{
    ul !{
        padding: "0";
        margin: "0";
        margin_left: "34px";
        margin_top: "6px";
        font_family: "'Source Sans Pro', sans-serif";
        text_transform: "uppercase";
        font_size: "0.9em";
        color: "#505050";
        checks
            .clone()
            .iter()
            .map(|check| render_check(check))
            .collect::<Vec<_>>();
    };
}}

pub fn render_check(check: &Check) -> View<Msg> {
    let pred = check.active.zip(&check.valid).map(|(active, valid)| active.clone() && !valid);
    v1!{
        if &pred => {
            li !{
                font_weight: "100";
                font_size: "0.9em";
                color: "#ff6262";
                check.error_msg.clone();
            };
        };
    }
}


