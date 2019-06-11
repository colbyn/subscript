use std::marker::*;
use std::any::*;
use std::rc::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use crate::view_sys::dsl::View;

pub trait Spec where Self: Clone {
	type Msg;
	type Model;
	
	fn init(&self, startup: StartupInfo<Self>) -> Init<Self>;
	fn update(&self, model: &mut Self::Model, msg: Self::Msg, sys: &SubSystems<Self>);
	fn view(&self, model: &Self::Model) -> View<Self::Msg>;
}


#[derive(Debug)]
pub struct StartupInfo<S: Spec> {
	pub name: String,
	pub saved_model: Option<S::Model>,
}

#[derive(Debug)]
pub struct Init<S: Spec> {
	pub model: S::Model,
	pub subs: Subscriptions<S::Msg>
}

#[derive(Debug, PartialEq)]
pub struct Subscriptions<Msg> {
	listener: fn(Rc<Any>)->Option<Msg>,
}

impl<Msg> Default for Subscriptions<Msg> {
	fn default() -> Self {
		Subscriptions {
			listener: |_| None,
		}
	}
}
pub struct SubSystems<S: Spec> {
    pub(crate) mark: PhantomData<S>,
}