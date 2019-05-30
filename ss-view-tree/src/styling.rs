pub mod syntax;
pub mod selectors;

use std::collections::*;
pub use ss_css_properties::data::{Style, Untyped};
use crate::{Mixin, Viewable};


///////////////////////////////////////////////////////////////////////////////
// STYLESHEET
///////////////////////////////////////////////////////////////////////////////

pub type CssId = u32;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stylesheet {
	local: HashSet<Style>,
	media: HashSet<MediaQuerySelector>,
	state: HashSet<StateSelector>,
}

impl Stylesheet {
	pub fn is_empty(&self) -> bool {
		self.local.is_empty() &&
		self.media.is_empty() &&
		self.state.is_empty()
	}
}

///////////////////////////////////////////////////////////////////////////////
// STYLESHEET FIELDS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MediaQuerySelector {
    selector: Vec<Style>,
    body: Vec<Style>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StateSelector {
	selector: StateSelectorType,
	body: Vec<Style>,
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
    fn mixin<'a>(self, mixin: Mixin<'a, Msg>) {
    	mixin.styling.local.insert(self);
    }
}
impl<Msg> Viewable<Msg> for MediaQuerySelector {
    fn mixin<'a>(self, mixin: Mixin<'a, Msg>) {
    	mixin.styling.media.insert(self);
    }
}
impl<Msg> Viewable<Msg> for StateSelector {
    fn mixin<'a>(self, mixin: Mixin<'a, Msg>) {
    	mixin.styling.state.insert(self);
    }
}

