use futures::future;
use hyper::{Method, StatusCode};
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;



fn endpoints(req: Request<Body>) -> Response<Body> {
    let mut response = Response::new(Body::empty());
    let route = req.uri().path().split("/").collect::<Vec<&str>>();
    let method = req.method();
    let mut found = false;
    if route == ["account"] {
        if method == Method::POST {
            found = true;
            unimplemented!();
        } else if method == Method::GET {
            found = true;
            unimplemented!();
        } else if method == Method::DELETE {
            found = true;
            unimplemented!();
        }
    } else if route == ["account", "is-taken"] {
        if method == Method::POST {
            found = true;
            unimplemented!();
        }
    } else if route == ["account", "user"] {
        if method == Method::POST {
            found = true;
            unimplemented!();
        } else if method == Method::DELETE {
            found = true;
            unimplemented!();
        }
    } else if route == ["account", "user", "token"] {
        if method == Method::POST {
            found = true;
            unimplemented!();
        }
    } else if route == ["account", "user", "password"] {
        if method == Method::PUT {
            found = true;
            unimplemented!();
        } else {
            found = true;
            unimplemented!();
        }
    }
    if !found {
        *response.body_mut() = Body::from("not found");
        *response.status_mut() = StatusCode::NOT_FOUND;
    }
    response
}

pub fn run() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| {
            service_fn_ok(endpoints)
        })
        .map_err(|e| eprintln!("server error: {}", e));
    hyper::rt::run(server);
}

