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
pub struct BillingSpec {
    pub session: Session,
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

impl Spec for BillingSpec {
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
        width: "100%";
        height: "100%";
        justify_content: "flex-start";
        align_items: "flex-start";
        display: "grid";
        grid_template_columns: "1fr";
        grid_auto_rows: "max-content";
        grid_row_gap: "10px";

        overview(model);
        subscriptions(model);
        payment(model);
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

fn overview(model: &Model) -> View<Msg> {v1!{
    div !{
        border: "1px solid #c3c3c3";
        background_color: "#fff";
        border_radius: "3px";
        width: "100%";
        height: "fit-content";
        header !{
            border_top_left_radius: "3px";
            border_top_right_radius: "3px";
            border_bottom: "1px solid #c3c3c3";
            background_color: "#f6f6f7";
            display: "flex";
            justify_content: "space-between";
            align_items: "center";
            height: "30px";
            overflow: "hidden";

            button !{
                height: "100%";
                display: "flex";
                align_items: "center";
                justify_content: "center";
                border: "none";
                border_right: "1px solid #c3c3c3";
                background_color: "transparent";
                outline: "none";

                i !{
                    padding: "8px";
                    class = "fas fa-expand-arrows-alt";
                };
            };
            h1 !{
                margin: "0";
                font_size: "1.2em";
                font_weight: "500";
                text_align: "center";
                "Billing Overview";
            };
            div !{};
        };
        div !{
            min_height: "2px";
        };
    };
}}

fn subscriptions(model: &Model) -> View<Msg> {v1!{
    div !{
        border: "1px solid #c3c3c3";
        background_color: "#fff";
        border_radius: "3px";
        width: "100%";
        height: "fit-content";
        header !{
            border_top_left_radius: "3px";
            border_top_right_radius: "3px";
            border_bottom: "1px solid #c3c3c3";
            background_color: "#f6f6f7";
            display: "flex";
            justify_content: "space-between";
            align_items: "center";
            height: "30px";
            overflow: "hidden";

            button !{
                height: "100%";
                display: "flex";
                align_items: "center";
                justify_content: "center";
                border: "none";
                border_right: "1px solid #c3c3c3";
                background_color: "transparent";
                outline: "none";

                i !{
                    padding: "8px";
                    class = "fas fa-compress-arrows-alt";
                };
            };
            h1 !{
                margin: "0";
                font_size: "1.2em";
                font_weight: "500";
                text_align: "center";
                "Your Subscriptions";
            };
            div !{};
        };
        div !{
            min_height: "100px";
        };
    };
}}

fn payment(model: &Model) -> View<Msg> {v1!{
    div !{
        border: "1px solid #c3c3c3";
        background_color: "#fff";
        border_radius: "3px";
        width: "100%";
        height: "fit-content";
        header !{
            border_top_left_radius: "3px";
            border_top_right_radius: "3px";
            border_bottom: "1px solid #c3c3c3";
            background_color: "#f6f6f7";
            display: "flex";
            justify_content: "space-between";
            align_items: "center";
            height: "30px";
            overflow: "hidden";

            button !{
                height: "100%";
                display: "flex";
                align_items: "center";
                justify_content: "center";
                border: "none";
                border_right: "1px solid #c3c3c3";
                background_color: "transparent";
                outline: "none";

                i !{
                    padding: "8px";
                    class = "fas fa-compress-arrows-alt";
                };
            };
            h1 !{
                margin: "0";
                font_size: "1.2em";
                font_weight: "500";
                text_align: "center";
                "Payment Information";
            };
            div !{};
        };
        div !{
            min_height: "100px";
        };
    };
}}

