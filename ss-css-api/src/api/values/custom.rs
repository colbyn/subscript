use std::hash::{Hash};

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