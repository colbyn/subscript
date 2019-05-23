use std::fmt;
use core::cmp::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Url(pub String);

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Length {
    Px(Float),
    Rem(Float),
    Em(Float),
    Cm(Float),
    Pt(Float),
    Q(Float),
    Mm(Float),
    In(Float),
}

impl Length {
    pub fn px(x: impl Number) -> Self {
        Length::Px(x.normalize())
    }
    pub fn render_css_syntax(&self) -> String {
        unimplemented!()
    }
}




///////////////////////////////////////////////////////////////////////////////
// NUMBER
///////////////////////////////////////////////////////////////////////////////

pub trait Number {
    fn normalize(self) -> Float;
}
impl Number for f64 {
    fn normalize(self) -> Float {
        Float(self)
    }
}
impl Number for i32 {
    fn normalize(self) -> Float {
        Float(self as f64)
    }
}


#[derive(Debug, Clone, PartialOrd)]
pub struct Float(f64);

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Eq for Float {}
impl Ord for Float {
    fn cmp(&self, other: &Float) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Less)
    }
}
impl PartialEq for Float {
    fn eq(&self, other: &Float) -> bool {
        self.0 == other.0
    }
}
impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x = format!("{}", self.0);
        x.hash(state);
    }
}