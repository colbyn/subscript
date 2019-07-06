use core::marker::PhantomData;
use std::rc::*;
use std::any::*;
use std::cell::*;
use std::collections::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsValue, JsCast};
use js_sys::Function;
use uuid::Uuid;

use crate::backend::browser;
use crate::view_sys::dsl::View;
use crate::program_sys::instances::TickEnv;
use crate::program_sys::spec::Spec;
use crate::program_sys::effect::nav::*;


///////////////////////////////////////////////////////////////////////////////
// SHELL
///////////////////////////////////////////////////////////////////////////////


/// It’s a reincarnated-bourne-again shell for your everyday web-app
/// needs. :)
///
/// User-level commands are exposed or rather implemented as methods on
/// the `Shell` type (so from your docs navigate to “methods” section).
pub struct Shell<S: Spec> {
    pub(crate) instance_name: String,
    pub(crate) commands: RefCell<VecDeque<Command>>,
    pub(crate) mark: PhantomData<S>,
    pub(crate) http_client: HttpClient<S>,
}

pub(crate) enum Command {
    Save,
    Message(SystemMessage),
    Navigate(String),
}


impl<S: Spec + 'static> Shell<S> {
    // pub fn save(&mut self) {
    //     self.commands.borrow_mut().push_back(Command::Save);
    // }

    /// Heterogeneous value broadcasting system.
    /// ```
    /// // lets broadcast some random messages (values) of different types
    /// sh.broadcast(SomeType(...));
    /// sh.broadcast(SomeOtherType(...));
    /// sh.broadcast(UrlRequest(Page::Something));
    /// ```
    pub fn broadcast(&mut self, msg: impl Any) {
        self.commands.borrow_mut().push_back(Command::Message(
            SystemMessage::Public {
                from_name: self.instance_name.clone(),
                from_tid: TypeId::of::<S>(),
                value: Rc::new(msg),
            }
        ));
    }
    /// Heterogeneous "component-to-component" value messaging system.
    /// ```
    /// // This is perhaps impossible without types hehe
    /// sh.message::<SomeComponentType>(message_value);
    /// ///         ^^^^^^^^^^^^^^^^^^^ sent a message to any component of the given type.
    /// ```
    pub fn message<T: Spec + 'static, V: Any>(&mut self, msg: V) {
        let from_tid = TypeId::of::<S>();
        let to_tid = TypeId::of::<T>();
        self.commands.borrow_mut().push_back(Command::Message(SystemMessage::Private {
            from_name: self.instance_name.clone(),
            from_tid,
            to_tid,
            value: Rc::new(msg)
        }));
    }
    /// Update the browser’s URL (i.e. for SPAs).
    /// ```
    /// sh.navigate("/account");
    /// ```
    pub fn navigate(&mut self, path: impl UrlString) {
        self.commands.borrow_mut().push_back(Command::Navigate(path.url_string()));
    }
    /// Returns the current URL.
    pub fn current_url(&self) -> Url {
        Url::get_current(&browser::window())
    }
    /// Caching support.
    pub fn cache(&self) -> Cache {
        Cache(())
    }
    /// Make http requests (e.g. for interacting with backend API services).
    pub fn http_client(&self) -> &HttpClient<S> {
        &self.http_client
    }
    pub(crate) fn tick(&self, tick_env: &mut TickEnv<S::Msg>) where S: 'static {
        tick_env.local_messages.append(
            &mut self.http_client.local_queue
                .borrow_mut()
                .drain(..)
                .collect::<Vec<_>>()
        );
    }
}

///////////////////////////////////////////////////////////////////////////////
// CACHE
///////////////////////////////////////////////////////////////////////////////

pub struct Cache(pub(crate) ());

impl Cache {
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        browser::window()
            .local_storage
            .get::<T>(key)
    }
    pub fn insert<T: Serialize>(&self, key: &str, value: &T) {
        browser::window()
            .local_storage
            .set::<T>(key, value);
    }
    pub fn remove(&self, key: &str) {
        browser::window()
            .local_storage
            .remove(key);
    }
}

///////////////////////////////////////////////////////////////////////////////
// HTTP-CLIENT
///////////////////////////////////////////////////////////////////////////////

pub struct HttpClient<S: Spec> {
    pub(crate) mark: PhantomData<S>,
    pub(crate) local_queue: Rc<RefCell<VecDeque<S::Msg>>>,
}

impl<S: Spec> HttpClient<S> {
    pub fn send(
        &self,
        request: impl ToHttpRequest,
        f: impl Fn(HttpResponse) -> S::Msg + 'static,
    ) -> Result<(), ()> where S::Msg: 'static {
        // HELPERS
        fn parse_headers(value: String) -> Vec<(String, String)> {
            value
                .split("\r\n")
                .map(|line| -> (String, String) {
                    let pos = line
                        .chars()
                        .position(|x| {
                            x == ':'
                        })
                        .expect("missing colon");
                    let (x, y) = line.split_at(pos);
                    let x = String::from(x);
                    let y = String::from(y.trim_start_matches(":"));
                    (x, y)
                })
                .collect::<Vec<_>>()
        }
        // SETUP
        let HttpRequest{url,method,headers,body} = request.to_http_request();
        let mut request = web_sys::XmlHttpRequest::new().expect("new XmlHttpRequest failed");
        let method = method.unwrap_or(String::from("GET"));
        request.open(&method, &url);
        for (k, v) in headers {
            request.set_request_header(&k, &v).expect("XmlHttpRequest.setRequestHeader() failed");
        }
        let onload_callback = Closure::once_into_js({
            let local_queue = self.local_queue.clone();
            let request = request.clone();
            move |value: JsValue| {
                let request = request;
                let local_queue = local_queue;
                let response_text = request
                    .response_text()
                    .expect("XmlHttpRequest.responseText getter failed");
                let response_status = request
                    .status()
                    .expect("XmlHttpRequest.status getter failed");
                let response_headers = request
                    .get_all_response_headers()
                    .expect("XmlHttpRequest.getAllResponseHeaders() failed");
                console!("headers: {:#?}", &response_headers);
                let response_headers = {
                    if response_headers.is_empty() {
                        Default::default()
                    } else {
                        parse_headers(response_headers)
                    }
                };
                let response = HttpResponse {
                    status: response_status,
                    headers: response_headers,
                    body: response_text.unwrap_or(Default::default()),
                };
                local_queue
                    .borrow_mut()
                    .push_back(f(response));
            }
        });
        let onload_callback: js_sys::Function = From::from(onload_callback);
        request.set_onloadend(Some(&onload_callback));
        // SEND
        if let Some(body) = body {
            request.send_with_opt_str(Some(&body)).expect("XmlHttpRequest.send method failed");
        } else {
            request.send().expect("XmlHttpRequest.send method failed");
        }
        // DONE
        Ok(())
    }
    pub fn send_ext(
        &self,
        custom: impl HttpClientExt + 'static
    ) -> Result<(), ()> where S::Msg: 'static {
        // HELPERS
        fn parse_headers(value: String) -> Vec<(String, String)> {
            value
                .split("\r\n")
                .map(|line| -> (String, String) {
                    let pos = line
                        .chars()
                        .position(|x| {
                            x == ':'
                        })
                        .expect("missing colon");
                    let (x, y) = line.split_at(pos);
                    let x = String::from(x);
                    let y = String::from(y.trim_start_matches(":"));
                    (x, y)
                })
                .collect::<Vec<_>>()
        }
        // SETUP
        let HttpRequest{url,method,headers,body} = custom.to_http_request();
        let mut request = web_sys::XmlHttpRequest::new().expect("new XmlHttpRequest failed");
        let method = method.unwrap_or(String::from("GET"));
        request.open(&method, &url);
        for (k, v) in headers {
            request.set_request_header(&k, &v).expect("XmlHttpRequest.setRequestHeader() failed");
        }
        let onload_callback = Closure::once_into_js({
            let local_queue = self.local_queue.clone();
            let request = request.clone();
            move |value: JsValue| {
                let request = request;
                let local_queue = local_queue;
                let response_text = request
                    .response_text()
                    .expect("XmlHttpRequest.responseText getter failed");
                let response_status = request
                    .status()
                    .expect("XmlHttpRequest.status getter failed");
                let response_headers = request
                    .get_all_response_headers()
                    .expect("XmlHttpRequest.getAllResponseHeaders() failed");
                console!("headers: {:#?}", &response_headers);
                let response_headers = {
                    if response_headers.is_empty() {
                        Default::default()
                    } else {
                        parse_headers(response_headers)
                    }
                };
                let response = HttpResponse {
                    status: response_status,
                    headers: response_headers,
                    body: response_text.unwrap_or(Default::default()),
                };
                register_message(SystemMessage::Public {
                    from_name: String::from(""),
                    from_tid: TypeId::of::<()>(),
                    value: custom.on_reply(response),
                });
            }
        });
        let onload_callback: js_sys::Function = From::from(onload_callback);
        request.set_onloadend(Some(&onload_callback));
        // SEND
        if let Some(body) = body {
            request.send_with_opt_str(Some(&body)).expect("XmlHttpRequest.send method failed");
        } else {
            request.send().expect("XmlHttpRequest.send method failed");
        }
        // DONE
        Ok(())
    }
}

pub trait HttpClientExt : ToHttpRequest {
    fn on_reply(&self, value: HttpResponse)-> Rc<Any>;
}

pub trait ToHttpRequest {
    fn to_http_request(&self) -> HttpRequest;
}

impl ToHttpRequest for HttpRequest {
    fn to_http_request(&self) -> HttpRequest {
        self.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HttpRequest {
    pub url: String,
    pub method: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}



///////////////////////////////////////////////////////////////////////////////
// TIMEOUT
///////////////////////////////////////////////////////////////////////////////

// struct Timeouts<Msg>(Vec<Timeout<Msg>>);
// struct Timeout<Msg>{
//     triggered: RefCell<bool>,
//     name: Option<String>,
//     on_timeout: Box<Fn()->Msg>,
// }


///////////////////////////////////////////////////////////////////////////////
// GLOABL MESSAGES
///////////////////////////////////////////////////////////////////////////////
thread_local! {
    pub(crate) static GLOABL_MESSAGE_REGISTRY: RefCell<VecDeque<SystemMessage>> = {
        RefCell::new(VecDeque::new())
    };
}

#[derive(Debug, Clone)]
pub(crate) enum SystemMessage {
    Public {
        from_name: String,
        from_tid: TypeId,
        value: Rc<dyn Any>,
    },
    Private {
        from_name: String,
        from_tid: TypeId,
        to_tid: TypeId,
        value: Rc<dyn Any>,
    },
}

impl SystemMessage {
    pub fn is_private(&self) -> Option<TypeId> {
        match self {
            SystemMessage::Private{to_tid, ..} => Some(to_tid.clone()),
            _ => None
        }
    }
    pub(crate) fn value(&self) -> Rc<dyn Any> {
        match self {
            SystemMessage::Private{value, ..} => value.clone(),
            SystemMessage::Public{value, ..} => value.clone(),
        }
    }
    pub(crate) fn from_name(&self) -> String {
        match self {
            SystemMessage::Private{from_name, ..} => from_name.clone(),
            SystemMessage::Public{from_name, ..} => from_name.clone(),
        }
    }
    pub(crate) fn from_tid(&self) -> TypeId {
        match self {
            SystemMessage::Private{from_tid, ..} => from_tid.clone(),
            SystemMessage::Public{from_tid, ..} => from_tid.clone(),
        }
    }
    pub(crate) fn sender_is_receiver<T: Spec + 'static>(&self, this_name: &str) -> bool {
        let this_tid = TypeId::of::<T>();
        let this_name = String::from(this_name);
        (self.from_name() == this_name) && (self.from_tid() == this_tid)
    }
}



///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS HELPERS
///////////////////////////////////////////////////////////////////////////////

pub(crate) fn process_system_requests<S: Spec + 'static>(name: &str, model: &S::Model, sys: &mut Shell<S>) {
    for msg in sys.commands.borrow_mut().drain(..) {
        match msg {
            Command::Save => {
                // save_model::<S>(name, model);
                unimplemented!()
            }
            Command::Message(msg) => {
                register_message(msg);
            }
            Command::Navigate(nav) => {
                navigate(nav.as_str());
                crate::program_sys::CURRENT_URL.with(|cell| {
                    let new_url = Url::get_current(&browser::window());
                    cell.replace(Some(new_url));
                });
            }
        }
    }
}

pub(crate) fn spec_key<S: Spec + 'static>(name: &str) -> String {
    let tid = TypeId::of::<S>();
    format!("{:?}-{}", tid, name)
}

pub(crate) fn save_model<S: Spec + 'static>(name: &str, model: &S::Model) {
    unimplemented!()
    // browser::window()
    //     .local_storage
    //     .set::<S::Model>(&spec_key::<S>(name), model);
}

pub(crate) fn load_saved_model<S: Spec + 'static>(name: &str) -> Option<S::Model> {
    unimplemented!()
    // browser::window()
    //     .local_storage
    //     .get::<S::Model>(&spec_key::<S>(name))
}

pub(crate) fn register_message(msg: SystemMessage) {
    GLOABL_MESSAGE_REGISTRY.with(move |reg| {
        reg.borrow_mut().push_back(msg);
    });
}

pub(crate) fn navigate(route: &str) {
    browser::window()
        .history
        .push_state(route);
}

// pub(crate) fn http_request()
