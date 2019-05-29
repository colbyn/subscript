use crate::styling::*;

pub type MediaSelectorType = String;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderedStylesheet {
	pub local: RenderedSelector,
	pub state: HashMap<StateSelectorType, RenderedSelector>,
	pub media: HashMap<MediaSelectorType, RenderedSelector>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderedSelector(pub String);

impl Stylesheet {
	pub fn render_css_syntax(&self, css_id: &CssId) -> RenderedStylesheet {
		let mut local: Vec<String> = Vec::with_capacity(self.local.len());
		let mut state: HashMap<StateSelectorType, RenderedSelector> = HashMap::new();
		let mut media: HashMap<MediaSelectorType, RenderedSelector> = HashMap::new();
		for x in self.local.iter() {
			local.push(format!("{};", x.render_css_syntax()));
		}
		for x in self.media.iter() {
			// INIT
			let mut media_selector: Vec<String> = Vec::with_capacity(x.selector.len());
			let mut body: Vec<String> = Vec::with_capacity(x.body.len());
			for s in x.selector.iter() {
				media_selector.push(format!("({})", s.render_css_syntax()));
			}
			for s in x.body.iter() {
				local.push(format!("{};", s.render_css_syntax()));
			}
			// CONCAT
			let media_selector: String = media_selector.join(" and ");
			let body: String = body.join("");
			let body: String = format!(
				"[css=\"{css_id}\"] {{{body}}}",
				css_id=css_id,
				body=body,
			);
			let body: RenderedSelector = RenderedSelector(body);
			// DONE
			media.insert(media_selector, body);
		}
		for x in self.state.iter() {
			// INIT
			let mut selector: &str = x.selector.render_css_syntax();
			let mut body: Vec<String> = Vec::with_capacity(x.body.len());
			for s in x.body.iter() {
				local.push(format!("{};", s.render_css_syntax()));
			}
			// CONCAT
			let body: String = body.join("");
			let body = RenderedSelector(format!(
				"[css=\"{css_id}\"]{selector} {{{body}}}",
				css_id=css_id,
				selector=selector,
				body=body,
			));
			// DONE
			state.insert(x.selector.clone(), body);
		}
		// CONCAT
		let local: String = local.join("");
		// DONE
		RenderedStylesheet {
			local: RenderedSelector(local),
			state,
			media,
		}
	}
}

// TODO: pseudo-elements use "::"!
impl StateSelectorType {
	pub fn render_css_syntax(&self) -> &str {
		match self {
			StateSelectorType::Active => ":active",
			StateSelectorType::After => ":after",
			StateSelectorType::Before => ":before",
			StateSelectorType::Checked => ":checked",
			StateSelectorType::Disabled => ":disabled",
			StateSelectorType::Empty => ":empty",
			StateSelectorType::Enabled => ":enabled",
			StateSelectorType::FirstChild => ":firstchild",
			StateSelectorType::FirstLetter => ":firstletter",
			StateSelectorType::FirstLine => ":firstline",
			StateSelectorType::Focus => ":focus",
			StateSelectorType::Hover => ":hover",
			StateSelectorType::LastChild => ":lastchild",
			StateSelectorType::OnlyChild => ":onlychild",
			StateSelectorType::Link => ":link",
			StateSelectorType::Visited => ":visited",
			StateSelectorType::SpellingError => ":spellingerror",
			StateSelectorType::GrammarError => ":grammarerror",
			StateSelectorType::Selection => ":selection",
			StateSelectorType::Placeholder => ":placeholder",
			StateSelectorType::Marker => ":marker",
			StateSelectorType::Cue => ":cue",
			StateSelectorType::Backdrop => ":backdrop",
		}
	}
}
