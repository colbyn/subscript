use crate::core::{self, Style, MediaSelector};

/// The '@media' at-rule.
pub fn media(header: Vec<Style>, body: Vec<Style>) -> MediaSelector {
    MediaSelector {
        header: header,
        body: body,
    }
}
