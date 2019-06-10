pub mod hashids;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::cell::*;
use std::collections::*;

use crate::backend::css;
use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi, Element, Stylesheet, AdjacentPosition};
use crate::view::shared::*;

///////////////////////////////////////////////////////////////////////////////
// EXTERNAL API
///////////////////////////////////////////////////////////////////////////////

pub(crate) fn upsert(styling: &Styling) -> StylingEnv {
    CSS_REGISTRY.with(move |reg| {
        reg .borrow_mut()
            .notify(styling);
    });
    styling_env(&styling)
}

pub(crate) fn removed(styling: &Styling) -> StylingEnv {
    styling_env(&styling)
}

pub struct StylingEnv {
    pub css_id: u64,
    pub animation_ids: Vec<u64>,
}

impl StylingEnv {
    pub fn css_id(&self) -> String {
        format!("cid-{}", self.css_id)
    }
    pub fn animation_ids(&self) -> Vec<String> {
        self.animation_ids
            .iter()
            .map(|x| format!("cid-{}-{}", self.css_id, x))
            .collect()
    }
}


///////////////////////////////////////////////////////////////////////////////
// DATA
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub(crate) static CSS_REGISTRY: RefCell<CssRegistry> = {
        RefCell::new(CssRegistry::Pending)
    };
}

pub enum CssRegistry {
    Pending,
    Live(LiveCssRegistry)
}

pub struct LiveCssRegistry {
    mount: CssMount,
    added: HashSet<u64>,
}

pub struct CssMount {
    wrapper: browser::Element,
    local: browser::Stylesheet,
    state: browser::Stylesheet,
    media: browser::Stylesheet,
}



///////////////////////////////////////////////////////////////////////////////
// FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

impl CssRegistry {
    fn init() -> Self {
        let window = browser::window();
        let wrapper = window.document.create_element("div");
        let local = window.document.create_element("style");
        let state = window.document.create_element("style");
        let media = window.document.create_element("style");
        local.set_attribute("ss-defaults", "");
        state.set_attribute("ss-state-and-animation", "");
        media.set_attribute("ss-media", "");
        wrapper.append_child(&local);
        wrapper.append_child(&state);
        wrapper.append_child(&media);
        let local = browser::Stylesheet::from_element(local);
        let state = browser::Stylesheet::from_element(state);
        let media = browser::Stylesheet::from_element(media);
        wrapper.set_attribute("subscript-cssom-interface", "");
        wrapper.set_attribute("style", "display: none;");
        window.document.body.insert_adjacent_element(AdjacentPosition::AfterBegin, &wrapper);
        CssRegistry::Live(LiveCssRegistry {
            added: HashSet::new(),
            mount: CssMount {wrapper,local,state,media},
        })
    }
    fn get_live(&mut self) -> &mut LiveCssRegistry {
        match self {
            CssRegistry::Live(live) => live,
            CssRegistry::Pending => {
                *self = CssRegistry::init();
                match self {
                    CssRegistry::Live(live) => live,
                    CssRegistry::Pending => panic!()
                }
            }
        }
    }
    fn notify(&mut self, styling: &Styling) {
        let hash = calculate_hash(&styling);
        let live = self.get_live();
        let already_added = live.added.contains(&hash);
        if !already_added {
            console!("new stylesheet");
            insert_styling(styling, hash, &live.mount);
        }
    }
}

fn styling_env(styling: &Styling) -> StylingEnv {
    // SETUP
    let hash = calculate_hash(&styling);
    let mut env = StylingEnv {
        css_id: hash,
        animation_ids: Vec::new(),
    };
    for animation in styling.animations.iter() {
        let aid = calculate_hash(&animation);
        env.animation_ids.push(aid);
    }
    env
}

fn insert_styling(styling: &Styling, hash: u64, mount: &CssMount) {
    // HELPERS
    let to_properties = |xs: &Vec<Style>| -> css::Properties {
        let xs = xs
            .iter()
            .map(|x| -> css::Property {
                css::Property{
                    property: x.property.clone(),
                    value: x.value.clone(),
                }
            })
            .collect::<Vec<_>>();
        css::Properties(xs)
    };
    // DEFAULT
    let default = css::Declaration {
        selector: format!(".cid-{}", hash),
        properties: to_properties(&styling.default.0),
    };
    mount.local.push_declaration(default);
    // STATE-SELECTOR
    for state in styling.state.iter() {
        let state = css::Declaration {
            selector: format!(".cid-{}{}", hash, state.name.as_str()),
            properties: to_properties(&state.body.0),
        };
        mount.state.push_declaration(state);
    }
    // ANIMATION
    for animation in styling.animations.iter() {
        let aid = calculate_hash(&animation);
        let keyfrmaes = animation.0
            .iter()
            .map(|keyframe| -> css::KeyframeInterval {
                css::KeyframeInterval {
                    value: keyframe.value.clone(),
                    style: to_properties(&keyframe.style.0),
                }
            })
            .collect::<Vec<_>>();
        let keyfrmaes = css::Keyframes {
            name: format!("cid-{}-{}", hash, aid),
            keyframes: keyfrmaes,
        };
        mount.state.push_keyframes(keyfrmaes);
    }
    // MEDIA
    for media in styling.media.iter() {
        let media = css::Media {
            condition: to_properties(&media.condition.0).0,
            declarations: vec![css::Declaration {
                selector: format!(".cid-{}", hash),
                properties: to_properties(&media.body.0),
            }],
        };
        mount.media.push_media(media);
    }
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}