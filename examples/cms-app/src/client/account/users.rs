use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use subscript::prelude::*;

use crate::client::data::*;
use crate::client::ui_utils::{self, text_theme};


///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct UsersSpec {
    pub session: Session,
    pub page: UsersPage,
}

pub enum Msg {
    NoOp,
    ToggleEditMode,
    ToggleAddUserMode,
    NameInput(String),
    PasswordInput(String),
    PasswordConfirmInput(String),
    SubmitNewUser,
    DeleteUser(Uuid, UserName),
}

#[derive(Default)]
pub struct Model {
    in_edit_mode: Signal<bool>,
    in_add_user_mode: Signal<bool>,
    name: Signal<String>,
    name_checks: Vec<Check>,
    password: Signal<String>,
    password_checks: Vec<Check>,
    password_confirm: Signal<String>,
    password_confirm_checks: Vec<Check>,
    loading: Signal<bool>,
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

impl Spec for UsersSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, startup: &Shell<Self>) -> Init<Self> {
        let name = Signal::new(String::new());
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
        let mut submit_new_user = |model: &mut Model, sh: &mut Shell<UsersSpec>| {
            let no_errors = {
                all_valid(&model.name_checks) &&
                all_valid(&model.password_checks) &&
                all_valid(&model.password_confirm_checks)
            };
            if no_errors {
                model.loading.set(true);
                let mut account = self.session.account.clone();
                let user_name = model.name.get_copy();
                let new_user = User {
                    id: Uuid::new_v4(),
                    ts: Timestamp::new(),
                    name: user_name.clone(),
                };
                account.users.insert(user_name, new_user);
                let session = Session::new(&account);
                model.in_add_user_mode.set(false);
                model.name.set(String::new());
                model.password.set(String::new());
                model.password_confirm.set(String::new());
                model.loading.set(false);
                sh.broadcast(NewSession(session));
            }
        };
        let mut delete_user = |model: &mut Model, sh: &mut Shell<UsersSpec>, uid: Uuid, name: String| {
            let mut account = self.session.account.clone();
            let removed_user = account.users.remove(&name);
            if let Some(removed_user) = removed_user {
                assert!(removed_user.id == uid);
                let session = Session::new(&account);
                sh.broadcast(NewSession(session));
            }
        };
        // GO!
        match msg {
            Msg::NoOp => {}
            Msg::ToggleEditMode => {
                model.in_edit_mode.set(!model.in_edit_mode.get_copy());
            }
            Msg::ToggleAddUserMode => {
                model.in_add_user_mode.set(!model.in_add_user_mode.get_copy());
            }
            Msg::NameInput(x) => {
                model.name.set(x);
            }
            Msg::PasswordInput(x) => {
                model.password.set(x);
            }
            Msg::PasswordConfirmInput(x) => {
                model.password_confirm.set(x);
            }
            Msg::SubmitNewUser => {
                submit_new_user(model, sh);
            }
            Msg::DeleteUser(uid, name) => {
                delete_user(model, sh, uid, name);
                // let deleted_user = self.session.account.users.remove()
            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        border: "1px solid #c3c3c3";
        background_color: "#fff";
        border_radius: "3px";
        width: "100%";
        height: "fit-content";

        header !{
            padding: "8px";
            border_top_left_radius: "3px";
            border_top_right_radius: "3px";
            border_bottom: "1px solid #c3c3c3";
            background_color: "#f6f6f7";
            display: "flex";
            justify_content: "space-between";
            align_items: "center";
            button !{
                text_theme();
                outline: "none";
                display: "flex";
                align_items: "center";
                border: "1px solid";
                padding: "0";
                margin: "0";
                border_radius: "2px";
                overflow: "hidden";
                border_color: "#b3b3b3";
                user_select: "none";
                transition: "0.5s";
                css.hover => s1!{
                    color: "#0089ff";
                    border_color: "#0089ff";
                };
                if &model.in_edit_mode => {
                    box_shadow: "0px 0px 2px #0436ea";
                };
                event.click[] => move || Msg::ToggleEditMode;
                if &model.in_edit_mode.map(|x| !x) => {
                    i !{
                        padding: "4px 8px";
                        border_right: "1px solid";
                        border_color: "inherit";
                        class = "fas fa-lock";
                    };
                };
                if &model.in_edit_mode => {
                    i !{
                        padding: "4px 8px";
                        border_right: "1px solid";
                        border_color: "inherit";
                        class = "fas fa-unlock";
                    };
                };
                span !{
                    padding: "0 8px";
                    font_weight: "400";
                    font_size: "1.1em";
                    "Edit Users";
                };
            };
            h1 !{
                margin: "0";
                font_size: "1.4em";
                font_weight: "500";
                text_align: "center";
                "Auxiliary Users";
            };
            button !{
                text_theme();
                outline: "none";
                display: "flex";
                align_items: "center";
                border: "1px solid";
                padding: "0";
                margin: "0";
                border_radius: "2px";
                overflow: "hidden";
                border_color: "#b3b3b3";
                user_select: "none";
                transition: "0.5s";
                css.hover => s1!{
                    color: "#0089ff";
                    border_color: "#0089ff";
                };
                if &model.in_add_user_mode => {
                    box_shadow: "0px 0px 2px #0436ea";
                };
                event.click[] => move || Msg::ToggleAddUserMode;
                if &model.in_add_user_mode.map(|x| !x) => {
                    i !{
                        padding: "4px 8px";
                        border_right: "1px solid";
                        border_color: "inherit";
                        class = "fas fa-plus";
                    };
                };
                if &model.in_add_user_mode => {
                    i !{
                        padding: "4px 8px";
                        border_right: "1px solid";
                        border_color: "inherit";
                        class = "fas fa-minus";
                    };
                };
                span !{
                    padding: "0 8px";
                    font_weight: "400";
                    font_size: "1.1em";
                    "Add User";
                };
            };
        };
        if &model.in_add_user_mode => {
            add_user_form(model);
        };
        if &Signal::new(self.session.account.users.is_empty()) => {
            h2 !{
                text_theme();
                text_align: "center";
                padding: "20px";
                font_size: "3em";
                margin: "0";
                font_weight: "600";
                color: "#ccc";
                "Empty";
            };
        };
        if &Signal::new(!self.session.account.users.is_empty()) => {
            users_list(self, model);
        };
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS - ADD USER
///////////////////////////////////////////////////////////////////////////////

fn add_user_form(model: &Model) -> View<Msg> {
    let name_id = format!("id-{}", rand::random::<u16>());
    let password_id = format!("id-{}", rand::random::<u16>());
    let password_confirm_id = format!("id-{}", rand::random::<u16>());
    v1!{
        form !{
            display: "flex";
            flex_direction: "column";
            border_bottom: "1px solid #c3c3c3";
            padding: "8px";
            div !{
                display: "flex";
                flex_direction: "column";
                width: "100%";
                margin_bottom: "8px";
                label !{
                    text_theme();
                    font_size: "1em";
                    for = &name_id;
                    "Name";
                };
                input !{
                    text_theme();
                    font_size: "1em";
                    outline: "none";
                    id = name_id;
                    type = "text";
                    value = &model.name;
                    event.input[] => move |x| Msg::NameInput(x);
                };
            };
            render_checks(&model.name_checks);
            div !{
                display: "flex";
                flex_direction: "column";
                width: "100%";
                margin_bottom: "8px";
                label !{
                    text_theme();
                    font_size: "1em";
                    for = &password_id;
                    "Password";
                };
                input !{
                    text_theme();
                    font_size: "1em";
                    outline: "none";
                    id = password_id;
                    type = "password";
                    value = &model.password;
                    event.input[] => move |x| Msg::PasswordInput(x);
                };
            };
            render_checks(&model.password_checks);
            div !{
                display: "flex";
                flex_direction: "column";
                width: "100%";
                margin_bottom: "8px";
                label !{
                    text_theme();
                    font_size: "1em";
                    for = &password_confirm_id;
                    "Confirm Password";
                };
                input !{
                    text_theme();
                    font_size: "1em";
                    outline: "none";
                    id = password_confirm_id;
                    type = "password";
                    value = &model.password_confirm;
                    event.input[] => move |x| Msg::PasswordConfirmInput(x);
                };
            };
            render_checks(&model.password_confirm_checks);
            input !{
                text_theme();
                outline: "none";
                border: "1px solid #d2d2d2";
                width: "100%";
                text_transform: "uppercase";
                font_weight: "400";
                margin_top: "8px";
                padding: "4px";
                font_size: "1em";
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
                    Msg::SubmitNewUser
                };
                type = "submit";
                value = "Submit";
            };
        };
    }
}

pub fn render_checks(checks: &Vec<Check>) -> View<Msg> {v1!{
    ul !{
        padding: "0";
        margin: "0";
        margin_left: "34px";
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
                css.first_child => s1!{
                    margin_top: "8px";
                };
                font_weight: "100";
                font_size: "0.9em";
                color: "#ff6262";
                check.error_msg.clone();
            };
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS - MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

fn users_list(spec: &UsersSpec, model: &Model) -> View<Msg> {v1!{
    ul !{
        list_style: "none";
        padding: "0";
        margin: "0";
        display: "grid";
        grid_template_columns: "1fr 1fr 1fr 1fr";
        grid_auto_rows: "50px";
        grid_column_gap: "10px";
        grid_row_gap: "10px";
        padding: "10px";

        spec.session.account.users
            .values()
            .map(|x| user_item(model, x))
            .collect::<Vec<_>>();
    };
}}

fn user_item(model: &Model, user: &User) -> View<Msg> {v1!{
    li !{
        display: "flex";
        // justify_content: "center";
        justify_content: "space-between";
        align_items: "center";
        background_color: "#f7f7f7";
        border: "1px solid #c3c3c3";

        if &model.in_edit_mode => {
            button !{
                height: "100%";
                display: "flex";
                align_items: "center";
                padding: "0 4px";
                outline: "none";
                border: "none";
                border_right: "1px solid #c3c3c3";
                transition: "box-shadow 0.5s";
                css.hover => s1!{
                    box_shadow: "0px 0px 2px #ea0404";
                    border_right: "1px solid transparent";
                    z_index: "1";
                };
                event.click[id@user.id, name@user.name] => move || {
                    Msg::DeleteUser(id, name)
                };
                i !{class = "fas fa-trash-alt";};
            };
        };
        span !{
            width: "100%";
            text_align: "center";
            &user.name;
        };
    };
}}

