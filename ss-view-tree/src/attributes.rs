use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Attribute {
    Value(String),
    Toggle(bool),
}
