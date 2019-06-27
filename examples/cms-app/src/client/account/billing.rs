use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};
use subscript::prelude::*;

use crate::client::data::*;
use crate::client::ui_utils::{self, text_theme};


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
        margin_bottom: "100px";

        overview(model);
        subscriptions(model);
        payment(model);
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

fn overview(model: &Model) -> View<Msg> {
    let link = |txt: &str| v1!{
        a !{
            color: "#0089ff";
            font_size: "0.9em";
            cursor: "pointer";
            css.hover => s1!{
                text_decoration: "underline";
            };
            txt;
        };
    };
    v1!{
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
                    "Billing Overview";
                };
            };
            div !{
                display: "grid";
                grid_template_columns: "1fr 1fr 1fr";
                grid_column_gap: "10px";
                padding: "10px";
                div !{
                    display: "flex";
                    align_items: "center";
                    flex_direction: "column";
                    justify_content: "flex-start";
                    padding: "8px";
                    h3 !{
                        margin: "0";
                        "Current bill total";
                    };
                    span !{
                        padding: "2px";
                        font_size: "1.2em";
                        color: "#848383";
                        font_weight: "600";
                        "$0";
                    };
                    link("Change to yearly billing");
                };
                div !{
                    display: "flex";
                    align_items: "center";
                    flex_direction: "column";
                    justify_content: "flex-start";
                    padding: "8px";
                    h3 !{
                        margin: "0";
                        "Next payment due";
                    };
                    span !{
                        padding: "2px";
                        font_size: "1.2em";
                        color: "#848383";
                        font_weight: "600";
                        "--";
                    };
                    link("Past payments and receipts");
                };
                div !{
                    display: "flex";
                    align_items: "center";
                    flex_direction: "column";
                    justify_content: "flex-start";
                    padding: "8px";
                    h3 !{
                        margin: "0";
                        "Quick actions";
                    };
                    link("Update payment method");
                    link("View payment history");
                    link("Explore subscription options");
                };
            };
        };
    }
}

fn subscriptions(model: &Model) -> View<Msg> {
    let free_id = format!("id-{}", rand::random::<u16>());
    let independent_id = format!("id-{}", rand::random::<u16>());
    let enterprise_id = format!("id-{}", rand::random::<u16>());
    let free = || v1!{
        display: "flex";
        justify_content: "flex-start";
        align_items: "center";
        div !{
            display: "flex";
            justify_content: "center";
            align_items: "center";
            padding: "8px";
            border_right: "1px solid #c3c3c3";
            input !{
                id = &free_id;
                name = "subscription";
                type = "radio";
            };
        };
        label !{
            width: "100%";
            display: "flex";
            justify_content: "space-between";
            align_items: "center";
            padding_left: "8px";
            padding_right: "8px";
            for = &free_id;
            span !{
                font_size: "1.1em";
                margin_right: "10px";
                font_weight: "600";
                "LOGO.IO Free";
            };
            span !{
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit";
            };
        };
    };
    let independent = || v1!{
        display: "flex";
        justify_content: "flex-start";
        align_items: "center";
        div !{
            display: "flex";
            justify_content: "center";
            align_items: "center";
            padding: "8px";
            border_right: "1px solid #c3c3c3";
            input !{
                id = &independent_id;
                name = "subscription";
                type = "radio";
            };
        };
        label !{
            width: "100%";
            display: "flex";
            justify_content: "space-between";
            align_items: "center";
            padding_left: "8px";
            padding_right: "8px";
            for = &independent_id;
            span !{
                font_size: "1.1em";
                margin_right: "10px";
                font_weight: "600";
                "LOGO.IO Independent";
            };
            span !{
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit";
            };
        };
    };
    let enterprise = || v1!{
        display: "flex";
        justify_content: "flex-start";
        align_items: "center";
        div !{
            display: "flex";
            justify_content: "center";
            align_items: "center";
            padding: "8px";
            border_right: "1px solid #c3c3c3";
            input !{
                id = &enterprise_id;
                name = "subscription";
                type = "radio";
            };
        };
        label !{
            width: "100%";
            display: "flex";
            justify_content: "space-between";
            align_items: "center";
            padding_left: "8px";
            padding_right: "8px";
            for = &enterprise_id;
            span !{
                font_size: "1.1em";
                margin_right: "10px";
                font_weight: "600";
                "LOGO.IO Enterprise";
            };
            span !{
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit";
            };
        };
    };
    let submit = || v1!{
        border_top: "1px solid #c3c3c3";
        padding: "8px";
        input !{
            text_theme();
            outline: "none";
            border: "1px solid #d2d2d2";
            width: "100%";
            text_transform: "uppercase";
            font_weight: "400";
            padding: "4px";
            font_size: "1em";
            // if &model.new_user_loading.map(|x| !x) => {
            //     css.hover => s1!{
            //         box_shadow: "0 0 4px 1px #e0e0e0";
            //     };
            // };
            // if &model.new_user_loading => {
            //     background:
            //         "repeating-linear-gradient(
            //             -45deg, \
            //             hsl(0, 0%, 94%), \
            //             hsl(0, 0%, 94%) 11px, \
            //             hsl(0, 0%, 88%) 10px, \
            //             hsl(0, 0%, 88%) 20px /* determines size */ \
            //         )";
            //       background_size: "28px 28px";
            //       animation: "default-move 1s linear infinite";
            // };
            css.animation => {
                from => s1!{
                    background_position: "0 0";
                };
                to => s1!{
                    background_position: "28px 0";
                };
            };
            // event.click[] => move || {
            //     Msg::SubmitNewUser
            // };
            type = "submit";
            value = "Submit";
        };
    };
    v1!{
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
                    "Subscription";
                };
            };
            form !{
                list_style: "none";
                padding: "0";
                margin: "0";

                div !{
                    free();
                };
                div !{
                    independent();
                };
                div !{
                    enterprise();
                };
                div !{
                    submit();
                };
            };
        };
    }
}

fn payment(model: &Model) -> View<Msg> {
    let link = |txt: &str| v1!{
        a !{
            color: "#0089ff";
            font_size: "0.9em";
            cursor: "pointer";
            css.hover => s1!{
                text_decoration: "underline";
            };
            txt;
        };
    };
    v1!{
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
                    "Payment Information";
                };
            };
            div !{
                display: "grid";
                grid_template_columns: "1fr 1fr 1fr";
                grid_column_gap: "10px";
                padding: "10px";
                div !{
                    display: "flex";
                    align_items: "center";
                    flex_direction: "column";
                    justify_content: "flex-start";
                    padding: "8px";
                    h3 !{
                        margin: "0";
                        "Payment method";
                    };
                    span !{
                        padding: "2px";
                        font_size: "0.9em";
                        color: "#848383";
                        font_weight: "200";
                        text_align: "center";
                        "You have not added a payment method";
                    };
                    link("Add payment method");
                };
                div !{
                    display: "flex";
                    align_items: "center";
                    flex_direction: "column";
                    justify_content: "flex-start";
                    padding: "8px";
                    h3 !{
                        margin: "0";
                        "Last payment";
                    };
                    span !{
                        padding: "2px";
                        font_size: "0.9em";
                        color: "#848383";
                        font_weight: "200";
                        text_align: "center";
                        "You have not made any payments";
                    };
                };
                div !{
                    display: "flex";
                    align_items: "center";
                    flex_direction: "column";
                    justify_content: "flex-start";
                    padding: "8px";
                    h3 !{
                        margin: "0";
                        "Coupon";
                    };
                    span !{
                        padding: "2px";
                        font_size: "0.9em";
                        color: "#848383";
                        font_weight: "200";
                        text_align: "center";
                        "You don’t have an active coupon";
                    };
                    link("Redeem Coupon");
                };
            };
        };
    }
}

