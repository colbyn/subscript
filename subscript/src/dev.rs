use std::cell::*;
use serde::{Serialize, Deserialize};

use ss_web_utils::js::console;

use ss_view_tree::{View, Mixin};
use ss_view_tree::attributes::*;
use ss_view_tree::styling::*;
use ss_view_tree::styling::selectors::*;
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
pub type EntryId = u16;

#[derive(Debug, PartialEq, Clone)] // REQUIRED
pub enum Msg {
    NoOp,
    ToggleAll,
    NewEntryName(String),
    SubmitNewEntryName,
    RemoveEntry(EntryId, EntryIx),
    EntryCompleted(EntryId, EntryIx, bool),
    EntryMouseEnter(EntryId, EntryIx),
    EntryMouseLeave(EntryId, EntryIx),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)] // REQUIRED
#[derive(Default)] // OPTIONAL
pub struct Model {
    toggle_all: bool,
    new_entry_name: String,
    entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Entry {
    id: EntryId,
    name: String,
    completed: bool,
    mouse_hovering: bool,
}


///////////////////////////////////////////////////////////////////////////////
// APP - IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct AppSpec {}

impl Spec for AppSpec {
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
            Msg::ToggleAll => {
                model.toggle_all = !model.toggle_all;
                for entry in model.entries.iter_mut() {
                    entry.completed = model.toggle_all;
                }
            }
            Msg::NewEntryName(str) => {model.new_entry_name = str;}
            Msg::SubmitNewEntryName => {
                if !model.new_entry_name.is_empty() {
                    let id = rand::random::<u16>();
                    let name = model.new_entry_name.drain(..).collect::<String>();
                    let entry = Entry {id, name, completed: false, mouse_hovering: false};
                    model.entries.push(entry);
                }
            }
            Msg::EntryCompleted(id, ix, toggle) => {
                let mut entry = model.entries.get_mut(ix).expect("missing entry");
                assert!(entry.id == id);
                entry.completed = toggle;
            }
            Msg::RemoveEntry(id, ix) => {
                let mut entry = model.entries.get_mut(ix).expect("missing entry");
                assert!(entry.id == id);
                model.entries.remove(ix);
            }
            Msg::EntryMouseEnter(id, ix) => {
                let mut entry = model.entries.get_mut(ix).expect("missing entry");
                assert!(entry.id == id);
                entry.mouse_hovering = true;
            }
            Msg::EntryMouseLeave(id, ix) => {
                let mut entry = model.entries.get_mut(ix).expect("missing entry");
                assert!(entry.id == id);
                entry.mouse_hovering = false;
            }
        }
    }
    fn view(&self, model: &Self::Model) -> View<Self::Msg> {v!{
        height: "100%";
        grid_template_rows: "100px min-content";
        display: "grid";
        background_color: "#e8e8e8";
        grid_template_columns: "min-content";
        justify_content: "center";
        h1 {
            header_styling();
            "todos";
        }
        main {
            box_shadow: "0 2px 4px 0 rgba(0, 0, 0, 0.2), 0 25px 50px 0 rgba(0, 0, 0, 0.1)";
            display: "flex";
            flex_direction: "column";
            new_todo_form(model);
            ul {
                text_styling();
                padding: "0";
                margin: "0";
                list_style: "none";
                model.entries
                    .iter()
                    .enumerate()
                    .map(move |(ix, entry)| todo_entry(ix, entry))
                    .collect::<Vec<_>>();
            }
        }
    }}
}

///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

fn text_styling<Msg>() -> Mixin<Msg> {mix!{
    font_family: "Source Sans Pro";
    color: "#6d6d6d";
}}

fn header_styling<Msg>() -> Mixin<Msg> {mix!{
    text_styling();
    margin: "0";
    padding: "0";
    display: "flex";
    justify_content: "center";
    align_items: "center";
    font_size: "4em";
    font_weight: "300";
    letter_spacing: "0.04em";
}}

fn new_todo_form(model: &Model) -> View<Msg> {v!{
    min_width: "500px";
    display: "flex";
    background_color: "#e8e8e8";
    button {
        padding: "12px";
        background_color: "transparent";
        border: "none";
        font_size: "2em";
        outline: "none";
        color: "#676767";
        on_click(move || Msg::ToggleAll);
        if (!model.toggle_all) {
            i {class = "far fa-check-circle";}
        };
        if (model.toggle_all) {
            i {class = "fas fa-check-circle";}
        };
    }
    form {
        display: "contents";
        on_submit(|| Msg::SubmitNewEntryName);
        input {
            text_styling();
            padding: "12px 12px 12px 0";
            padding_left: "0px";
            background_color: "transparent";
            border: "none";
            font_size: "2em";
            outline: "none";
            width: "100%";
            font_weight: "100";
            :placeholder {
                font_size: "1em";
                font_variant: "petite-caps";
            };
            type = "text";
            value = model.new_entry_name.as_str();
            placeholder = "What needs to be done?";
            on_input(move |str| Msg::NewEntryName(str));
        }
    }
}}

fn todo_entry(ix: EntryIx, entry: &Entry) -> View<Msg> {v!{li|form {
    padding: "10px";
    extend!(on_mouse_enter, [id@entry.id], move || Msg::EntryMouseEnter(id, ix));
    extend!(on_mouse_leave, [id@entry.id], move || Msg::EntryMouseLeave(id, ix));
    input {
        extend!(on_check, [id@entry.id], move |x| Msg::EntryCompleted(id, ix, x));
        type = "checkbox";
        checked = entry.completed;
    }
    label {
        entry.name.as_str();
    }
    button {
        extend!(on_click, [id@entry.id], move || Msg::RemoveEntry(id, ix));
        if (entry.mouse_hovering) {
            visibility: "visible";
        };
        if (!entry.mouse_hovering) {
            visibility: "hidden";
        };
        i {class = "fas fa-times";}
    }
}}}


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