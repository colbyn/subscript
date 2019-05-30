use crate::styling::*;

/// The '@media' at-rule.
pub fn media<Msg>(selector: Vec<Style>, body: Vec<Style>) -> impl Viewable<Msg> {
	MediaQuerySelector {selector, body}
}
/// The 'active' pseudo selector.
pub fn active<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Active, body}
}
/// The 'after' pseudo selector.
pub fn after<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::After, body}
}
/// The 'before' pseudo selector.
pub fn before<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Before, body}
}
/// The 'checked' pseudo selector.
pub fn checked<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Checked, body}
}
/// The 'disabled' pseudo selector.
pub fn disabled<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Disabled, body}
}
/// The 'empty' pseudo selector.
pub fn empty<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Empty, body}
}
/// The 'enabled' pseudo selector.
pub fn enabled<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Enabled, body}
}
/// The 'first-child' pseudo selector.
pub fn first_child<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::FirstChild, body}
}
/// The 'first-letter' pseudo selector.
pub fn first_letter<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::FirstLetter, body}
}
/// The 'first-line' pseudo selector.
pub fn first_line<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::FirstLine, body}
}
/// The 'focus' pseudo selector.
pub fn focus<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Focus, body}
}
/// The 'hover' pseudo selector.
pub fn hover<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Hover, body}
}
/// The 'last-child' pseudo selector.
pub fn last_child<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::LastChild, body}
}
/// The 'only-child' pseudo selector.
pub fn only_child<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::OnlyChild, body}
}
/// The 'link' pseudo selector.
pub fn link<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Link, body}
}
/// The 'visited' pseudo selector.
pub fn visited<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Visited, body}
}
/// The 'spelling-error' pseudo selector.
pub fn spelling_error<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::SpellingError, body}
}
/// The 'grammar-error' pseudo selector.
pub fn grammar_error<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::GrammarError, body}
}
/// The 'selection' pseudo selector.
pub fn selection<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Selection, body}
}
/// The 'placeholder' pseudo selector.
pub fn placeholder<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Placeholder, body}
}
/// The 'marker' pseudo selector.
pub fn marker<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Marker, body}
}
/// The 'cue' pseudo selector.
pub fn cue<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Cue, body}
}
/// The 'backdrop' pseudo selector.
pub fn backdrop<Msg>(body: Vec<Style>) -> impl Viewable<Msg> {
	StateSelector {selector: StateSelectorType::Backdrop, body}
}