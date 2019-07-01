pub mod billing;
pub mod password;
pub mod users;

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
pub struct AccountSpec {
    pub session: Session,
    pub page: AccountPage,
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

impl Spec for AccountSpec {
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

fn navigation(account: &AccountSpec, model: &Model) -> View<Msg> {
    let link = |page: AccountPage, text: &str| v1!{
        li !{
            display: "block";
            border_bottom: "1px solid";
            border_color: "#c3c3c3";
            css.last_child => s1!{
                border_bottom: "none !important";
            };
            {
                if account.page == page {v1!{
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
                    Msg::UrlRequest(Page::Account(page))
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
                    "Personal Settings";
                };
                ul !{
                    list_style: "none";
                    padding: "0";
                    margin: "0";
                    link(AccountPage::Password, "Password");
                };
            };
            section !{
                background_color: "#fff";
                overflow: "hidden";
                border: "1px solid";
                border_color: "inherit";
                margin: "8px";
                border_radius: "3px";
                h3 !{
                    text_theme();
                    margin: "0";
                    padding: "8px";
                    border_bottom: "1px solid";
                    border_color: "inherit";
                    background_color: "#f6f6f7";
                    font_weight: "400";
                    font_size: "1em";
                    "Organization Settings";
                };
                ul !{
                    list_style: "none";
                    padding: "0";
                    margin: "0";
                    link(AccountPage::Users(UsersPage::default()), "Users");
                    link(AccountPage::Billing, "Billing");
                };
            };
        };
    }
}

fn page(account: &AccountSpec, model: &Model) -> View<Msg> {
    match &account.page {
        AccountPage::Password => v1!{
            Component::singleton(PasswordSpec{
                session: account.session.clone(),
            });
        },
        AccountPage::Users(users_page) => v1!{
            Component::singleton(UsersSpec{
                session: account.session.clone(),
                page: users_page.clone(),
            });
        },
        AccountPage::Billing => v1!{
            Component::singleton(BillingSpec{
                session: account.session.clone(),
            });
        },
    }
}
