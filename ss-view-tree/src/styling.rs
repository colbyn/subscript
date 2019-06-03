pub mod syntax;
pub mod selectors;

use std::collections::*;
pub use ss_css_properties::data::{Style, Untyped};
use crate::{Env, Viewable};


///////////////////////////////////////////////////////////////////////////////
// STYLESHEET
///////////////////////////////////////////////////////////////////////////////

pub type CssId = u64;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct Stylesheet {
	local: Vec<Style>,
	media: Vec<MediaQuerySelector>,
	keyframes: Vec<KeyframeSelector>,
	state: Vec<StateSelector>,
}

impl Stylesheet {
	pub fn is_empty(&self) -> bool {
		self.local.is_empty() &&
		self.media.is_empty() &&
		self.state.is_empty() &&
		self.keyframes.is_empty()
	}
	pub fn merge(&mut self, other: Stylesheet) {
		self.local.extend(other.local);
		self.media.extend(other.media);
		self.state.extend(other.state);
		self.keyframes.extend(other.keyframes);
	}
}

///////////////////////////////////////////////////////////////////////////////
// STYLESHEET FIELDS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct KeyframeSelector(pub(crate) Vec<KeyframeInterval>);
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct KeyframeInterval {
	pub(crate) value: String,
	pub(crate) body: Vec<Style>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MediaQuerySelector {
    pub(crate) selector: Vec<Style>,
    pub(crate) body: Vec<Style>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StateSelector {
	pub(crate) selector: StateSelectorType,
	pub(crate) body: Vec<Style>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum StateSelectorType {
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
// VIEWABLE INSTANCES
///////////////////////////////////////////////////////////////////////////////

impl<Msg> Viewable<Msg> for Style {
    fn extend<'a>(self, env: Env<'a, Msg>) {
    	env.styling.local.push(self);
    }
}
impl<Msg> Viewable<Msg> for MediaQuerySelector {
    fn extend<'a>(self, env: Env<'a, Msg>) {
    	env.styling.media.push(self);
    }
}
impl<Msg> Viewable<Msg> for StateSelector {
    fn extend<'a>(self, env: Env<'a, Msg>) {
    	env.styling.state.push(self);
    }
}
impl<Msg> Viewable<Msg> for KeyframeSelector {
    fn extend<'a>(self, env: Env<'a, Msg>) {
    	env.styling.keyframes.push(self);
    }
}

