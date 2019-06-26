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
pub struct UsersSpec {
    pub session: Session,
    pub page: UsersPage,
}

pub enum Msg {
    NoOp,
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

impl Spec for UsersSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, startup: &Shell<Self>) -> Init<Self> {
        Default::default()
    }
    fn update(&self, model: &mut Model, msg: Msg, sh: &mut Shell<Self>) {
        match msg {
            Msg::NoOp => {}
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
                i !{
                    padding: "4px 8px";
                    border_right: "1px solid";
                    border_color: "inherit";
                    class = "fas fa-lock";
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
                i !{
                    padding: "4px 8px";
                    border_right: "1px solid";
                    border_color: "inherit";
                    class = "fas fa-plus";
                };
                span !{
                    padding: "0 8px";
                    font_weight: "400";
                    font_size: "1.1em";
                    "Add User";
                };
            };
        };
        if &Signal::new(self.session.account.account_users.is_empty()) => {
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
        if &Signal::new(!self.session.account.account_users.is_empty()) => {

        };
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

