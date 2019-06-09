use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi, CallbackSettings, QueueCallback, VoidCallback};
use crate::model::incremental::{IVecSub};
use crate::view::dsl::{self as dsl, Dsl, View, SubComponent};
use crate::view::shared::*;
use crate::view::dom::*;


///////////////////////////////////////////////////////////////////////////////
// THUNK
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub(crate) struct DomThunk<Msg>(Box<RefCell<ThunkState<Msg>>>);

// INTERNAL
#[derive(Debug)]
enum ThunkState<Msg> {
	Borrowed,
	View(View<Msg>),
	Dom(Dom<Msg>),
}

pub(crate) struct EvalDomThunk<New, Update> {
	pub new: New,
	pub update: Update,
}

impl<Msg> DomThunk<Msg> {
	pub(crate) fn into_inner(self) -> Either<View<Msg>, Dom<Msg>> {
		match self.0.into_inner() {
			ThunkState::View(view) => Left(view),
			ThunkState::Dom(dom) => Right(dom),
			ThunkState::Borrowed => {panic!()}
		}
	}
	pub(crate) fn new(x: Dom<Msg>) -> Self {
		DomThunk(Box::new(RefCell::new(ThunkState::Dom(x))))
	}
	pub(crate) fn from_view(x: View<Msg>) -> Self {
		DomThunk(Box::new(RefCell::new(ThunkState::View(x))))
	}
	pub(crate) fn inspect(&self, f: &mut FnMut(&Dom<Msg>)) {
		// let mut f = Box::new(f);
		let inner: &ThunkState<Msg> = &self.0.borrow();
		match inner {
			ThunkState::Dom(dom) => {f(dom)}
			ThunkState::View(_) => {}
			ThunkState::Borrowed => {panic!()}
		}
	}
	pub(crate) fn eval(&self, f: EvalDomThunk<impl Fn(View<Msg>)->Dom<Msg>, impl Fn(&mut Dom<Msg>)>) {
		match self.0.replace(ThunkState::Borrowed) {
			ThunkState::View(view) => {
				self.0.replace(ThunkState::Dom((f.new)(view)));
			}
			ThunkState::Dom(mut x) => {
				(f.update)(&mut x);
				self.0.replace(ThunkState::Dom(x));
			}
			ThunkState::Borrowed => panic!()
		}
	}
}
