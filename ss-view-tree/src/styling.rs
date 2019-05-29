pub mod syntax;

use std::collections::*;
pub use ss_css_properties::data::Style;
use crate::{Mixin, Viewable};

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
    	unimplemented!()
    }
}


// pub struct RenderedStylesheet {
// 	pub locals: String,
// }

// impl Stylesheet {
// 	pub fn render_css_syntax(&self, key: &CssHashKey) -> RenderedStylesheet {
// 		let mut rendered_locals: Vec<String> = Vec::new();
// 		for style in self.local.iter() {
// 			rendered_locals.push(format!("{};", style.render_css_syntax()));
// 		}
// 		RenderedStylesheet {
// 			locals: {
// 				let body = rendered_locals.join("");
// 				format!("[css=\"{key}\"] {{{body}}}", key=key, body=body)
// 			},
// 		}
// 	}
// }


// impl Style {
// 	pub fn render_css_syntax(&self) -> String {
// 		match self {
// 			Style::Native(rule) => rule.to_css_syntax(),
// 			Style::Raw{property, value} => format!(
// 				"{prop}: {value}",
// 				prop=property,
// 				value=value,
// 			),
// 		}
// 	}	
// }