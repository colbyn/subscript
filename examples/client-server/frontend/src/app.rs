use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};
use subscript::prelude::*;



///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppSpec {

}

pub enum Msg {
    NoOp,
    ServerReply(String),
    Clicked,
}

#[derive(Default)]
pub struct Model {
    replies: VecSignal<Reply>,
}

pub struct Reply(String);



///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
// VIEWABLE DATA TYPES
///////////////////////////////////////////////////////////////////////////////

impl Viewable<Msg> for Reply {
    fn view(&self) -> View<Msg> {v1!{
        li !{
            text_theme();
            &self.0;
        };
    }}
}



///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Spec for AppSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, startup: &Shell<Self>) -> Init<Self> {
        Init{
            ..Default::default()
        }
    }
    fn update(&self, model: &mut Model, msg: Msg, sh: &mut Shell<Self>) {
        match msg {
            Msg::NoOp => {}
            Msg::ServerReply(value) => {
                model.replies.push(Reply(value));
            }
            Msg::Clicked => {
                use futures::future::Future;
                use subscript::program_sys::tasks::http::{Request, Response, ToHttpRequest};
                let req = Request {
                    url: String::from("http://127.0.0.1:3000"),
                    ..Request::default()
                };
                sh.task(req
                    .send()
                    .map(|response: Response| -> Msg {
                        let body = response.body.clone();
                        Msg::ServerReply(body)
                    }));
            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {v1!{
        button !{
            event.click[] => move || Msg::Clicked;
            "Click Me";
        };
        server_replies(model);
    }}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////

fn server_replies(model: &Model) -> View<Msg> {v1!{
    div !{
        h1 !{
            text_theme();
            "Server Replies:";
        };
        ul !{
            &model.replies;
        };
    };
}}

///////////////////////////////////////////////////////////////////////////////
// VIEW AGNOSTIC UTILS
///////////////////////////////////////////////////////////////////////////////

pub fn text_theme<Msg: 'static>() -> View<Msg> {v1!{
    font_family: "'Source Sans Pro', sans-serif";
    font_weight: "200";
}}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

pub fn setup() {
    Program::run_spec(AppSpec{

    });
}

pub fn tick() {
    subscript::prelude::on_request_animation_frame();
}
