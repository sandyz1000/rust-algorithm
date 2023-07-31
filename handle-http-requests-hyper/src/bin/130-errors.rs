use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};

// START_SPAN types
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;
// END_SPAN types

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

fn method_not_allowed<S: AsRef<str>>(methods: S) -> Response<Body> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .header(header::ALLOW, methods.as_ref())
        .body(Body::empty())
        .unwrap()
}

// START_SPAN internal_server_error
fn internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap()
}
// END_SPAN internal_server_error

// START_SPAN index
fn index(req: &Request<Body>) -> Result<Response<Body>> {
    if !matches!(req.method(), &Method::GET | &Method::HEAD) {
        return Ok(method_not_allowed("GET, HEAD"));
    }

    Ok(ok())
}
// END_SPAN index

// START_SPAN route
fn route(req: &Request<Body>) -> Result<Response<Body>> {
    match req.uri().path() {
        "/" => index(req),
        "/error" => Err("This is an error".into()),
        _ => Ok(not_found()),
    }
}
// END_SPAN route

// START_SPAN handle
fn handle(req: Request<Body>) -> Response<Body> {
    route(&req).unwrap_or_else(|err| {
        eprintln!("Error: {:#}", err);
        internal_server_error()
    })
}
// END_SPAN handle

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
