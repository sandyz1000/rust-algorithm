use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};

fn ok() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}

// START_SPAN method_not_allowed
fn method_not_allowed<S: AsRef<str>>(methods: S) -> Response<Body> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .header(header::ALLOW, methods.as_ref())
        .body(Body::empty())
        .unwrap()
}
// END_SPAN method_not_allowed

// START_SPAN index
fn index(req: &Request<Body>) -> Response<Body> {
    if !matches!(req.method(), &Method::GET | &Method::HEAD) {
        return method_not_allowed("GET, HEAD");
    }

    ok()
}
// END_SPAN index

fn handle(req: Request<Body>) -> Response<Body> {
    match req.uri().path() {
        "/" => index(&req),
        _ => not_found(),
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    eprintln!("Listening on {}", addr);

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req| async {
            Ok::<_, Infallible>(handle(req))
        }))
    });

    if let Err(e) = Server::bind(&addr).serve(make_service).await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
