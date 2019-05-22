//! This crate combines CSS at-rules, pseudo selectors and pseudo elements.

use crate::api::styles::Style;
use crate::selectors::{MediaQuerySelector, StateSelector};

/// The '@media' at-rule.
pub fn media(header: Vec<Style>, body: Vec<Style>) -> MediaQuerySelector {
    unimplemented!()
}

/// The ':active' selector.
pub fn active(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The '::after' selector.
pub fn after(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The '::before' selector.
pub fn before(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The ':checked' selector.
pub fn checked(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The ':disabled' selector.
pub fn disabled(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The ':empty' selector.
pub fn empty(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The ':enabled' selector.
pub fn enabled(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The ':first-child' selector.
pub fn first_child(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The '::first-letter' selector.
pub fn first_letter(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The '::first-line' selector.
pub fn first_line(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The ':focus' selector.
pub fn focus(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The ':hover' selector.
pub fn hover(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The ':last-child' selector.
pub fn last_child(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}
/// The ':only-child' selector.
pub fn only_child(styles: Vec<Style>) -> StateSelector {
	unimplemented!()
}
/// The ':link' selector.
pub fn link(styles: Vec<Style>) -> StateSelector {
	unimplemented!()
}
/// The ':visited' selector.
pub fn visited(styles: Vec<Style>) -> StateSelector {
	unimplemented!()
}
/// The '::spelling-error' selector.
pub fn spelling_error(styles: Vec<Style>) -> StateSelector {
	unimplemented!()
}
/// The '::grammar-error' selector.
pub fn grammar_error(styles: Vec<Style>) -> StateSelector {
	unimplemented!()
}
/// The '::selection' selector.
pub fn selection(styles: Vec<Style>) -> StateSelector {
	unimplemented!()
}
/// The '::placeholder' selector.
pub fn placeholder(styles: Vec<Style>) -> StateSelector {
	unimplemented!()
}
/// The '::marker' selector.
pub fn marker(styles: Vec<Style>) -> StateSelector {
	unimplemented!()
}
/// The '::cue' selector.
pub fn cue(styles: Vec<Style>) -> StateSelector {
	unimplemented!()
}
/// The '::backdrop' selector.
pub fn backdrop(styles: Vec<Style>) -> StateSelector {
    unimplemented!()
}