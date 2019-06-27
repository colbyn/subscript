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
pub struct InsightSpec {
    pub session: Session,
    pub page: InsightPage,
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

impl Spec for InsightSpec {
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
        display: "grid";
        grid_template_columns: "max-content 1fr";
        grid_column_gap: "20px";
        navigation(self, model);
        page(self, model);
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

fn navigation(spec: &InsightSpec, model: &Model) -> View<Msg> {
    let link = |page: InsightPage, text: &str| v1!{
        li !{
            display: "block";
            border_bottom: "1px solid";
            border_color: "#c3c3c3";
            css.last_child => s1!{
                border_bottom: "none !important";
            };
            {
                if spec.page == page {v1!{
                    border_left: "3px solid #0089ff !important";
                }}
                else {v1!{
                    padding_left: "3px";
                    css.hover => s1!{
                        background_color: "#e7edf1";
                    };
                }}
            };
            a !{
                color: "#0089ff";
                user_select: "none";
                display: "block";
                font_weight: "300";
                font_size: "0.9em";
                padding: "7px";
                padding_left: "8px";
                event.click[page] => move || {
                    Msg::UrlRequest(Page::Insight(page))
                };
                text;
            };
        };
    };
    v1!{
        aside !{
            border_color: "#c3c3c3";
            min_width: "200px";
            section !{
                background_color: "#fff";
                overflow: "hidden";
                border: "1px solid";
                border_color: "inherit";
                margin: "8px";
                border_radius: "3px";
                margin_top: "0"; // FIRST-CHILD
                h3 !{
                    text_theme();
                    margin: "0";
                    padding: "8px";
                    border_bottom: "1px solid";
                    border_color: "inherit";
                    background_color: "#f6f6f7";
                    font_weight: "400";
                    font_size: "1em";
                    "System";
                };
                ul !{
                    list_style: "none";
                    padding: "0";
                    margin: "0";
                    link(InsightPage::Overview, "Overview");
                    link(InsightPage::Health, "Health");
                };
            };
            section !{
                background_color: "#fff";
                overflow: "hidden";
                border: "1px solid";
                border_color: "inherit";
                margin: "8px";
                border_radius: "3px";
                margin_top: "0"; // FIRST-CHILD
                h3 !{
                    text_theme();
                    margin: "0";
                    padding: "8px";
                    border_bottom: "1px solid";
                    border_color: "inherit";
                    background_color: "#f6f6f7";
                    font_weight: "400";
                    font_size: "1em";
                    "Network";
                };
                ul !{
                    list_style: "none";
                    padding: "0";
                    margin: "0";
                    link(InsightPage::Traffic, "Traffic");
                    link(InsightPage::Bandwidth, "Bandwidth");
                    link(InsightPage::Cache, "Cache");
                };
            };
            section !{
                background_color: "#fff";
                overflow: "hidden";
                border: "1px solid";
                border_color: "inherit";
                margin: "8px";
                border_radius: "3px";
                margin_top: "0"; // FIRST-CHILD
                h3 !{
                    text_theme();
                    margin: "0";
                    padding: "8px";
                    border_bottom: "1px solid";
                    border_color: "inherit";
                    background_color: "#f6f6f7";
                    font_weight: "400";
                    font_size: "1em";
                    "Data";
                };
                ul !{
                    list_style: "none";
                    padding: "0";
                    margin: "0";
                    link(InsightPage::Storage, "Storage");
                };
            };
        };
    }
}

fn page(spec: &InsightSpec, model: &Model) -> View<Msg> {
    match &spec.page {
        InsightPage::Overview => overview(),
        InsightPage::Health => health(),
        InsightPage::Traffic => traffic(),
        InsightPage::Bandwidth => bandwidth(),
        InsightPage::Cache => cache(),
        InsightPage::Storage => storage(),
    }
}

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
                "Overview";
            };
        };
        div !{
            height: "100px";
            min_height: "100px";
        };
    };
}}

fn health() -> View<Msg> {v1!{
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
                "Health";
            };
        };
        div !{
            height: "100px";
            min_height: "100px";
        };
    };
}}

fn traffic() -> View<Msg> {v1!{
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
                "Traffic";
            };
        };
        div !{
            height: "100px";
            min_height: "100px";
        };
    };
}}


fn bandwidth() -> View<Msg> {v1!{
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
                "Bandwidth";
            };
        };
        div !{
            height: "100px";
            min_height: "100px";
        };
    };
}}


fn cache() -> View<Msg> {v1!{
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
                "Cache";
            };
        };
        div !{
            height: "100px";
            min_height: "100px";
        };
    };
}}


fn storage() -> View<Msg> {v1!{
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
                "Storage";
            };
        };
        div !{
            height: "100px";
            min_height: "100px";
        };
    };
}}