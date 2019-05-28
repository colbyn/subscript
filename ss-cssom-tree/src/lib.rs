//! This will be created and updated internally by the Subscript program.
#![allow(dead_code, unused, unused_variables)]

use std::collections::*;
use std::cell::*;
use std::hash::{Hash, Hasher};
use ss_css_types::internal::*;
use ss_css_types::stylesheet::syntax::*;
use ss_trees::ext::map::{SMap, MapApi};
use ss_web_utils::{dom, js, js::console, dom::DomRef};


///////////////////////////////////////////////////////////////////////////////
// GLOBAL CSS MOUNT
///////////////////////////////////////////////////////////////////////////////

pub type HashKey = u64;

thread_local! {
    pub static GLOBAL_CSS_REGISTRY: RefCell<GlobalCssRegistry> = {
    	RefCell::new(GlobalCssRegistry::Pending)
    };
}

#[derive(Debug, PartialEq)]
pub enum GlobalCssRegistry {
	Pending,
	Live(LiveCssRegistry)
}

#[derive(Debug)]
pub struct LiveCssRegistry {
	mount: LiveCssMount,
	offline_entries: HashMap<u64, Stylesheet>,
	online_entries: SMap<LiveCssMount, u64, CssEntry, Stylesheet>,
}


impl PartialEq for LiveCssRegistry {
    fn eq(&self, other: &LiveCssRegistry) -> bool {
    	self.offline_entries == other.offline_entries &&
    	self.online_entries == other.online_entries
    }
}

impl GlobalCssRegistry {
	pub(crate) fn init(&mut self) {
		let window = dom::window();
		let dom_ref = window.document.create_element("style");
		window.document.body.append_child(&dom_ref);
		*self = GlobalCssRegistry::Live(LiveCssRegistry {
			mount: LiveCssMount {
				dom_ref
			},
			offline_entries: HashMap::new(),
			online_entries: SMap::default(),
		});
	}
	pub(crate) fn enliven(&mut self) -> &mut LiveCssRegistry {
		match self {
			GlobalCssRegistry::Live(live) => {live}
			GlobalCssRegistry::Pending => {
				self.init();
				self.sync();
				match self {
					GlobalCssRegistry::Live(x) => {x}
					GlobalCssRegistry::Pending => {panic!()}
				}
			}
		}
	}
}

///////////////////////////////////////////////////////////////////////////////
// EXTERNAL API
///////////////////////////////////////////////////////////////////////////////

impl GlobalCssRegistry {
	pub fn upsert(&mut self, sheet: Stylesheet) -> HashKey {
		let live = self.enliven();
		let hash_key = calculate_hash(&sheet);
		live.offline_entries.insert(hash_key.clone(), sheet);
		hash_key
	}
	pub fn sync(&mut self) {
		let live = self.enliven();
		let unchanged = live.online_entries.unchanged(&CssMapApi::default(), &live.offline_entries);
		if !unchanged {
			live.online_entries.sync(
				&CssMapApi::default(),
				&live.mount,
				live.offline_entries.clone(),
			);
		}
	}
}




///////////////////////////////////////////////////////////////////////////////
// GLOBAL CSS MAP ENTRY
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct LiveCssMount {
	dom_ref: dom::Tag,
}

impl PartialEq for LiveCssMount {
    fn eq(&self, other: &LiveCssMount) -> bool {true}
}

#[derive(Debug)]
pub struct CssEntry {
	dom_ref: dom::Text,
	hash: u64,
	stylesheet: Stylesheet
}

impl PartialEq for CssEntry {
    fn eq(&self, other: &CssEntry) -> bool {
    	self.hash == other.hash &&
        self.stylesheet == other.stylesheet
    }
}


pub struct CssMapApi {
	window: dom::Window,
}

impl Default for CssMapApi {
	fn default() -> Self {
		CssMapApi {
			window: dom::window(),
		}
	}
}

impl MapApi<LiveCssMount, u64, CssEntry, Stylesheet> for CssMapApi {
	fn create(&self, attached: &LiveCssMount, key: &u64, new: Stylesheet) -> CssEntry {
		let RenderedStylesheet{locals: locals_stringified} = new.render_css_syntax(key);
		let text_node = self.window.document.create_text_node(locals_stringified.as_str());
		attached.dom_ref.append_child(&text_node);
		CssEntry {
			dom_ref: text_node,
			hash: key.clone(),
			stylesheet: new,
		}
	}
	fn modified(&self, attached: &LiveCssMount, key: &u64, old: &mut CssEntry, new: Stylesheet) {
		let RenderedStylesheet{locals: locals_stringified} = new.render_css_syntax(key);
		old.dom_ref.set_text_content(&locals_stringified);
		old.hash = calculate_hash(&new);
		old.stylesheet = new;
	}
	fn remove(&self, attached: &LiveCssMount, key: u64, old: CssEntry) {
		attached.dom_ref.remove_child(&old.dom_ref);
	}
	fn unchanged(&self, old: &CssEntry, new: &Stylesheet) -> bool {
		&old.stylesheet == new
	}
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
	use std::collections::hash_map::DefaultHasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

