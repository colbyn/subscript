use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum AttributeValue {
    Value(String),
    Toggle(bool),
}

impl AttributeValue {
	pub fn get_string(&self) -> Option<&String> {
		match self {
			AttributeValue::Value(x) => Some(&x),
			_ => None
		}
	}
	pub fn get_bool(&self) -> Option<bool> {
		match self {
			AttributeValue::Toggle(x) => Some(x.clone()),
			_ => None
		}	
	}
}

pub trait AttributeValueInterface {
	fn to_attribute(self) -> AttributeValue;
}
impl AttributeValueInterface for String {
	fn to_attribute(self) -> AttributeValue {
		AttributeValue::Value(self)
	}
}
impl AttributeValueInterface for &str {
	fn to_attribute(self) -> AttributeValue {
		AttributeValue::Value(String::from(self))
	}
}
impl AttributeValueInterface for bool {
	fn to_attribute(self) -> AttributeValue {
		AttributeValue::Toggle(self)
	}
}

pub fn internal_normalize_attribute_value(x: impl AttributeValueInterface) -> AttributeValue {
	x.to_attribute()
}





