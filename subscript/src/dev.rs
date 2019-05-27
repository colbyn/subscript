use std::cell::*;
use serde::{Serialize, Deserialize};

use ss_web_utils::js::console;

use ss_view_tree::View;
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
use ss_css_types::api::*;
use crate::css::{common::*, everything as css};


#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    NoOp,
    SetCounter(i32),
    Test(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Model {
    counter: i32,
    test: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct AppSpec {

}

// impl<'a> Spec<'a> for AppSpec {
//     type Model = Model;
//     type Msg = Msg;

//     fn init(&self, startup: StartupInfo<'a, Self>) -> Init<'a, Self> {
//     	Init {
//     		model: Model::default(),
//     		subs: Subscriptions::default(),
//     	}
//     }
//     fn update(&self, model: &'a mut Self::Model, msg: Self::Msg, sys: &SubSystems<Self>) {
//         match msg {
//             Msg::NoOp => {}
//             Msg::SetCounter(x) => {
//                 model.counter = x;
//             }
//             Msg::Test(_) => {}
//         }
//     }
//     fn view(&self, model: &'a Self::Model) -> View<'a, Self::Msg> {
//         use crate::css::everything::*;
//         v!{
//             h1 {
//                 format!("{}", model.counter);
//             }
//             button {
//                 on_click({
//                     move || {Msg::SetCounter(model.clone().counter + 1)}
//                 });
//                 "Set";
//             }
//             main {
//                 p {
//                     "Lorem Ipsim";
//                 }
//             }
//             footer {
//                 // on_click(move || {Msg::Test(model.test.clone())});
//                 // format!("{}", model.counter);
//             }
//     	}
//     }
// }


pub fn main() {
	// let program = Program::from_component(Component {
	// 	name: "app",
	// 	spec: AppSpec::default(),
	// });
	// program.start();
}