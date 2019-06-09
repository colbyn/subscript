use std::collections::*;

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi};

pub enum GlobalCssRegistry {
	Pending,
	Live(LiveCssRegistry)
}

pub struct LiveCssRegistry {
	mount: CssMount,
	added: HashSet<u64>,
}

pub struct CssMount {
	wrapper: browser::Element,
	local: browser::Element,
	state: browser::Element,
	media: browser::Element,
}


