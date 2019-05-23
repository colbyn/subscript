pub mod syntax;

use crate::rules::*;
use crate::selectors::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Style(pub(crate) Rule);

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
	pub fn add_style(&mut self, x: Style) {
		unimplemented!()
	}
	pub fn union(&mut self, other: Stylesheet) {
		unimplemented!()
	}
}



