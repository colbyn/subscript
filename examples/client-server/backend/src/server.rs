use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn handler(req: Request<Body>) -> Response<Body> {
    let mut response = Response::new(Body::from("hello world...."));
    response.headers_mut().insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    response
}

pub fn run_server() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| {
            service_fn_ok(handler)
        })
        .map_err(|e| eprintln!("server error: {}", e));
    println!("server running...");
    hyper::rt::run(server);
}