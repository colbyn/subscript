pub mod billing;
pub mod email;
pub mod password;
pub mod users;

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
use crate::dev::cms_app::client::account::billing::BillingSpec;
use crate::dev::cms_app::client::account::email::EmailSpec;
use crate::dev::cms_app::client::account::password::PasswordSpec;
use crate::dev::cms_app::client::account::users::UsersSpec;


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
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {
        match &self.page {
            AccountPage::Password => v1!{
                Component::singleton(PasswordSpec{
                    session: self.session.clone(),
                });
            },
            AccountPage::Email => v1!{
                Component::singleton(EmailSpec{
                    session: self.session.clone(),
                });
            },
            AccountPage::Users(users_page) => v1!{
                Component::singleton(UsersSpec{
                    session: self.session.clone(),
                    page: users_page.clone(),
                });
            },
            AccountPage::Billing => v1!{
                Component::singleton(BillingSpec{
                    session: self.session.clone(),
                });
            },
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

