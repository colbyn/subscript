pub mod syntax;

use crate::rules::*;
use crate::selectors::*;

pub type CssHashKey = u64;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Style {
	Native(Rule),
	Raw {
		property: String,
		value: String,
	}
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Stylesheet {
	local: Vec<Style>
}

impl Default for Stylesheet {
	fn default() -> Self {
		Stylesheet {
			local: Vec::new(),
		}
	}
}


impl Stylesheet {
	pub fn is_empty(&self) -> bool {
		self.local.is_empty()
	}
	pub fn add_style(&mut self, x: Style) {
		self.local.push(x);
	}
	pub fn union(&mut self, other: Stylesheet) {
		let mut other = other;
		self.local.append(&mut other.local);
	}
}



