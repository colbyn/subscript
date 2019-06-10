
pub struct Declaration {
	pub selector: String,
	pub properties: Properties,
}

pub struct Properties(pub Vec<Property>);

pub struct Property {
	pub property: String,
	pub value: String,
}

pub struct Keyframes {
	pub name: String,
	pub keyframes: Vec<KeyframeInterval>,
}

pub struct KeyframeInterval {
	pub value: String,
	pub style: Properties,
}

pub struct Media {
	pub condition: Vec<Property>,
	pub declarations: Vec<Declaration>,
}


///////////////////////////////////////////////////////////////////////////////
// SYNTAX RENDERING
///////////////////////////////////////////////////////////////////////////////
impl Declaration {
	pub fn as_str(&self) -> String {
		format!("{} {}", self.selector, self.properties.as_str())
	}
}
impl Property {
	pub fn as_str(&self) -> String {
		format!("{}: {}", self.property, self.value)
	}
}
impl Properties {
	pub fn as_str(&self) -> String {
		let body = self.0
			.iter()
			.map(|x| format!("{};", x.as_str()))
			.collect::<Vec<_>>()
			.join("");
		format!("{{{}}}", body)
	}
}
impl Keyframes {
	pub fn as_str(&self) -> String {
		let body = self.keyframes
			.iter()
			.map(|x| x.as_str())
			.collect::<Vec<_>>()
			.join("");
		format!("@keyframes {} {{{}}}", self.name, body)
	}
}
impl KeyframeInterval {
	pub fn as_str(&self) -> String {
		format!("{} {}", self.value, self.style.as_str())
	}
}
impl Media {
	pub fn as_str(&self) -> String {
		let condition = self.condition
			.iter()
			.map(|x| format!("({})", x.as_str()))
			.collect::<Vec<_>>()
			.join(" and ");
		let declarations = self.declarations
			.iter()
			.map(|x| x.as_str())
			.collect::<Vec<_>>()
			.join("");
		format!("@keyframes {} {{{}}}", condition, declarations)
	}
}