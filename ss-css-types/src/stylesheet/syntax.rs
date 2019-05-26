use crate::stylesheet::*;
use crate::rules::*;
use crate::selectors::*;

pub struct RenderedStylesheet {
	pub locals: String,
}

impl Stylesheet {
	pub fn render_css_syntax(&self, key: &CssHashKey) -> RenderedStylesheet {
		let mut rendered_locals: Vec<String> = Vec::new();
		for style in self.local.iter() {
			rendered_locals.push(format!("{};", style.render_css_syntax()));
		}
		RenderedStylesheet {
			locals: {
				let body = rendered_locals.join("");
				format!("[css=\"{key}\"] {{{body}}}", key=key, body=body)
			},
		}
	}
}


impl Style {
	pub fn render_css_syntax(&self) -> String {
		match self {
			Style::Native(rule) => rule.to_css_syntax(),
			Style::Raw{property, value} => format!(
				"{prop}: {value}",
				prop=property,
				value=value,
			),
		}
	}	
}