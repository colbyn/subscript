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
pub struct BillingSpec {
    pub session: Reactive<Option<Session>>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    NoOp,
    Session(Option<Session>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Model {
    session: Option<Session>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            session: None,
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// APP SPECIFICATION - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl Spec for BillingSpec {
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
        match msg {
            Msg::NoOp => (),
            Msg::Session(session) => {
                model.session = session;
            }
        }
    }
    fn view(&self, model: &Self::Model) -> Html<Self::Msg> {
        if model.session.is_none() {
            html!()
        } else {
            billing(model)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// APP VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn billing(model: &Model) -> Html<Msg> {
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
            text("Billing overview")
        )
        hr(
            border: "1px solid #eaeaea"
        )
        self.add(billing_overview(model))
        h1(
            color: "#5a5a5a"
            font_family: "'Source Sans Pro', sans-serif"
            margin: "0"
            margin_top: "28px"
            padding: "0"
            padding_bottom: "4px"
            text("Your subscriptions")
        )
        hr(
            border: "1px solid #eaeaea"
        )
        self.add(your_subscriptions(model))
        h1(
            color: "#5a5a5a"
            font_family: "'Source Sans Pro', sans-serif"
            margin: "0"
            margin_top: "28px"
            padding: "0"
            padding_bottom: "4px"
            text("Payment information")
        )
        hr(
            border: "1px solid #eaeaea"
        )
        self.add(payment_information(model))
    )
}


pub fn billing_overview(model: &Model) -> Html<Msg> {
    html!(
        display: "grid"
        grid_template_columns: "1fr 1fr 1fr"
        grid_column_gap: "20px"
        
        // CURRENT PAYMENT
        div(
            display: "flex"
            flex_direction: "column"
            padding: "30px 10px"
            padding_top: "24px"
            align_items: "center"
            justify_content: "center"
            background_color: "#f9f9f9"
            color: "#5a5a5a"
            font_family: "'Source Sans Pro', sans-serif"
            h4(
                margin: "0"
                font_weight: "400"
                text("Current bill total")
            )
            span(
                font_size: "1.7em"
                font_weight: "500"
                padding_top: "2px"
                padding_bottom: "2px"
                text("$0")
            )
            a(
                :hover(
                    border_bottom: "1px solid #000"
                )
                border_bottom: "1px solid #0000" // TRANSPARENT
                text_transform: "uppercase"
                font_size: "0.8em"
                text("Change to yearly billing")
            )
        )
        // NEXT PAYMENT
        div(
            display: "flex"
            flex_direction: "column"
            padding: "30px 10px"
            padding_top: "24px"
            align_items: "center"
            justify_content: "center"
            background_color: "#f9f9f9"
            color: "#5a5a5a"
            font_family: "'Source Sans Pro', sans-serif"
            h4(
                margin: "0"
                font_weight: "400"
                text("Next payment due")
            )
            span(
                font_size: "1.7em"
                font_weight: "500"
                padding_top: "2px"
                padding_bottom: "2px"
                text("--")
            )
            a(
                :hover(
                    border_bottom: "1px solid #000"
                )
                border_bottom: "1px solid #0000" // TRANSPARENT
                outline: "none"
                margin_top: "2px"
                margin_bottom: "2px"
                text_transform: "uppercase"
                font_size: "0.8em"
                text("Past payments")
            )
        )
        // QUICK ACTIONS
        div(
            display: "flex"
            flex_direction: "column"
            padding: "30px 10px"
            padding_top: "24px"
            align_items: "center"
            justify_content: "center"
            background_color: "#f9f9f9"
            color: "#5a5a5a"
            font_family: "'Source Sans Pro', sans-serif"
            h4(
                margin: "0"
                font_weight: "400"
                text("Quick actions")
            )
            a(
                :hover(
                    border_bottom: "1px solid #000"
                )
                border_bottom: "1px solid #0000" // TRANSPARENT
                margin_top: "2px"
                margin_bottom: "2px"
                outline: "none"
                text_transform: "uppercase"
                font_size: "0.8em"
                text("Update payment method")
            )
            a(
                :hover(
                    border_bottom: "1px solid #000"
                )
                border_bottom: "1px solid #0000" // TRANSPARENT
                outline: "none"
                text_transform: "uppercase"
                font_size: "0.8em"
                text("View payment history")
            )
            a(
                :hover(
                    border_bottom: "1px solid #000"
                )
                border_bottom: "1px solid #0000" // TRANSPARENT
                margin_top: "2px"
                margin_bottom: "2px"
                outline: "none"
                text_transform: "uppercase"
                font_size: "0.8em"
                text("Explore subscription options")
            )
        )
    )
}

pub fn your_subscriptions(model: &Model) -> Html<Msg> {
    html!(
        color: "#5a5a5a"
        font_family: "'Source Sans Pro', sans-serif"
        div(
            display: "flex"
            align_items: "center"
            background_color: "#f9f9f9"
            padding: "8px"
            margin_bottom: "8px"
            h4(
                margin: "0"
                text("LOGO.IO Free")
            )
            p(
                margin: "0"
                text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
            )
        )
        div(
            display: "flex"
            align_items: "center"
            background_color: "#f9f9f9"
            padding: "8px"
            margin_bottom: "8px"
            h4(
                margin: "0"
                text("LOGO.IO Independent")
            )
            p(
                margin: "0"
                text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
            )
        )
        div(
            display: "flex"
            align_items: "center"
            background_color: "#f9f9f9"
            padding: "8px"
            h4(
                margin: "0"
                text("LOGO.IO Enterprise")
            )
            p(
                margin: "0"
                text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
            )
        )
    )
}


pub fn payment_information(model: &Model) -> Html<Msg> {
    html!(
        display: "grid"
        grid_template_columns: "1fr 1fr 1fr"
        grid_column_gap: "20px"
        
        // PAYMENT METHOD
        div(
            display: "flex"
            flex_direction: "column"
            padding: "30px 10px"
            padding_top: "24px"
            align_items: "center"
            justify_content: "center"
            background_color: "#f9f9f9"
            color: "#5a5a5a"
            font_family: "'Source Sans Pro', sans-serif"
            text_align: "center"
            h4(
                margin: "0"
                font_weight: "400"
                text("Payment method")
            )
            p(
                text("You have not added a payment method")
            )
            a(
                :hover(
                    border_bottom: "1px solid #000"
                )
                border_bottom: "1px solid #0000" // TRANSPARENT
                text_transform: "uppercase"
                font_size: "0.8em"
                text("Add payment method")
            )
        )
        // LAST PAYMENT
        div(
            display: "flex"
            flex_direction: "column"
            padding: "30px 10px"
            padding_top: "24px"
            align_items: "center"
            justify_content: "center"
            background_color: "#f9f9f9"
            color: "#5a5a5a"
            font_family: "'Source Sans Pro', sans-serif"
            text_align: "center"
            h4(
                margin: "0"
                font_weight: "400"
                text("Last payment")
            )
            p(
                text("You have not made any payments.")
            )
        )
        // COUPON
        div(
            display: "flex"
            flex_direction: "column"
            padding: "30px 10px"
            padding_top: "24px"
            align_items: "center"
            justify_content: "center"
            background_color: "#f9f9f9"
            color: "#5a5a5a"
            font_family: "'Source Sans Pro', sans-serif"
            text_align: "center"
            h4(
                margin: "0"
                font_weight: "400"
                text("Coupon")
            )
            p(
                text("You don’t have an active coupon.")
            )
            a(
                :hover(
                    border_bottom: "1px solid #000"
                )
                border_bottom: "1px solid #0000" // TRANSPARENT
                text_transform: "uppercase"
                font_size: "0.8em"
                text("Redeem a coupon")
            )
        )
    )
}

