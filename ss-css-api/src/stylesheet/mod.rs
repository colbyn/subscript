use crate::rules::*;
use crate::selectors::*;

pub struct Style(pub(crate) Rule);

pub struct Stylesheet {
	local: Vec<Style>
}


