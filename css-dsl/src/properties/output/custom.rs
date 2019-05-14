use crate::core::{self, Style, CssRuleSyntax};
use crate::values as v;

/// The "display" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Display {
    Inline,
    Block,
    ListItem,
    InlineBlock,
    Table,
    InlineTable,
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableColumnGroup,
    TableColumn,
    TableCell,
    TableCaption,
    None,
    Inherit,
    Flex,
    Grid,
    Contents,
}
impl CssRuleSyntax for Display {
    fn css_syntax(&self) -> String {
        let res = match self {
            Display::Inline =>
                "display: inline",
            Display::Block =>
                "display: block",
            Display::ListItem =>
                "display: list-item",
            Display::InlineBlock =>
                "display: inline-block",
            Display::Table =>
                "display: table",
            Display::InlineTable =>
                "display: inline-table",
            Display::TableRowGroup =>
                "display: table-row-group",
            Display::TableHeaderGroup =>
                "display: table-header-group",
            Display::TableFooterGroup =>
                "display: table-footer-group",
            Display::TableRow =>
                "display: table-row;",
            Display::TableColumnGroup =>
                "display: table-column-group",
            Display::TableColumn =>
                "display: table-column",
            Display::TableCell =>
                "display: table-cell",
            Display::TableCaption =>
                "display: table-caption",
            Display::None =>
                "display: none",
            Display::Inherit =>
                "display: inherit",
            Display::Flex =>
                "display: flex",
            Display::Grid =>
                "display: grid",
            Display::Contents =>
                "display: contents",
        };
        String::from(res)
    }
}
/// The "padding" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Padding {
    P1(v::o::Length),
    P2(v::o::Length, v::o::Length),
    P3(v::o::Length, v::o::Length, v::o::Length),
    P4(v::o::Length, v::o::Length, v::o::Length, v::o::Length),
}
impl CssRuleSyntax for Padding {
    fn css_syntax(&self) -> String {
        let res = match self {
            Padding::P1(p1) =>
                format!("padding: {}", p1.css_syntax()),
            Padding::P2(p1, p2) =>
                format!("padding: {} {}", p1.css_syntax(), p2.css_syntax()),
            Padding::P3(p1, p2, p3) =>
                format!("padding: {} {} {}", p1.css_syntax(), p2.css_syntax(), p3.css_syntax()),
            Padding::P4(p1, p2, p3, p4) =>
                format!("padding: {} {} {} {}", p1.css_syntax(), p2.css_syntax(), p3.css_syntax(), p4.css_syntax()),
        };
        String::from(res)
    }
}
/// The "padding-bottom" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PaddingBottom(pub v::o::Length);
impl CssRuleSyntax for PaddingBottom {
    fn css_syntax(&self) -> String {
        format!("padding-bottom: {}", self.0.css_syntax())
    }
}
/// The "padding-left" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PaddingLeft(pub v::o::Length);
impl CssRuleSyntax for PaddingLeft {
    fn css_syntax(&self) -> String {
        format!("padding-left: {}", self.0.css_syntax())
    }
}
/// The "padding-right" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PaddingRight(pub v::o::Length);
impl CssRuleSyntax for PaddingRight {
    fn css_syntax(&self) -> String {
        format!("padding-right: {}", self.0.css_syntax())
    }
}
/// The "padding-top" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PaddingTop(pub v::o::Length);
impl CssRuleSyntax for PaddingTop {
    fn css_syntax(&self) -> String {
        format!("padding-top: {}", self.0.css_syntax())
    }
}
/// The "max-height" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MaxHeight(pub v::o::Length);
impl CssRuleSyntax for MaxHeight {
    fn css_syntax(&self) -> String {
        format!("max-height: {}", self.0.css_syntax())
    }
}

/// The "max-width" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MaxWidth(pub v::o::Length);
impl CssRuleSyntax for MaxWidth {
    fn css_syntax(&self) -> String {
        format!("max-width: {}", self.0.css_syntax())
    }
}

/// The "min-height" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MinHeight(pub v::o::Length);
impl CssRuleSyntax for MinHeight {
    fn css_syntax(&self) -> String {
        format!("min-height: {}", self.0.css_syntax())
    }
}

/// The "min-width" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MinWidth(pub v::o::Length);
impl CssRuleSyntax for MinWidth {
    fn css_syntax(&self) -> String {
        format!("min-width: {}", self.0.css_syntax())
    }
}

/// The "width" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Width(pub v::o::Length);
impl CssRuleSyntax for Width {
    fn css_syntax(&self) -> String {
        format!("width: {}", self.0.css_syntax())
    }
}

/// The "height" property.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Height(pub v::o::Length);
impl CssRuleSyntax for Height {
    fn css_syntax(&self) -> String {
        format!("height: {}", self.0.css_syntax())
    }
}