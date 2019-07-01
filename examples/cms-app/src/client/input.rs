use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};
use subscript::prelude::*;
use uuid::Uuid;

use crate::client::AppSpec;
use crate::client::data::*;
use crate::client::ui_utils::{self, text_theme};


///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct InputSpec {
    pub session: Session,
}

pub enum Msg {
    NoOp,
    UrlRequest(Page),
    ToggleEditMode,
    ToggleAddInputMode,
    SubmitNewInput,
    DeleteInput(Uuid),
    InputType(InputType),
    Name(String),
    HttpAddress(String),
}

#[derive(Default)]
pub struct Model {
    in_edit_mode: Signal<bool>,
    in_add_input_mode: Signal<bool>,
    loading: Signal<bool>,
    input_type: Signal<InputType>,
    name: Signal<String>,
    http_address: Signal<String>,
    name_checks: Vec<Check>,
    http_address_checks: Vec<Check>,
}

#[derive(PartialEq, Clone)]
pub enum InputType {
    Http,
    AwsS3,
    GoogleStorage,
}

impl Default for InputType {
    fn default() -> Self {
        InputType::Http
    }
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

pub fn all_valid(checks: &Vec<Check>) -> bool {
    checks
        .iter()
        .all(|check| check.valid.get_copy())
}
pub fn name_checks(account: &Account, name: &Signal<String>) -> Vec<Check> {
    let name = name.clone();
    let active = name.map(|value| !value.is_empty());
    let mut xs = Vec::new();
    xs.push(Check {
        error_msg: String::from("Already taken"),
        active: active.clone(),
        valid: name.map({
            let account = account.clone();
            move |value| {
                !account.inputs.contains_key(value)
            }
        }),
    });
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
pub fn http_address_checks(address: &Signal<String>) -> Vec<Check> {
    let address = address.clone();
    let active = address.map(|value| !value.is_empty());
    let mut xs = Vec::new();
    xs.push(Check {
        error_msg: String::from("Must be ASCII"),
        active: active.clone(),
        valid: address.map(|value| value.is_ascii()),
    });
    xs.push(Check {
        error_msg: String::from("Invalid length"),
        active: active.clone(),
        valid: address.map(|value| {
            value.len() >= 3 && 
            value.len() <= 200
        })
    });
    xs.push(Check {
        error_msg: String::from("Must not contain spaces"),
        active: active.clone(),
        valid: address.map(|value| {
            !value.contains(" ")
        })
    });
    xs.push(Check {
        error_msg: String::from("Must be all lowercase"),
        active: active.clone(),
        valid: address.map(|value| {
            !value.contains(char::is_uppercase)
        })
    });
    xs
}


///////////////////////////////////////////////////////////////////////////////
// VIEWABLE DATA TYPES
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Spec for InputSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, startup: &Shell<Self>) -> Init<Self> {
        let name = if self.session.account.inputs.is_empty() {
            Signal::new(String::from(self.session.account.name.as_str()))
        } else {
            Default::default()
        };
        let http_address = Default::default();
        let name_checks = name_checks(&self.session.account, &name);
        let http_address_checks = http_address_checks(&http_address);
        let model = Model {
            name,
            http_address,
            name_checks,
            http_address_checks,
            ..Default::default()
        };
        Init {
            model,  
            ..Default::default()
        }
    }
    fn update(&self, model: &mut Model, msg: Msg, sh: &mut Shell<Self>) {
        // HELPERS
        let mut submit_new_input = |model: &mut Model, sh: &mut Shell<InputSpec>| {
            model.loading.set(true);
            match model.input_type.get_copy() {
                InputType::Http => {
                    let no_errors = {
                        all_valid(&model.name_checks) &&
                        all_valid(&model.http_address_checks)
                    };
                    if no_errors {
                        let mut account = self.session.account.clone();
                        let input_name = model.name.get_copy();
                        account.inputs.insert(input_name.clone(), Input {
                            id: Uuid::new_v4(),
                            ts: Timestamp::new(),
                            name: input_name.clone(),
                            driver: InputDriver::Http {
                                address: model.http_address.get_copy(),
                            },
                        });
                        let session = Session::new(&account);
                        model.in_add_input_mode.set(Default::default());
                        model.name.set(Default::default());
                        model.http_address.set(Default::default());
                        sh.broadcast(NewSession(session));
                    }
                }
                InputType::AwsS3 => {

                }
                InputType::GoogleStorage => {

                }
            }
            model.loading.set(false);
        };
        let mut delete_input = |model: &mut Model, sh: &mut Shell<InputSpec>, uid: Uuid| {
            let mut account = self.session.account.clone();
            let key = account.inputs
                .iter()
                .find_map(|(k, v)| {
                    let mut result = None;
                    if v.id == uid {
                        result = Some(k.clone());
                    }
                    result
                });
            assert!(key.is_some());
            if let Some(key) = key {
                account.inputs.remove(&key);
                let session = Session::new(&account);
                sh.broadcast(NewSession(session));
            }
        };
        // GO!
        match msg {
            Msg::NoOp => {}
            Msg::UrlRequest(page) => {
                sh.message::<AppSpec, _>(UrlRequest(page));
            }
            Msg::ToggleEditMode => {
                model.in_edit_mode.set(!model.in_edit_mode.get_copy());
            }
            Msg::ToggleAddInputMode => {
                model.in_add_input_mode.set(!model.in_add_input_mode.get_copy());
            }
            Msg::SubmitNewInput => {
                submit_new_input(model, sh);
            }
            Msg::DeleteInput(uid) => {
                delete_input(model, sh, uid);
            }
            Msg::InputType(input_type) => {
                model.input_type.set(input_type);
            }
            Msg::Name(name) => {
                model.name.set(name);
            }
            Msg::HttpAddress(x) => {
                model.http_address.set(x);
            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        max_width: "900px";
        width: "100%";
        margin: "0 auto";
        padding_top: "24px";
        padding_bottom: "100px";
        div !{
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
                        "Edit Inputs";
                    };
                };
                h1 !{
                    margin: "0";
                    font_size: "1.4em";
                    font_weight: "500";
                    text_align: "center";
                    "Inputs";
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
                    if &model.in_add_input_mode => {
                        box_shadow: "0px 0px 2px #0436ea";
                    };
                    event.click[] => move || Msg::ToggleAddInputMode;
                    if &model.in_add_input_mode.map(|x| !x) => {
                        i !{
                            padding: "4px 8px";
                            border_right: "1px solid";
                            border_color: "inherit";
                            class = "fas fa-plus";
                        };
                    };
                    if &model.in_add_input_mode => {
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
                        "Add Input";
                    };
                };
            };
            // BODY
            div !{
                add_input_form(self, model);
                inputs_list(self, model);
            };
        };
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

fn add_input_form(spec: &InputSpec, model: &Model) -> View<Msg> {
    let name_id = format!("id-{}", rand::random::<u32>());
    let type_http_id = format!("id-{}", rand::random::<u32>());
    let type_aws_id = format!("id-{}", rand::random::<u32>());
    let type_google_id = format!("id-{}", rand::random::<u32>());
    let http_server_id = format!("id-{}", rand::random::<u32>());
    let submit_btn = v1!{
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
                  animation: "1s linear infinite";
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
                Msg::SubmitNewInput
            };
            type = "submit";
            value = "Submit";
        };
    };
    let http_input = v1!{
        display: "flex";
        flex_direction: "column";
        width: "100%";
        margin_bottom: "8px";
        label !{
            font_size: "1em";
            for = &http_server_id;
            "Server Address";
        };
        input !{
            text_theme();
            width: "100%";
            font_size: "1em";
            outline: "none";
            css.placeholder => s1!{
                color: "#b9b9b9";
            };
            id = &http_server_id;
            placeholder = "https://example.com/optional-mount-path";
            type = "text";
            value = &model.http_address;
            event.input[] => move |str| Msg::HttpAddress(str);
        };
        render_checks(&model.http_address_checks);
    };
    let aws_s3_input = v1!{
        h2 !{
            text_theme();
            color: "#ccc !important";
            font_size: "2em !important";
            font_weight: "600 !important";
            margin: "0";
            "Not yet supported";
        };
    };
    let google_storage_input = v1!{
        h2 !{
            text_theme();
            color: "#ccc !important";
            font_size: "2em !important";
            font_weight: "600 !important";
            margin: "0";
            "Not yet supported";
        };
    };
    v1!{
        if &model.in_add_input_mode => {
            form !{
                text_theme();
                display: "flex";
                flex_direction: "column";
                border_bottom: "1px solid #c3c3c3";
                padding: "8px";
                // NAME
                div !{
                    display: "flex";
                    flex_direction: "column";
                    width: "100%";
                    margin_bottom: "8px";
                    label !{
                        font_size: "1em";
                        user_select: "none";
                        for = &name_id;
                        "Name";
                    };
                    input !{
                        text_theme();
                        font_size: "1em";
                        outline: "none";
                        type = "text";
                        id = &name_id;
                        if &Signal::new(spec.session.account.inputs.is_empty()) => {
                            cursor: "not-allowed";
                            color: "#858e96 !important";
                            font_weight: "500 !important";
                            user_select: "none";
                            readonly = true;
                        };
                        if &Signal::new(!spec.session.account.inputs.is_empty()) => {
                            event.input[] => move |x| Msg::Name(x);
                        };
                        value = &model.name;
                    };
                    render_checks(&model.name_checks);
                    if &Signal::new(spec.session.account.inputs.is_empty()) => {
                        ul !{
                            padding: "0";
                            padding_left: "28px";
                            margin: "0";
                            li !{
                                padding: "0";
                                margin: "0";
                                color: "#858e96";
                                font_size: "0.9em";
                                font_weight: "500";
                                "The first input is always the account name. \
                                For subsequent inputs this field will be customizable.";
                                sup !{
                                    "(";
                                    a !{
                                        href = "#";
                                        "details";
                                    };
                                    ")";
                                };
                            };
                        };
                    };
                    if &model.name.map({
                        let account_name = spec.session.account.name.clone();
                        move |x| x == &account_name.clone()
                    }) => {
                        dl !{
                            margin: "0";
                            margin_top: "8px";
                            margin_bottom: "8px";
                            color: "#888";

                            dt !{
                                "Url Preview";
                            };
                            dd !{
                                border_left: "3px solid #f5f5f5";
                                padding_left: "8px";
                                margin_left: "0";

                                dt !{
                                    display: "flex";
                                    justify_content: "flex-start";
                                    align_items: "center";
                                    margin_bottom: "2px";
                                    "Account Alias";
                                };
                                dd !{
                                    padding: "4px";
                                    background_color: "#f5f5f5";
                                    margin_left: "0";
                                    border_radius: "3px";
                                    margin_bottom: "4px";
                                    "account.logo.media/url-path";
                                };
                                dt !{
                                    margin_bottom: "2px";
                                    "Canonical";
                                };
                                dd !{
                                    padding: "4px";
                                    background_color: "#f5f5f5";
                                    margin_left: "0";
                                    border_radius: "3px";
                                    margin_bottom: "4px";
                                    "logo.media/account/input/url-path";
                                };
                            };
                        };
                    };
                    if &model.name.map({
                        let account_name = spec.session.account.name.clone();
                        let name_checks = model.name_checks.clone();
                        move |x| {
                            (!x.is_empty() && x != &account_name.clone()) &&
                            all_valid(&name_checks)
                        }
                    }) => {
                        dl !{
                            margin: "0";
                            margin_top: "8px";
                            margin_bottom: "8px";
                            color: "#888";

                            dt !{
                                "Url Preview";
                            };
                            dd !{
                                border_left: "3px solid #f5f5f5";
                                padding_left: "8px";
                                margin_left: "0";
                                dt !{
                                    margin_bottom: "2px";
                                    "Canonical";
                                };
                                dd !{
                                    padding: "4px";
                                    background_color: "#f5f5f5";
                                    margin_left: "0";
                                    border_radius: "3px";
                                    margin_bottom: "4px";
                                    model.name.map({
                                        let account_name = spec.session.account.name.clone();
                                        move |x| format!(
                                            "logo.media/{}/{}/url-path",
                                            account_name.clone(),
                                            x,
                                        )
                                    });
                                };
                            };
                        };
                    };
                };
                // TYPE
                fieldset !{
                    display: "flex";
                    flex_direction: "column";
                    width: "100%";
                    margin_bottom: "8px";
                    border: "1px solid #dfdfdf";
                    legend !{
                        "Input Type";
                    };
                    div !{
                        display: "flex";
                        width: "100%";
                        margin_bottom: "8px";
                        input !{
                            font_size: "1em";
                            event.click[] => || Msg::InputType(InputType::Http);
                            id = &type_http_id;
                            name = "input-type";
                            type = "radio";
                            checked = model.input_type.map(|x| x == &InputType::Http);
                        };
                        label !{
                            font_size: "1em";
                            width: "100%";
                            for = &type_http_id;
                            "HTTP";
                        };
                    };
                    div !{
                        display: "flex";
                        width: "100%";
                        margin_bottom: "8px";
                        input !{
                            font_size: "1em";
                            event.click[] => || Msg::InputType(InputType::AwsS3);
                            id = &type_aws_id;
                            name = "input-type";
                            type = "radio";
                            checked = model.input_type.map(|x| x == &InputType::AwsS3);
                        };
                        label !{
                            font_size: "1em";
                            width: "100%";
                            for = &type_aws_id;
                            "AWS S3";
                        };
                    };
                    div !{
                        display: "flex";
                        width: "100%";
                        margin_bottom: "8px";
                        input !{
                            font_size: "1em";
                            event.click[] => || Msg::InputType(InputType::GoogleStorage);
                            id = &type_google_id;
                            name = "input-type";
                            type = "radio";
                            checked = model.input_type.map(|x| x == &InputType::GoogleStorage);
                        };
                        label !{
                            font_size: "1em";
                            width: "100%";
                            for = &type_google_id;
                            "Google Storage";
                        };
                    };
                };
                // TYPE-INFO
                if &model.input_type.map(|x| x == &InputType::Http) => {
                    fieldset !{
                        display: "flex";
                        flex_direction: "column";
                        width: "100%";
                        margin_bottom: "8px";
                        border: "1px solid #dfdfdf";
                        legend !{
                            "HTTP Configuration";
                        };
                        http_input;
                    };
                };
                if &model.input_type.map(|x| x == &InputType::AwsS3) => {
                    fieldset !{
                        display: "flex";
                        flex_direction: "column";
                        width: "100%";
                        margin_bottom: "8px";
                        border: "1px solid #dfdfdf";
                        legend !{
                            "AWS S3 Configuration";
                        };
                        aws_s3_input;
                    };
                };
                if &model.input_type.map(|x| x == &InputType::GoogleStorage) => {
                    fieldset !{
                        display: "flex";
                        flex_direction: "column";
                        width: "100%";
                        margin_bottom: "8px";
                        border: "1px solid #dfdfdf";
                        legend !{
                            "Google Storage Configuration";
                        };
                        google_storage_input;
                    };
                };
                // SUBMIT
                submit_btn;
            };
        };
    }
}

fn inputs_list(spec: &InputSpec, model: &Model) -> View<Msg> {
    let input_entry = |input: &Input| match &input.driver {
        InputDriver::Http{address, ..} => {
            let dest_address = format!(
                "logo.media/{}/{}",
                spec.session.account.name.clone(),
                &input.name,
            );
            v1!{
                tr !{
                    border_bottom: "1px solid #c3c3c3";
                    css.hover => s1!{
                        background_color: "#f0f9ff";
                    };
                    css.last_child => s1!{
                        border_bottom: "none !important";
                    };
                    if &model.in_edit_mode => {
                        td !{
                            font_weight: "800";
                            color: "#777";
                            border_right: "1px solid #c3c3c3";
                            display: "table-cell";
                            outline: "none";
                            transition: "box-shadow 0.5s";
                            padding: "0 12px";
                            position: "relative";
                            text_align: "center";
                            css.hover => s1!{
                                box_shadow: "0px 0px 2px #ea0404";
                                z_index: "1";
                            };
                            event.click[id@input.id] => move || Msg::DeleteInput(id);
                            i !{class = "fas fa-trash-alt";};
                        };
                    };
                    td !{
                        padding: "8px";
                        font_weight: "800";
                        color: "#777";
                        border_right: "1px solid #c3c3c3";
                        text_align: "center";
                        "HTTP";
                    };
                    td !{
                        padding: "0";
                        padding_left: "8px";
                        padding_right: "8px";
                        border_right: "1px solid #c3c3c3";
                        &input.name;
                    };
                    td !{
                        padding: "0";
                        padding_left: "8px";
                        padding_right: "8px";
                        const if (dest_address.len() >= 50) => {
                            font_size: "0.8em";
                        };
                        &dest_address;
                    };
                    td !{
                        i !{
                            class = "fas fa-long-arrow-alt-right";
                        };
                    };
                    td !{
                        padding: "0";
                        padding_left: "8px";
                        padding_right: "8px";
                        const if (address.len() >= 50) => {
                            font_size: "0.8em";
                        };
                        address;
                    };
                };
            }
        },
        InputDriver::AwsS3{..} => v1!{

        },
        InputDriver::GoogleStorage{..} => v1!{

        },
    };
    if spec.session.account.inputs.is_empty() {v1!{
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
    }}
    else {v1!{
        table !{
            list_style: "none";
            padding: "0";
            margin: "0";
            width: "100%";
            border_collapse: "collapse";
            cellspacing = "0";
            spec.session.account.inputs
                .values()
                .map(|x| input_entry(x))
                .collect::<Vec<_>>();
        };
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VALIDATION VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

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
