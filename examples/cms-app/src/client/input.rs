use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};
use subscript::prelude::*;

use crate::client::AppSpec;
use crate::client::data::*;
use crate::client::ui_utils::{self, text_theme};
use crate::client::account::billing::BillingSpec;
use crate::client::account::password::PasswordSpec;
use crate::client::account::users::UsersSpec;


///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct InputSpec {
    pub session: Session,
}

pub enum Msg {
    NoOp,
    UrlRequest(Page)
}

#[derive(Default)]
pub struct Model {

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
        Default::default()
    }
    fn update(&self, model: &mut Model, msg: Msg, sh: &mut Shell<Self>) {
        match msg {
            Msg::NoOp => {}
            Msg::UrlRequest(page) => {
                sh.message::<AppSpec, _>(UrlRequest(page));
            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        max_width: "900px";
        width: "100%";
        margin: "0 auto";
        padding_top: "24px";
        overview();
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

fn overview() -> View<Msg> {v1!{
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
            justify_content: "center";
            align_items: "center";
            overflow: "hidden";

            h1 !{
                margin: "0";
                font_size: "1.4em";
                font_weight: "500";
                text_align: "center";
                "Inputs";
            };
        };
        div !{
            height: "100px";
            min_height: "100px";
        };
    };
}}
