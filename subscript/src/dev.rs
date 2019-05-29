use std::cell::*;
use serde::{Serialize, Deserialize};

use ss_web_utils::js::console;

use ss_view_tree::View;
use ss_view_tree::attributes::*;
use ss_view_tree::styling::*;
use ss_view_tree::styling::selectors::{
    media,
    active,
    after,
    before,
    checked,
    disabled,
    empty,
    enabled,
    first_child,
    first_letter,
    first_line,
    focus,
    hover,
    last_child,
    only_child,
    link,
    visited,
    spelling_error,
    grammar_error,
    selection,
    placeholder,
    marker,
    cue,
    backdrop,
};
use ss_view_tree::events::EventHandler;
use ss_view_tree::events::{
    on_click,
    on_mouse_down,
    on_mouse_up,
    on_mouse_enter,
    on_mouse_leave,
    on_mouse_over,
    on_mouse_out,
    on_input,
    on_check,
    on_submit,
    on_blur,
    on_focus,
};
use ss_program::Subscriptions;
use ss_program::StartupInfo;
use ss_program::Init;
use ss_program::SubSystems;
use ss_program::Component;
use ss_program::Program;
use ss_program::Spec;
// use ss_css_types::api::*;
// use crate::css::{common::*, everything as css};


///////////////////////////////////////////////////////////////////////////////
// APP - INTERNAL DATA TYPES
///////////////////////////////////////////////////////////////////////////////

pub type EntryIx = usize;

#[derive(Debug, PartialEq, Clone)] // REQUIRED
pub enum Msg {
    NoOp,
    NewEntryName(String),
    SubmitNewEntryName,
    EntryCompleted(EntryIx, bool),
    EntryMouseOver(EntryIx),
    EntryMouseOut(EntryIx),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)] // REQUIRED
#[derive(Default)] // OPTIONAL
pub struct Model {
    new_entry_name: String,
    entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)] // REQUIRED
#[derive(Default)] // OPTIONAL
pub struct Entry {
    name: String,
    completed: bool,
    mouse_over: bool,
}


///////////////////////////////////////////////////////////////////////////////
// APP - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct AppSpec {}

impl Spec for AppSpec {
    /// See 'Spec' docs for required derive/trait implementations.
    type Model = Model;
    type Msg = Msg;

    fn init(&self, startup: StartupInfo<Self>) -> Init<Self> {
    	Init {
    		model: Model::default(),
    		subs: Subscriptions::default(),
    	}
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, sys: &SubSystems<Self>) {
        match msg {
            Msg::NoOp => {}
            Msg::NewEntryName(str) => {model.new_entry_name = str;}
            Msg::SubmitNewEntryName => {
                if !model.new_entry_name.is_empty() {
                    let name = model.new_entry_name.drain(..).collect::<String>();
                    let entry = Entry {name, completed: false, mouse_over: false};
                    model.entries.push(entry);
                }
            }
            Msg::EntryCompleted(ix, toggle) => {
                if let Some(entry) = model.entries.get_mut(ix) {
                    entry.completed = toggle;
                }
            }
            Msg::EntryMouseOver(ix) => {
                if let Some(entry) = model.entries.get_mut(ix) {
                    entry.mouse_over = true;
                }
            }
            Msg::EntryMouseOut(ix) => {
                if let Some(entry) = model.entries.get_mut(ix) {
                    entry.mouse_over = false;
                }
            }
        }
    }
    fn view(&self, model: &Self::Model) -> View<Self::Msg> {v!{
        h1 {
            "todos";
        }
        main {
            form {
                on_submit(|| Msg::SubmitNewEntryName);
                input {
                    type = "text";
                    value = model.new_entry_name.as_str();
                    on_input(move |str| Msg::NewEntryName(str));
                }
            }
            ul {
                list_style: "none";
                model.entries
                    .iter()
                    .enumerate()
                    .map(move |(ix, entry)| render_entry(ix, entry))
                    .collect::<Vec<_>>();
            }
        }
    }}
}

///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

fn render_entry(ix: usize, entry: &Entry) -> View<Msg> {v!{li|
    form {
        on_mouse_over(move || Msg::EntryMouseOver(ix));
        on_mouse_out(move || Msg::EntryMouseOut(ix));
        input {
            on_check(move |toggle| Msg::EntryCompleted(ix, toggle));
            type = "checkbox";
            checked = entry.completed;
        }
        label {
            entry.name.as_str();
        }
        button {
            // if (!entry.mouse_over) {
            //     display: "none";
            // };
            i {class = "fas fa-times";}
        }
    }
}}


///////////////////////////////////////////////////////////////////////////////
// MAIN
///////////////////////////////////////////////////////////////////////////////

pub fn main() {
	let program = Program::from_component(Component {
		name: "app",
		spec: AppSpec::default(),
	});
	program.start();
}