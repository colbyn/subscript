use std::collections::*;
use std::cell::*;
use std::marker::*;
use std::any::*;
use std::rc::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::backend::browser;
use crate::view_sys::dsl::View;
use crate::program_sys::instances::TickEnv;
use crate::program_sys::shell::*;
use crate::program_sys::effect::nav::Url;

pub use crate::program_sys::shell::{Shell};
pub use crate::program_sys::effect::nav::{UrlString, UrlParser, UrlChanged};
pub use crate::program_sys::effect::sub::Subscriptions;


/// A spec is like a blueprint, that may manifest itself as either the root
/// component (a `Program`) or some child `Component`.
/// The spec’s model and message types should be considered
/// internal, and not exposed to the parent.
///
/// A child-spec may define fields that the parent-spec must fulfill like so:
/// ```
/// #[derive(Clone)]
/// pub struct AccountSpec {
///     pub session: Session,
///     pub page: AccountPage,
/// }
/// ```
/// Then from the parent-specs view:
/// ```
/// v1!{
///     Component::singleton(AccountSpec {
///         page: accunt_page.clone(),
///         session: session.clone(),
///     });
/// }
/// ```
pub trait Spec where Self: Clone {
	type Msg;
    type Model;
	
	fn init(&self, sh: &Shell<Self>) -> Init<Self>;
	fn update(&self, model: &mut Self::Model, msg: Self::Msg, sh: &mut Shell<Self>);
	fn view(&self, model: &Self::Model) -> View<Self::Msg>;
}

pub struct Init<S: Spec> {
	pub model: S::Model,
	pub subs: Subscriptions<S::Msg>,
}

impl<S: Spec> Default for Init<S>
where
    S::Model: Default,
{
    fn default() -> Self {
        Init {
            model: Default::default(),
            subs: Subscriptions::default(),
        }
    }
}





