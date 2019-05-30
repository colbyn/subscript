use std::iter::FromIterator;
use std::rc::*;
use std::cell::*;
use std::collections::*;
use ss_view_tree::styling::*;
use ss_view_tree::styling::syntax::*;
use ss_web_utils::dom::DomRef;
use ss_web_utils::{dom, js, js::console};
use ss_trees::ext::map::{SMap, MapApi};
use crate::LiveStylesheet;

///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type MediaSelectorType = String;

///////////////////////////////////////////////////////////////////////////////
// CRATE-LEVEL EXTERNAL API
///////////////////////////////////////////////////////////////////////////////

pub(crate) fn upsert(sheet: &LiveStylesheet) {
	GLOBAL_CSS_REGISTRY.with(move |reg| {
		reg.borrow_mut().upsert(sheet);
	});
}
pub(crate) fn remove(css_id: &CssId) {
	GLOBAL_CSS_REGISTRY.with(move |reg| {
		reg.borrow_mut().remove(css_id);
	});
}
pub fn sync() {
	GLOBAL_CSS_REGISTRY.with(move |reg| {
		reg.borrow_mut().sync();
	});
}

///////////////////////////////////////////////////////////////////////////////
// GLOBAL-CSS-REGISTRY
///////////////////////////////////////////////////////////////////////////////

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

impl GlobalCssRegistry {
	pub fn get_live(&mut self) -> &mut LiveCssRegistry {
		let mut start = || -> Self {
			GlobalCssRegistry::Live(LiveCssRegistry {
				mount: {
					let window = dom::window();
					let wrapper = dom::Tag::new("div");
					let local = dom::Tag::new("style");
					let state = dom::Tag::new("style");
					let media = dom::Tag::new("style");
					wrapper.append_child(&local);
					wrapper.append_child(&state);
					wrapper.append_child(&media);
					window.document.body.append_child(&wrapper);
					GlobalCssMount {wrapper,local,state,media}
				},
				node_locals_api: NodeLocalsApi{},
				node_states_api: NodeStatesApi{},
				global_media_api: GlobalMediaApi{},
				offline: OfflineGlobalStylesheet {
					node_locals: Rc::new(HashMap::new()),
					node_states: Rc::new(HashMap::new()),
					global_media: Rc::new(HashMap::new()),
				},
				online: OnlineGlobalStylesheet {
					node_locals: SMap::default(),
					node_states: SMap::default(),
					global_media: SMap::default(),
				},
			})
		};
		match self {
			GlobalCssRegistry::Pending => {
				*self = start();
				match self {
					GlobalCssRegistry::Live(x) => {x}
					GlobalCssRegistry::Pending => {panic!()}
				}
			}
			GlobalCssRegistry::Live(x) => x
		}
	}
	pub fn upsert(&mut self, sheet: &LiveStylesheet) {
		let live = self.get_live();
		live.upsert(sheet);
	}
	pub fn remove(&mut self, css_id: &CssId) {
		let live = self.get_live();
		live.remove(css_id);
	}
	pub fn sync(&mut self) {
		let live = self.get_live();
		live.sync();
	}
}


#[derive(Debug, PartialEq)]
pub struct LiveCssRegistry {
	mount: GlobalCssMount,
	node_locals_api: NodeLocalsApi,
	node_states_api: NodeStatesApi,
	global_media_api: GlobalMediaApi,
	offline: OfflineGlobalStylesheet,
	online: OnlineGlobalStylesheet,
}

impl LiveCssRegistry {
	pub fn upsert(&mut self, sheet: &LiveStylesheet) {
		// SETUP
		let css_id = sheet.css_id;
		let RenderedStylesheet{local, state, media} = sheet.value.borrow().render_css_syntax(&css_id);
		let local: RenderedSelector = local;
		let state: HashMap<StateSelectorType, RenderedSelector> = state;
		let media: HashMap<MediaSelectorType, RenderedSelector> = media;
		// UPSERTS
		self.offline.get_node_locals().insert(css_id, Rc::new(local));
		for (selector, rendered_selector) in state {
			self.offline.get_node_states().insert((css_id, selector), Rc::new(rendered_selector));
		}
		for (media_type, rendered_selector) in media {
			if let Some(existing) = self.offline.get_global_media().get_mut(&media_type) {
				let existing = Rc::make_mut(existing);
				existing.insert(css_id, rendered_selector);
			} else {
				let inner: HashMap<CssId, RenderedSelector> = HashMap::from_iter(vec![
					(css_id, rendered_selector)
				]);
				self.offline.get_global_media().insert(media_type, Rc::new(inner));
			}
		}
	}
	pub fn remove(&mut self, css_id: &CssId) {
		// GO - LOCALS
		self.offline.get_node_locals().remove(css_id);
		// GO - STATES
		let mut node_states: Vec<(CssId, StateSelectorType)> = Vec::new();
		for (id, ty) in self.offline.node_states.keys() {
			if css_id == id {
				node_states.push((id.clone(), ty.clone()));
			}
		}
		for key in node_states {
			self.offline.get_node_states().remove(&key);
		}
		// GO - GLOBAL MEDIA
		for inner in self.offline.get_global_media().values_mut() {
			let mut inner = Rc::make_mut(inner);
			let mut keys: Vec<CssId> = Vec::new();
			for id in inner.keys()  {
				if id == css_id {
					keys.push(id.clone());
				}
			}
			for key in keys {
				inner.remove(&key);
			}
		}
	}
	pub fn sync(&mut self) {
		self.online.node_locals.sync_ref(&self.node_locals_api, &self.mount, self.offline.node_locals.as_ref());
		self.online.node_states.sync_ref(&self.node_states_api, &self.mount, self.offline.node_states.as_ref());
		self.online.global_media.sync_ref(&self.global_media_api, &self.mount, self.offline.global_media.as_ref());
	}
} 


///////////////////////////////////////////////////////////////////////////////
// GLOBAL-STYLESHEET
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone)]
pub struct OfflineGlobalStylesheet {
	node_locals: Rc<HashMap<CssId, Rc<RenderedSelector>>>,
	node_states: Rc<HashMap<(CssId, StateSelectorType), Rc<RenderedSelector>>>,
	global_media: Rc<HashMap<MediaSelectorType, Rc<HashMap<CssId, RenderedSelector>>>>,
}

impl OfflineGlobalStylesheet {
	fn get_node_locals(&mut self) -> &mut HashMap<CssId, Rc<RenderedSelector>> {
		Rc::make_mut(&mut self.node_locals)
	}
	fn get_node_states(&mut self) -> &mut HashMap<(CssId, StateSelectorType), Rc<RenderedSelector>> {
		Rc::make_mut(&mut self.node_states)
	}
	fn get_global_media(&mut self) -> &mut HashMap<MediaSelectorType, Rc<HashMap<CssId, RenderedSelector>>> {
		Rc::make_mut(&mut self.global_media)
	}
}

#[derive(Debug, PartialEq)]
pub struct OnlineGlobalStylesheet {
	node_locals: SMap<GlobalCssMount, CssId, LiveSelector, Rc<RenderedSelector>>,
	node_states: SMap<GlobalCssMount, (CssId, StateSelectorType), LiveSelector, Rc<RenderedSelector>>,
	global_media: SMap<GlobalCssMount, MediaSelectorType, LiveMediaSelector, Rc<HashMap<CssId, RenderedSelector>>>,
}


#[derive(Debug, PartialEq)]
pub struct LiveSelector {
	dom_ref: dom::Text,
	value: Rc<RenderedSelector>,
}

impl LiveSelector {
	pub fn new(value: Rc<RenderedSelector>) -> Self {
		let dom_ref = dom::Text::new(value.0.as_str());
		LiveSelector{dom_ref, value}
	}
}

#[derive(Debug, PartialEq)]
pub struct LiveMediaSelector{
	dom_ref: dom::Text,
	value: Rc<HashMap<CssId, RenderedSelector>>
}


///////////////////////////////////////////////////////////////////////////////
// GLOBAL-STYLESHEET-CONTROLLER - DATA
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct GlobalCssMount {
	wrapper: dom::Tag,
	local: dom::Tag,
	state: dom::Tag,
	media: dom::Tag,
}

#[derive(Debug, PartialEq)]
pub struct NodeLocalsApi {}

#[derive(Debug, PartialEq)]
pub struct NodeStatesApi {}

#[derive(Debug, PartialEq)]
pub struct GlobalMediaApi {}

///////////////////////////////////////////////////////////////////////////////
// GLOBAL-STYLESHEET-CONTROLLER - COMMON
///////////////////////////////////////////////////////////////////////////////
pub fn selector_create(attached: &GlobalCssMount, new: Rc<RenderedSelector>) -> LiveSelector {
	let live_selector = LiveSelector::new(new);
	attached.local.append_child(&live_selector.dom_ref);
	live_selector
}
fn selector_modified(attached: &GlobalCssMount, old: &mut LiveSelector, new: Rc<RenderedSelector>) {
	if old.value != new {
		old.dom_ref.set_text_content(&new.0);
		old.value = new;
	}
}
fn selector_remove(attached: &GlobalCssMount, old: LiveSelector) {
	attached.local.remove_child(&old.dom_ref);
}
fn selector_unchanged(old: &LiveSelector, new: &Rc<RenderedSelector>) -> bool {
	&old.value == new
}

fn render_media_selector(key: &MediaSelectorType, value: &Rc<HashMap<CssId, RenderedSelector>>) -> String {
	let mut inner: Vec<String> = Vec::with_capacity(value.len());
	for x in value.values() {
		inner.push(x.0.clone());
	}
	let inner: String = inner.join("");
	let result = format!(
		"@media {key} {{{inner}}}",
		key=key,
		inner=inner,
	);
	result
}


///////////////////////////////////////////////////////////////////////////////
// GLOBAL-STYLESHEET-CONTROLLER - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////
impl MapApi<GlobalCssMount, CssId, LiveSelector, Rc<RenderedSelector>> for NodeLocalsApi {
	fn create(&self, attached: &GlobalCssMount, key: &CssId, new: Rc<RenderedSelector>) -> LiveSelector {
		selector_create(attached, new)
	}
	fn modified(&self, attached: &GlobalCssMount, key: &CssId, old: &mut LiveSelector, new: Rc<RenderedSelector>) {
		selector_modified(attached, old, new)
	}
	fn remove(&self, attached: &GlobalCssMount, key: CssId, old: LiveSelector) {
		selector_remove(attached, old)
	}
	fn unchanged(&self, old: &LiveSelector, new: &Rc<RenderedSelector>) -> bool {
		selector_unchanged(old, new)
	}
}


impl MapApi<GlobalCssMount, (CssId, StateSelectorType), LiveSelector, Rc<RenderedSelector>> for NodeStatesApi {
	fn create(&self, attached: &GlobalCssMount, key: &(CssId, StateSelectorType), new: Rc<RenderedSelector>) -> LiveSelector {
		selector_create(attached, new)
	}
	fn modified(&self, attached: &GlobalCssMount, key: &(CssId, StateSelectorType), old: &mut LiveSelector, new: Rc<RenderedSelector>) {
		selector_modified(attached, old, new)
	}
	fn remove(&self, attached: &GlobalCssMount, key: (CssId, StateSelectorType), old: LiveSelector) {
		selector_remove(attached, old)
	}
	fn unchanged(&self, old: &LiveSelector, new: &Rc<RenderedSelector>) -> bool {
		selector_unchanged(old, new)
	}
}



impl MapApi<GlobalCssMount, MediaSelectorType, LiveMediaSelector, Rc<HashMap<CssId, RenderedSelector>>> for GlobalMediaApi {
	fn create(
		&self,
		attached: &GlobalCssMount,
		key: &MediaSelectorType,
		new: Rc<HashMap<CssId, RenderedSelector>>
	) -> LiveMediaSelector {
		let dom_ref = dom::Text::new(&render_media_selector(key, &new));
		attached.media.append_child(&dom_ref);
		LiveMediaSelector{dom_ref, value: new}
	}
	fn modified(
		&self,
		attached: &GlobalCssMount,
		key: &MediaSelectorType,
		old: &mut LiveMediaSelector,
		new: Rc<HashMap<CssId, RenderedSelector>>
	) {
		if old.value != new {
			old.dom_ref.set_text_content(&render_media_selector(key, &new));
			old.value = new;
		}
	}
	fn remove(
		&self,
		attached: &GlobalCssMount,
		key: MediaSelectorType,
		old: LiveMediaSelector
	) {
		attached.media.remove_child(&old.dom_ref);
	}
	fn unchanged(
		&self,
		old: &LiveMediaSelector,
		new: &Rc<HashMap<CssId, RenderedSelector>>
	) -> bool {
		&old.value == new
	}
}

