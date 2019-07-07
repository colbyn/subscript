use std::rc::Rc;
use std::cell::RefCell;
use futures::future::Future;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsValue, JsCast};

use crate::backend::browser;


///////////////////////////////////////////////////////////////////////////////
// GENRICS
///////////////////////////////////////////////////////////////////////////////
pub trait ToHttpRequest {
    fn to_http_request(&self) -> Request;
}

impl ToHttpRequest for Request {
    fn to_http_request(&self) -> Request {
        self.clone()
    }
}


///////////////////////////////////////////////////////////////////////////////
// HTTP-REQUEST
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Request {
    pub url: String,
    pub method: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl Request {
    /// Response headers not yet set (it's empty).
    pub fn send(&self) -> impl Future<Item=Response, Error=()> {
        mk_request_future(self.clone())
    }
}


///////////////////////////////////////////////////////////////////////////////
// HTTP-RESPONSE
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}


///////////////////////////////////////////////////////////////////////////////
// INTERNAL HELPERS
///////////////////////////////////////////////////////////////////////////////

fn mk_request_future(request: impl ToHttpRequest) -> impl Future<Item=Response, Error=()> {
    use web_sys::{Request, RequestInit, RequestMode, Response};
    use wasm_bindgen_futures::JsFuture;
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use serde::{Deserialize, Serialize};
    // SETUP
    let window = browser::window();
    let self::Request{url,method,headers,body} = request.to_http_request();
    let mut opts = RequestInit::new();
    opts.method(&method.unwrap_or(String::from("GET")));
    opts.mode(RequestMode::Cors);
    if let Some(body) = body {
        opts.body(Some(&JsValue::from_str(&body)));
    }
    let request = Request::new_with_str_and_init(
        &url,
        &opts,
    );
    let request = request.expect("Request.new failed");
    for (k, v) in headers {
        request
            .headers()
            .set(&k, &v)
            .expect("Request.headers.set failed");
    }
    let request_promise = window
        .instance_as_window()
        .fetch_with_request(&request);
    let response_hack: Rc<RefCell<Option<Response>>> = Rc::new(RefCell::new(None));
    JsFuture::from(request_promise)
        .and_then({
            let response_hack = response_hack.clone();
            move |resp_value: JsValue| {
                assert!(resp_value.is_instance_of::<Response>());
                let resp: Response = From::from(resp_value);
                response_hack.replace(Some(Clone::clone(&resp)));
                let body = JsFuture::from(resp.text().expect("resp.text() failed"));
                body
            }
        })
        .map(move |value: JsValue| {
            let response = response_hack.replace(None)
                .expect("response_hack failed");
            let value: String = value
                .as_string()
                .expect("Request: Response.body to string failed");
            self::Response {
                status: response.status(),
                headers: Default::default(),
                body: value,
            }
        })
        .map_err(|_| ())
}

// fn mk_old_request(request: impl ToHttpRequest) -> impl Future<Item=Response, Error=()> {
//     // HELPERS
//     fn parse_headers(value: String) -> Vec<(String, String)> {
//         value
//             .split("\r\n")
//             .map(|line| -> (String, String) {
//                 let pos = line
//                     .chars()
//                     .position(|x| {
//                         x == ':'
//                     })
//                     .expect("missing colon");
//                 let (x, y) = line.split_at(pos);
//                 let x = String::from(x);
//                 let y = String::from(y.trim_start_matches(":"));
//                 (x, y)
//             })
//             .collect::<Vec<_>>()
//     }
//     // SETUP
//     let Request{url,method,headers,body} = request.to_http_request();
//     let mut request = web_sys::XmlHttpRequest::new().expect("new XmlHttpRequest failed");
//     let method = method.unwrap_or(String::from("GET"));
//     request.open(&method, &url);
//     for (k, v) in headers {
//         request.set_request_header(&k, &v).expect("XmlHttpRequest.setRequestHeader() failed");
//     }
//     let onload_callback = Closure::once_into_js({
//         // let local_queue = self.local_queue.clone();
//         let request = request.clone();
//         move |value: JsValue| {
//             let request = request;
//             // let local_queue = local_queue;
//             let response_text = request
//                 .response_text()
//                 .expect("XmlHttpRequest.responseText getter failed");
//             let response_status = request
//                 .status()
//                 .expect("XmlHttpRequest.status getter failed");
//             let response_headers = request
//                 .get_all_response_headers()
//                 .expect("XmlHttpRequest.getAllResponseHeaders() failed");
//             console!("headers: {:#?}", &response_headers);
//             let response_headers = {
//                 if response_headers.is_empty() {
//                     Default::default()
//                 } else {
//                     parse_headers(response_headers)
//                 }
//             };
//             let response = Response {
//                 status: response_status,
//                 headers: response_headers,
//                 body: response_text.unwrap_or(Default::default()),
//             };
//             unimplemented!()
//             // local_queue
//             //     .borrow_mut()
//             //     .push_back(f(response));
//         }
//     });
//     let onload_callback: js_sys::Function = From::from(onload_callback);
//     request.set_onloadend(Some(&onload_callback));
//     // SEND
//     if let Some(body) = body {
//         request.send_with_opt_str(Some(&body)).expect("XmlHttpRequest.send method failed");
//     } else {
//         request.send().expect("XmlHttpRequest.send method failed");
//     }
//     // DONE
//     futures::future::ok(unimplemented!())
// }
