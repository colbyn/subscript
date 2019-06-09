use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use wasm_bindgen::JsValue;

use crate::model::reactive::{Signal, SignalSub, Status};
use crate::model::incremental::{IVecSub, IVec};
use crate::program::spec::Spec;

///////////////////////////////////////////////////////////////////////////////
// STYLING
///////////////////////////////////////////////////////////////////////////////

pub(crate) struct Styling {
	default: Vec<Style>,
	media: Vec<MediaQuerySelector>,
	keyframes: Vec<KeyframeSelector>,
	state: Vec<StateSelector>,
}

pub(crate) struct KeyframeSelector(pub Vec<KeyframeInterval>);
pub(crate) struct KeyframeInterval {
	pub value: String,
	pub body: Vec<Style>,
}

pub(crate) struct MediaQuerySelector {
    pub condition: Vec<Style>,
    pub body: Vec<Style>,
}

pub(crate) struct Style {
	pub property: String,
	pub value: String,
}

pub(crate) struct StateSelector {
	pub name: StateSelectorName,
	pub body: Vec<Style>,
}

pub enum StateSelectorName {
	Active,
	After,
	Before,
	Checked,
	Disabled,
	Empty,
	Enabled,
	FirstChild,
	FirstLetter,
	FirstLine,
	Focus,
	Hover,
	LastChild,
	OnlyChild,
	Link,
	Visited,
	SpellingError,
	GrammarError,
	Selection,
	Placeholder,
	Marker,
	Cue,
	Backdrop,
}

impl StateSelectorName {
	pub fn as_str(&self) -> &str {
		match self {
			StateSelectorName::Active => ":active",
			StateSelectorName::After => "::after",
			StateSelectorName::Before => "::before",
			StateSelectorName::Checked => ":checked",
			StateSelectorName::Disabled => ":disabled",
			StateSelectorName::Empty => ":empty",
			StateSelectorName::Enabled => ":enabled",
			StateSelectorName::FirstChild => ":firstchild",
			StateSelectorName::FirstLetter => "::firstletter",
			StateSelectorName::FirstLine => "::firstline",
			StateSelectorName::Focus => ":focus",
			StateSelectorName::Hover => ":hover",
			StateSelectorName::LastChild => ":lastchild",
			StateSelectorName::OnlyChild => ":onlychild",
			StateSelectorName::Link => ":link",
			StateSelectorName::Visited => ":visited",
			StateSelectorName::SpellingError => "::spellingerror",
			StateSelectorName::GrammarError => "::grammarerror",
			StateSelectorName::Selection => "::selection",
			StateSelectorName::Placeholder => "::placeholder",
			StateSelectorName::Marker => "::marker",
			StateSelectorName::Cue => "::cue",
			StateSelectorName::Backdrop => "::backdrop",
		}
	}
}

///////////////////////////////////////////////////////////////////////////////
// COMPONENTS
///////////////////////////////////////////////////////////////////////////////

pub(crate) trait SubComponentImpl {}
impl<S: Spec> SubComponentImpl for Component<S> {}


pub struct Component<S: Spec> {
	pub name: String,
	pub spec: S
}


///////////////////////////////////////////////////////////////////////////////
// PROPERTIES
///////////////////////////////////////////////////////////////////////////////
pub(crate) struct EventHandler<Msg>(pub Rc<EventHandlerImpl<Msg>>);
pub(crate) trait EventHandlerImpl<Msg> {
	fn apply(&self, event: JsValue) -> Msg;
	fn event_type(&self) -> String;
}
impl<Msg>  EventHandler<Msg> {
	pub(crate) fn apply(&self, event: JsValue) -> Msg {self.0.apply(event)}
	pub(crate) fn event_type(&self) -> String {self.0.event_type()}
}
impl<Msg> Clone for EventHandler<Msg> {
	fn clone(&self) -> Self {
		EventHandler(self.0.clone())
	}
}

#[derive(Debug)]
pub struct Attribute {
	pub key: String,
	pub value: Either<Value<String>, Value<bool>>,
}

impl<Msg> std::fmt::Debug for EventHandler<Msg> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "EventHandler")
	}
}



///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum Value<T> {
	Static(T),
	Dynamic(SignalSub<T>),
}
impl<T> Value<T> {
	pub(crate) fn if_changed(&self, f: impl Fn(&T)) {
		match &self {
			Value::Dynamic(sub) => {
				sub.if_changed(f);
			}
			Value::Static(_) => {}
		}
	}
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

