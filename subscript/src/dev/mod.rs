pub mod ui_utils;

use std::cell::*;
use serde::{Serialize, Deserialize};

use ss_web_utils::js::console;

use ss_view_tree::{View, Mixin};
use ss_view_tree::attributes::*;
use ss_view_tree::styling::*;
use ss_view_tree::styling::selectors::*;
use ss_view_tree::events::EventHandler;
use ss_view_tree::events::*;
use ss_program::Subscriptions;
use ss_program::StartupInfo;
use ss_program::Init;
use ss_program::SubSystems;
use ss_program::Component;
use ss_program::Program;
use ss_program::Spec;


///////////////////////////////////////////////////////////////////////////////
// APP - INTERNAL DATA TYPES
///////////////////////////////////////////////////////////////////////////////

pub type EntryIx = usize;
pub type EntryId = u16;

#[derive(Debug, PartialEq, Clone)] // REQUIRED
pub enum Msg {
    NoOp,
    Display(Display),
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
    display: Display,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Display {
    All,
    Active,
    Completed,
}

impl Default for Display {
    fn default() -> Self {Display::All}
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
            Msg::Display(x) => {
                model.display = x;
            }
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
        grid_template_rows: "140px min-content";
        display: "grid";
        background_color: "#e8e8e8";
        grid_template_columns: "min-content";
        justify_content: "center";
        h1 {
            header_styling();
            color: "#f7b4b4";
            "todos";
        }
        main {
            box_shadow: "0 2px 4px 0 rgba(0, 0, 0, 0.2), 0 25px 50px 0 rgba(0, 0, 0, 0.1)";
            display: "flex";
            flex_direction: "column";
            background_color: "#fff";
            new_todo_form(model);
            ul {
                text_styling();
                padding: "0";
                margin: "0";
                list_style: "none";
                model.entries
                    .iter()
                    .enumerate()
                    .filter(|(_, entry)| {
                        match model.display {
                            Display::Completed => entry.completed,
                            Display::Active => !entry.completed,
                            Display::All => true
                        }
                    })
                    .map(move |(ix, entry)| todo_entry(ix, entry))
                    .collect::<Vec<_>>();
            }
            todo_footer(model);
        }
    }}
}

///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

fn checkbox<Msg: Clone>(checked: bool, cb: impl Fn(bool)->Msg + 'static) -> View<Msg> {v!{div|
    class="pretty p-icon p-round";
    input {
        checked = checked;
        on_check(cb);
        type="checkbox";
    }
    div {
        class="state";
        i {class="icon fas fa-check";}
        label {
            "Click Me";
        }
    }
}}

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
    button {
        padding: "12px";
        background_color: "transparent";
        border: "none";
        font_size: "2em";
        outline: "none";
        color: "#cacaca";
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
    display: "flex";
    align_items: "center";
    justify_content: "flex-start";
    position: "relative";
    extend!(on_mouse_enter, [id@entry.id], move || Msg::EntryMouseEnter(id, ix));
    extend!(on_mouse_leave, [id@entry.id], move || Msg::EntryMouseLeave(id, ix));
    i {
        content: "'\\F12C'";
    }

    // checkbox(entry.completed, extend!([id@entry.id], move |x| Msg::EntryCompleted(id, ix, x)));
    ui_utils::Checkbox {
        checked: entry.completed,
        label: entry.name.as_str(),
        on_click: extend!([id@entry.id], move |x| Msg::EntryCompleted(id, ix, x)),
    };
    // input {
    //     extend!(on_check, [id@entry.id], move |x| Msg::EntryCompleted(id, ix, x));
    //     type = "checkbox";
    //     checked = entry.completed;
    // }
    button {
        outline: "none";
        position: "absolute";
        right: "0";
        width: "100px";
        border: "none";
        background: "transparent";
        display: "flex";
        justify_content: "center";
        align_items: "center";
        font_size: "1.2em";
        padding: "0";
        margin: "0";
        span {
            extend!(on_click, [id@entry.id], move || Msg::RemoveEntry(id, ix));
            if (!entry.mouse_hovering) {
                display: "none";
            };
            "x";
        }
    }
}}}

fn todo_footer(model: &Model) -> View<Msg> {
    let set_display = move |name: &str, display: Display| v!{li|
        text_styling();
        user_select: "none";
        text_align: "center";
        color: "#808080";
        a{
            if (model.display == display) {
                border: "1px solid #ef87c1";
            };
            if (model.display != display) {
                border: "1px solid transparent";
                :hover {
                    border: "1px solid #fbb5c2";
                };
            };
            padding: "0px 7px";
            border_radius: "3px";
            margin_left: "14px";
            margin_right: "14px";
            text_transform: "uppercase";
            font_size: "0.8em";
            font_weight: "400";
            extend!(on_click, [display, current@model.display], move || Msg::Display(display.clone()));
            name;
        }
    };
    v!{footer|
        padding_top: "8px";
        padding_bottom: "8px";
        align_items: "center";
        display: "grid";
        grid_template_columns: "100px 1fr 100px";
        box_shadow:
            "0 1px 1px rgba(0, 0, 0, 0.2), 0 8px 0 -3px #f6f6f6, \
            0 9px 1px -3px rgba(0, 0, 0, 0.2), \
            0 16px 0 -6px #f6f6f6, \
            0 17px 2px -6px rgba(0, 0, 0, 0.2)";
        span {
            text_styling();
            text_align: "center";
            width: "100%";
            format!("{} items left", model.entries.len());
        }
        ul {
            text_styling();
            display: "grid";
            align_items: "center";
            grid_template_columns: "1fr 1fr 1fr";
            width: "100%";
            padding: "0";
            margin: "0";
            list_style: "none";
            set_display("All", Display::All);
            set_display("Active", Display::Active);
            set_display("Completed", Display::Completed);
        }
        div {
            width: "100%";
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// VIEW - SPEC/COMPONENT AGNOSTIC REUSABLE UTILS
///////////////////////////////////////////////////////////////////////////////

pub struct Checkbox<'a> {
    checked: bool,
    label: &'a str,
    on_click: &'a (Fn(bool) -> Msg + 'static),
}

fn checkbox_<Msg: Clone>(checked: bool, cb: impl Fn(bool)->Msg + 'static) -> View<Msg> {v!{div|
    box_sizing: "border-box";
    position: "relative";
    display: "inline-block";
    margin_right: "1em";
    white_space: "nowrap";
    line_height: "1";

    input {
        box_sizing: "border-box";
        position: "absolute";
        left: "0";
        top: "0";
        min_width: "1em";
        width: "100%";
        height: "100%";
        z_index: "2";
        opacity: "0";
        margin: "0";
        padding: "0";
        cursor: "pointer";

        on_check(cb);
        type="checkbox";
    }
    div {
        box_sizing: "border-box";
        position: "relative";
        display: "inline-block";
        margin_right: "1em";
        white_space: "nowrap";
        line_height: "1";
        i {
            display: "inline-block";
            margin_right: "1em";
            white_space: "nowrap";
            line_height: "1";
            box_sizing: "border-box";
            position: "absolute";
            font_size: "1em";
            width: "calc(1em + 2px)";
            height: "calc(1em + 2px)";
            left: "0";
            z_index: "1";
            text_align: "center";
            line_height: "normal";
            top: "calc((0% - (100% - 1em)) - 8%)";
            border: "1px solid transparent";
            border_radius: "100%";
            overflow: "hidden";

            :before {
                _webkit_transform: "scale(.8)";
                _ms_transform: "scale(.8)";
                transform: "scale(.8)";
                margin: "0";
                width: "100%";
                height: "100%";
                text_align: "center";
                display: "-webkit-box";
                display: "-ms-flexbox";
                display: "flex";
                _webkit_box_flex: "1";
                _ms_flex: "1";
                flex: "1";
                _webkit_box_pack: "center";
                _ms_flex_pack: "center";
                justify_content: "center";
                _webkit_box_align: "center";
                _ms_flex_align: "center";
                align_items: "center";
                line_height: "1";
            };
            if (checked) {
                class="icon fas fa-check";
            };
        }
        label {
            box_sizing: "border-box";
            font_weight: "400";
            margin: "0";
            text_indent: "1.5em";
            min_width: "calc(1em + 2px)";
            position: "relative";
            display: "inline-block";
            margin_right: "1em";
            white_space: "nowrap";
            line_height: "1";
            text_align: "-webkit-match-parent";
            :before {
                content: "''";
                width: "calc(1em + 2px)";
                height: "calc(1em + 2px)";
                display: "block";
                box_sizing: "border-box";
                border: "1px solid transparent";
                z_index: "0";
                position: "absolute";
                left: "0";
                top: "calc((0% - (100% - 1em)) - 8%)";
                background_color: "transparent";
                border_color: "#5a656b";
                border_radius: "100%";
            };
            "Click Me";
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