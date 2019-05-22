use crate::core::{self, Style, StateSelector};

/// The ':active' selector.
pub fn active(styles: Vec<Style>) -> StateSelector {
    StateSelector::Active(styles)
}
/// The '::after' selector.
pub fn after(styles: Vec<Style>) -> StateSelector {
    StateSelector::After(styles)
}
/// The '::before' selector.
pub fn before(styles: Vec<Style>) -> StateSelector {
    StateSelector::Before(styles)
}
/// The ':checked' selector.
pub fn checked(styles: Vec<Style>) -> StateSelector {
    StateSelector::Checked(styles)
}
/// The ':disabled' selector.
pub fn disabled(styles: Vec<Style>) -> StateSelector {
    StateSelector::Disabled(styles)
}
/// The ':empty' selector.
pub fn empty(styles: Vec<Style>) -> StateSelector {
    StateSelector::Empty(styles)
}
/// The ':enabled' selector.
pub fn enabled(styles: Vec<Style>) -> StateSelector {
    StateSelector::Enabled(styles)
}
/// The ':first-child' selector.
pub fn first_child(styles: Vec<Style>) -> StateSelector {
    StateSelector::FirstChild(styles)
}
/// The '::first-letter' selector.
pub fn first_letter(styles: Vec<Style>) -> StateSelector {
    StateSelector::FirstLetter(styles)
}
/// The '::first-line' selector.
pub fn first_line(styles: Vec<Style>) -> StateSelector {
    StateSelector::FirstLine(styles)
}
/// The ':focus' selector.
pub fn focus(styles: Vec<Style>) -> StateSelector {
    StateSelector::Focus(styles)
}
/// The ':hover' selector.
pub fn hover(styles: Vec<Style>) -> StateSelector {
    StateSelector::Hover(styles)
}
/// The ':last-child' selector.
pub fn last_child(styles: Vec<Style>) -> StateSelector {
    StateSelector::LastChild(styles)
}
/// The ':only-child' selector.
pub fn only_child(styles: Vec<Style>) -> StateSelector {
    StateSelector::OnlyChild(styles)
}
/// The ':link' selector.
pub fn link(styles: Vec<Style>) -> StateSelector {
    StateSelector::Link(styles)
}
/// The ':visited' selector.
pub fn visited(styles: Vec<Style>) -> StateSelector {
    StateSelector::Visited(styles)
}
/// The '::spelling-error' selector.
pub fn spelling_error(styles: Vec<Style>) -> StateSelector {
    StateSelector::SpellingError(styles)
}
/// The '::grammar-error' selector.
pub fn grammar_error(styles: Vec<Style>) -> StateSelector {
    StateSelector::GrammarError(styles)
}
/// The '::selection' selector.
pub fn selection(styles: Vec<Style>) -> StateSelector {
    StateSelector::Selection(styles)
}
/// The '::placeholder' selector.
pub fn placeholder(styles: Vec<Style>) -> StateSelector {
    StateSelector::Placeholder(styles)
}
/// The '::marker' selector.
pub fn marker(styles: Vec<Style>) -> StateSelector {
    StateSelector::Marker(styles)
}
/// The '::cue' selector.
pub fn cue(styles: Vec<Style>) -> StateSelector {
    StateSelector::Cue(styles)
}
/// The '::backdrop' selector.
pub fn backdrop(styles: Vec<Style>) -> StateSelector {
    StateSelector::Backdrop(styles)
}