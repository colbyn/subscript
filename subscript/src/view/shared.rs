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
// STYLING - DATA
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Default)]
pub struct Styling {
    pub(crate) default: StyleList,
    pub(crate) state: Vec<StateSelector>,
    pub(crate) animations: Vec<Animation>,
    pub(crate) media: Vec<MediaCondition>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Default)]
pub struct StyleList(pub(crate) Vec<Style>);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub(crate) struct Style {
    pub property: String,
    pub value: String,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Animation(pub(crate) Vec<AnimationInterval>);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct AnimationInterval {
    pub(crate) value: String,
    pub(crate) style: StyleList,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct MediaCondition {
    pub(crate) condition: StyleList,
    pub(crate) body: StyleList,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct StateSelector {
    pub(crate) name: StateSelectorName,
    pub(crate) body: StyleList,
}

impl StateSelector {
    pub fn new_active(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Active, body}
    }
    pub fn new_after(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::After, body}
    }
    pub fn new_before(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Before, body}
    }
    pub fn new_checked(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Checked, body}
    }
    pub fn new_disabled(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Disabled, body}
    }
    pub fn new_empty(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Empty, body}
    }
    pub fn new_enabled(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Enabled, body}
    }
    pub fn new_firstchild(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::FirstChild, body}
    }
    pub fn new_firstletter(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::FirstLetter, body}
    }
    pub fn new_firstline(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::FirstLine, body}
    }
    pub fn new_focus(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Focus, body}
    }
    pub fn new_hover(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Hover, body}
    }
    pub fn new_lastchild(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::LastChild, body}
    }
    pub fn new_onlychild(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::OnlyChild, body}
    }
    pub fn new_link(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Link, body}
    }
    pub fn new_visited(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Visited, body}
    }
    pub fn new_spellingerror(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::SpellingError, body}
    }
    pub fn new_grammarerror(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::GrammarError, body}
    }
    pub fn new_selection(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Selection, body}
    }
    pub fn new_placeholder(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Placeholder, body}
    }
    pub fn new_marker(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Marker, body}
    }
    pub fn new_cue(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Cue, body}
    }
    pub fn new_backdrop(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Backdrop, body}
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub(crate) enum StateSelectorName {
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


///////////////////////////////////////////////////////////////////////////////
// STYLING - API
///////////////////////////////////////////////////////////////////////////////

impl Styling {
    pub fn is_empty(&self) -> bool {
        self.default.0.is_empty() &&
        self.state.is_empty() &&
        self.animations.is_empty() &&
        self.media.is_empty()
    }
    pub fn extend(&mut self, new: Styling) {
        self.default.0.extend(new.default.0);
        self.state.extend(new.state);
        self.animations.extend(new.animations);
        self.media.extend(new.media);
    }
}

impl Style {
    pub fn new(property: &str, value: &str) -> Self {
        let property = String::from(property);
        let value = String::from(value);
        Style{property, value}
    }
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
// ATTRIBUTES
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


