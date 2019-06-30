use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};
use subscript::prelude::*;

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
        let model = Model {name, ..Default::default()};
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

                }
                InputType::AwsS3 => {

                }
                InputType::GoogleStorage => {

                }
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
            event.input[] => move |str| Msg::HttpAddress(str);
        };
    };
    pub fn alt_text_theme<Msg: 'static>() -> View<Msg> {v1!{
        font_family: "'Source Sans Pro', sans-serif";
        color: "#ccc";
        font_weight: "800";
    }}
    let aws_s3_input = v1!{
        h2 !{
            text_theme();
            // font_size: "2em";
            // margin: "0";
            // font_weight: "600";
            font_family: "'Source Sans Pro', sans-serif";
            color: "#000";
            font_weight: "800";
            "Not yet supported";
        };
    };
    let google_storage_input = v1!{
        h2 !{
            text_theme();
            // font_size: "2em";
            // margin: "0";
            // font_weight: "600";
            font_family: "'Source Sans Pro', sans-serif";
            color: "#000";
            font_weight: "800";
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
                        value = &model.name;
                    };
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
                        move |x| x != &account_name.clone()
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
                                    model.name.map(|x| format!("logo.media/test/{}/url-path", x));
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
    // let input_entry = |name: &str| v1!{
    //     li !{
    //         span !{
    //             name;
    //         };
    //     };
    // };
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
        // ul !{
        //     height: "100px";
        //     list_style: "none";
        //     padding: "0";
        //     margin: "0";
        //     // input_entry("master");
        // };
    }}
}

