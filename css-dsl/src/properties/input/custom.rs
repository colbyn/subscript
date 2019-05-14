use crate::properties::output;
use crate::values as v;

/// The "display" property.
pub trait Display {
    fn normalize(self) -> output::Display;
}
impl Display for v::output::Inline {
    fn normalize(self) -> output::Display {
        output::Display::Inline
    }
}
impl Display for v::output::Block {
    fn normalize(self) -> output::Display {
        output::Display::Block
    }
}
impl Display for v::output::ListItem {
    fn normalize(self) -> output::Display {
        output::Display::ListItem
    }
}
impl Display for v::output::InlineBlock {
    fn normalize(self) -> output::Display {
        output::Display::InlineBlock
    }
}
impl Display for v::output::Table {
    fn normalize(self) -> output::Display {
        output::Display::Table
    }
}
impl Display for v::output::InlineTable {
    fn normalize(self) -> output::Display {
        output::Display::InlineTable
    }
}
impl Display for v::output::TableRowGroup {
    fn normalize(self) -> output::Display {
        output::Display::TableRowGroup
    }
}
impl Display for v::output::TableHeaderGroup {
    fn normalize(self) -> output::Display {
        output::Display::TableHeaderGroup
    }
}
impl Display for v::output::TableFooterGroup {
    fn normalize(self) -> output::Display {
        output::Display::TableFooterGroup
    }
}
impl Display for v::output::TableRow {
    fn normalize(self) -> output::Display {
        output::Display::TableRow
    }
}
impl Display for v::output::TableColumnGroup {
    fn normalize(self) -> output::Display {
        output::Display::TableColumnGroup
    }
}
impl Display for v::output::TableColumn {
    fn normalize(self) -> output::Display {
        output::Display::TableColumn
    }
}
impl Display for v::output::TableCell {
    fn normalize(self) -> output::Display {
        output::Display::TableCell
    }
}
impl Display for v::output::TableCaption {
    fn normalize(self) -> output::Display {
        output::Display::TableCaption
    }
}
impl Display for v::output::None {
    fn normalize(self) -> output::Display {
        output::Display::None
    }
}
impl Display for v::output::Inherit {
    fn normalize(self) -> output::Display {
        output::Display::Inherit
    }
}
impl Display for v::output::Flex {
    fn normalize(self) -> output::Display {
        output::Display::Flex
    }
}
impl Display for v::output::Grid {
    fn normalize(self) -> output::Display {
        output::Display::Grid
    }
}
impl Display for v::output::Contents {
    fn normalize(self) -> output::Display {
        output::Display::Contents
    }
}
/// The "padding" property.
pub trait Padding {
    fn normalize(self) -> output::Padding;
}
impl Padding for i32 {
    fn normalize(self) -> output::Padding {
        output::Padding::P1(
            v::px(self)
        )
    }
}
impl Padding for (i32, i32) {
    fn normalize(self) -> output::Padding {
        output::Padding::P2(
            v::px(self.0),
            v::px(self.1),
        )
    }
}
impl Padding for (i32, i32, i32) {
    fn normalize(self) -> output::Padding {
        output::Padding::P3(
            v::px(self.0),
            v::px(self.1),
            v::px(self.2),
        )
    }
}
impl Padding for (i32, i32, i32, i32) {
    fn normalize(self) -> output::Padding {
        output::Padding::P4(
            v::px(self.0),
            v::px(self.1),
            v::px(self.2),
            v::px(self.3),
        )
    }
}
impl Padding for v::o::Length {
    fn normalize(self) -> output::Padding {
        output::Padding::P1(
            self
        )
    }
}
impl Padding for (v::o::Length, v::o::Length) {
    fn normalize(self) -> output::Padding {
        output::Padding::P2(
            self.0,
            self.1,
        )
    }
}
impl Padding for (v::o::Length, v::o::Length, v::o::Length) {
    fn normalize(self) -> output::Padding {
        output::Padding::P3(
            self.0,
            self.1,
            self.2,
        )
    }
}
impl Padding for (v::o::Length, v::o::Length, v::o::Length, v::o::Length) {
    fn normalize(self) -> output::Padding {
        output::Padding::P4(
            self.0,
            self.1,
            self.2,
            self.3,
        )
    }
}
/// The "padding-bottom" property.
pub trait PaddingBottom {
    fn normalize(self) -> output::PaddingBottom;
}
impl PaddingBottom for v::o::Length {
    fn normalize(self) -> output::PaddingBottom {
        output::PaddingBottom(self)
    }
}
/// The "padding-left" property.
pub trait PaddingLeft {
    fn normalize(self) -> output::PaddingLeft;
}
impl PaddingLeft for v::o::Length {
    fn normalize(self) -> output::PaddingLeft {
        output::PaddingLeft(self)
    }
}
/// The "padding-right" property.
pub trait PaddingRight {
    fn normalize(self) -> output::PaddingRight;
}
impl PaddingRight for v::o::Length {
    fn normalize(self) -> output::PaddingRight {
        output::PaddingRight(self)
    }
}
/// The "padding-top" property.
pub trait PaddingTop {
    fn normalize(self) -> output::PaddingTop;
}
impl PaddingTop for v::o::Length {
    fn normalize(self) -> output::PaddingTop {
        output::PaddingTop(self)
    }
}
/// The "width" property.
pub trait Width {
    fn normalize(self) -> output::Width;
}
impl Width for v::o::Length {
    fn normalize(self) -> output::Width {
        output::Width(self)
    }
}
/// The "max-width" property.
pub trait MaxWidth {
    fn normalize(self) -> output::MaxWidth;
}
impl MaxWidth for v::o::Length {
    fn normalize(self) -> output::MaxWidth {
        output::MaxWidth(self)
    }
}
/// The "min-width" property.
pub trait MinWidth {
    fn normalize(self) -> output::MinWidth;
}
impl MinWidth for v::o::Length {
    fn normalize(self) -> output::MinWidth {
        output::MinWidth(self)
    }
}
/// The "height" property.
pub trait Height {
    fn normalize(self) -> output::Height;
}
impl Height for v::o::Length {
    fn normalize(self) -> output::Height {
        output::Height(self)
    }
}
/// The "max-height" property.
pub trait MaxHeight {
    fn normalize(self) -> output::MaxHeight;
}
impl MaxHeight for v::o::Length {
    fn normalize(self) -> output::MaxHeight {
        output::MaxHeight(self)
    }
}
/// The "min-height" property.
pub trait MinHeight {
    fn normalize(self) -> output::MinHeight;
}
impl MinHeight for v::o::Length {
    fn normalize(self) -> output::MinHeight {
        output::MinHeight(self)
    }
}