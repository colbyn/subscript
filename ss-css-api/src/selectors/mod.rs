pub mod syntax;

use crate::rules::Rule;


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MediaQuerySelector {
    selector: Vec<Rule>,
    body: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StateSelector {
	selector: StateSelectorType,
	body: Vec<Rule>,
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
