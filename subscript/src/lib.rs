#![allow(dead_code, unused)]
//! <b>[Examples:](https://github.com/colbyn/subscript/tree/master/examples)</b>
//! * [Client <-> Server](https://github.com/colbyn/subscript/tree/master/examples/client-server)
//! * [Counter App](https://github.com/colbyn/subscript/tree/master/examples/counter-app)
//! * [Nav App](https://github.com/colbyn/subscript/tree/master/examples/nav-app) (currently perhaps the best example feature-wise)
//! * [Saas App](https://github.com/colbyn/subscript/tree/master/examples/saas-app)
//! * [Todo App](https://github.com/colbyn/subscript/tree/master/examples/todo-app)
//!
//! # [Counter Component Preview:](https://github.com/colbyn/subscript/tree/master/examples/counter-app)
//! ```
//! impl Spec for AppSpec {
//!     type Msg = Msg;
//!     type Model = Model;
//! 
//!     fn init(&self, sh: &Shell<Self>) -> Init<Self> {
//!         Init{
//!             ..Default::default()
//!         }
//!     }
//!     fn update(&self, model: &mut Model, msg: Msg, sh: &mut Shell<Self>) {
//!         match msg {
//!             Msg::NoOp => {}
//!             Msg::Increment => {
//!                 let current = model.counter.get_copy();
//!                 model.counter.set(current + 1);
//!             }
//!             Msg::Decrement => {
//!                 let current = model.counter.get_copy();
//!                 model.counter.set(current - 1);
//!             }
//!         }
//!     }
//!     fn view(&self, model: &Model) -> View<Msg> {v1!{
//!         display: "flex";
//!         flex_direction: "column";
//!         display: "flex";
//!         flex_direction: "column";
//!         max_width: "600px";
//!         margin: "0 auto";
//!         padding_top: "30px";
//! 
//!         css.media[max_width: "600px"] => s1!{
//!             padding: "0 10px";
//!         };
//! 
//!         h1 !{
//!             text_theme();
//!             margin: "0";
//!             text_align: "center";
//!             font_size: "6em";
//!             margin_bottom: "10px";
//!             color: "#777";
//!             font_weight: "700";
//!             transition: "1s";
//!             css.hover => s1!{
//!                 font_size: "8em";
//!                 color: "#00fdde";
//!             };
//!             model.counter.map(|x| format!("{}", x));
//!         };
//!         button !{
//!             text_theme();
//!             outline: "none";
//!             user_select: "none";
//!             padding: "4px";
//!             font_size: "2em";
//!             border: "none";
//!             border_radius: "3px";
//!             background_color: "#565656";
//!             color: "#fff";
//!             margin_bottom: "10px";
//!             event.click[] => move || Msg::Increment;
//!             "Increment";
//!         };
//!         button !{
//!             text_theme();
//!             outline: "none";
//!             user_select: "none";
//!             padding: "4px";
//!             font_size: "2em";
//!             border: "none";
//!             border_radius: "3px";
//!             background_color: "#565656";
//!             color: "#fff";
//!             event.click[] => move || Msg::Decrement;
//!             "Decrement";
//!         };
//!     }}
//! }
//! ```

#[macro_use]
pub mod backend;
#[macro_use]
pub mod view_sys;
#[macro_use]
pub mod program_sys;
// pub mod dev;
pub mod reactive_sys;
pub mod prelude;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn main() -> Result<(), wasm_bindgen::JsValue> {
//     console_error_panic_hook::set_once();
//     console!("started");
//     // dev::cms_app::client::setup();
//     // dev::todo_app::setup();
//     Ok(())
// }

// #[wasm_bindgen]
// pub fn tick() {
//     // dev::cms_app::client::tick();
//     // dev::todo_app::tick();
// }






