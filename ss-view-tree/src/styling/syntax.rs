use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::styling::*;

pub type MediaSelectorType = String;
pub type KeyframeHash = u64;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderedStylesheet {
	pub local: RenderedSelector,
	pub state: HashMap<StateSelectorType, RenderedSelector>,
	pub media: HashMap<MediaSelectorType, RenderedSelector>,
	pub keyframes: HashMap<KeyframeHash, RenderedSelector>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderedSelector(pub String);

impl Stylesheet {
	pub fn render_css_syntax(&self, css_id: &CssId) -> RenderedStylesheet {
		let mut local: Vec<String> = Vec::with_capacity(self.local.len());
		let mut state: HashMap<StateSelectorType, RenderedSelector> = HashMap::new();
		let mut keyframes: HashMap<KeyframeHash, RenderedSelector> = HashMap::new();
		let mut media: HashMap<MediaSelectorType, RenderedSelector> = HashMap::new();
		let mut active_keyframe_idents: Vec<String> = Vec::new();
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
				body.push(format!("{};", s.render_css_syntax()));
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
				body.push(format!("{};", s.render_css_syntax()));
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
		for keyframes_entry in self.keyframes.iter() {
			let keyframe_hash: u64 = calculate_hash(keyframes_entry);
			let keyframe_ident = format!(
				"{css_id}--{keyframe_hash}",
				css_id=css_id,
				keyframe_hash=keyframe_hash,
			);
			active_keyframe_idents.push(keyframe_ident.clone());
			let mut result = Vec::with_capacity(keyframes_entry.0.len());
			let render_interval = |i: &KeyframeInterval| -> String {
				let mut body: Vec<String> = Vec::with_capacity(keyframes_entry.0.len());
				for s in i.body.iter() {
					body.push(format!("{};", s.render_css_syntax()));
				}
				format!("{value} {{{body}}}", value=i.value, body=body.join(""))
			};
			for i in keyframes_entry.0.iter() {
				result.push(render_interval(i));
			}
			let result = RenderedSelector(format!(
				"@keyframes {keyframe_ident} {{{body}}}",
				keyframe_ident=keyframe_ident,
				body=result.join(""),
			));
			keyframes.insert(keyframe_hash, result);
		}
		// UPDATE LOCALS WITH KEYFRAME IDENTS
		if !active_keyframe_idents.is_empty() {
			local.push(format!(
				"animation-name: {names};",
				names=active_keyframe_idents.join(","),
			));
		}
		// CONCAT LOCALS
		let local: String = format!(
			"[css=\"{css_id}\"] {{{body}}}",
			css_id=css_id,
			body=local.join(""),
		);
		// DONE
		RenderedStylesheet {
			local: RenderedSelector(local),
			state,
			media,
			keyframes,
		}
	}
}

impl StateSelectorType {
	pub fn render_css_syntax(&self) -> &str {
		match self {
			StateSelectorType::Active => ":active",
			StateSelectorType::After => "::after",
			StateSelectorType::Before => "::before",
			StateSelectorType::Checked => ":checked",
			StateSelectorType::Disabled => ":disabled",
			StateSelectorType::Empty => ":empty",
			StateSelectorType::Enabled => ":enabled",
			StateSelectorType::FirstChild => ":firstchild",
			StateSelectorType::FirstLetter => "::firstletter",
			StateSelectorType::FirstLine => "::firstline",
			StateSelectorType::Focus => ":focus",
			StateSelectorType::Hover => ":hover",
			StateSelectorType::LastChild => ":lastchild",
			StateSelectorType::OnlyChild => ":onlychild",
			StateSelectorType::Link => ":link",
			StateSelectorType::Visited => ":visited",
			StateSelectorType::SpellingError => "::spellingerror",
			StateSelectorType::GrammarError => "::grammarerror",
			StateSelectorType::Selection => "::selection",
			StateSelectorType::Placeholder => "::placeholder",
			StateSelectorType::Marker => "::marker",
			StateSelectorType::Cue => "::cue",
			StateSelectorType::Backdrop => "::backdrop",
		}
	}
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}