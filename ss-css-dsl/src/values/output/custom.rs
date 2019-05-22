use std::hash::{Hash};
use crate::core::{self, CssRuleSyntax};

/// The "px" value.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Px;

/// The "rem" value.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Rem;

/// The "em" value.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Em;

/// The "cm" value.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Cm;

/// The "pt" value.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Pt;

/// The "q" value.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Q;

/// The "mm" value.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mm;

/// The "in" value.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct In;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Url(pub String);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Length {
    Px{
        v: i32,
    },
    Rem{
        v: i32,
    },
    Em{
        v: i32,
    },
    Cm{
        v: i32,
    },
    Pt{
        v: i32,
    },
    Q{
        v: i32,
    },
    Mm{
        v: i32,
    },
    In{
        v: i32,
    },
}
impl CssRuleSyntax for Length {
    fn css_syntax(&self) -> String {
        match self {
            Length::Px{v, ..} =>
                format!("{}px", v),
            Length::Rem{v, ..} =>
                format!("{}rem", v),
            Length::Em{v, ..} =>
                format!("{}em", v),
            Length::Cm{v, ..} =>
                format!("{}cm", v),
            Length::Pt{v, ..} =>
                format!("{}pt", v),
            Length::Q{v, ..} =>
                format!("{}q", v),
            Length::Mm{v, ..} =>
                format!("{}mm", v),
            Length::In{v, ..} =>
                format!("{}in", v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Angle {
    Deg,
    Rad,
    Grad,
    Turn,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Time {
    S(i32),
    Ms(i32)
}
pub trait Number {
    fn normalize(self) -> i32;
}
impl Number for i32 {
    fn normalize(self) -> i32 {
        self as i32
    }
}