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
use crate::process::data::*;
use crate::tree::offline::data::*;
use crate::tree::offline::api::*;
use crate::tree::online::data::*;
use crate::dev::server::data::*;
use crate::dev::client::data::*;
use crate::dev::client::utils;



///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AnalyticsSpec {
    
}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Model {
    
}

impl Default for Model {
    fn default() -> Self {
        Model {
            
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl Spec for AnalyticsSpec {
    type Model = Model;
    type Msg = Msg;
    
    fn init(&self, loaded: InitArgs<Self::Model>) -> Init<Self::Model, Self::Msg> {
        Init {
            model: match loaded.saved_model {
                Some(saved_model) => saved_model,
                None => Default::default(),
            },
            subs: subscriptions!()
        }
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd) {
        match msg {
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
                form_field(),
                form_field(),
                form_submit(),
            ])
        ));
        
        
        let create_account = panel("Create Account", markup!(form|
            border_radius: "3px"
            self.append(&[
                form_field(),
                form_field(),
                form_field(),
                form_submit()
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



fn form_field() -> Html<Msg> {
    let ref input_id: String = format!("{}", rand::random::<u16>());
    markup!(
        margin_bottom: "18px"
        label(
            text_transform: "uppercase"
            font_family: "'Source Sans Pro', sans-serif"
            font_size: "1em"
            color: "#656565"
            for = {input_id}
            text("test")
        )
        input(
            .input(move |event| {
                Msg::NoOp
            })
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
            font_size: "1.1em"
            padding: "2px"
            padding_left: "6px"
            id = {input_id}
            type = "text"
            text("")
        )
    )
}

fn form_submit() -> Html<Msg> {
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
                Msg::NoOp
            })
            text("Submit")
        )
    )
}

