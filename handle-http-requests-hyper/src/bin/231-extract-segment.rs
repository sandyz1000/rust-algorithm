use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Request, Response, Server, StatusCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

fn ok_with_text<S: Into<String>>(text: S) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from(text.into()))
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

fn internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap()
}

macro_rules! allow_method {
    ($l:expr, $($r:pat)|+) => {{
        use hyper::Method;

        if !matches!($l, $($r)|+) {
            let mut allowed = Vec::new();

            for method in &[
                Method::GET,
                Method::HEAD,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
            ] {
                if matches!(method, $($r)|+) {
                    allowed.push(method.as_str());
                }
            }

            return Ok(self::method_not_allowed(allowed.join(", ")));
        }
    }};
}

fn hello(req: &Request<Body>, name: &str) -> Result<Response<Body>> {
    allow_method!(req.method(), &Method::GET | &Method::HEAD);

    Ok(ok_with_text(format!("Hello, {}!", name)))
}

// START_SPAN route
fn route(req: &Request<Body>, segments: &[&str]) -> Result<Response<Body>> {
    match segments {
        ["hello"] => hello(req, "World"),
        ["hello", name] => hello(req, name),
        _ => Ok(not_found()),
    }
}
// END_SPAN route

fn handle(req: Request<Body>) -> Response<Body> {
    // Extract the segments from the URI path.
    let path = req.uri().path().to_owned();
    let segments: Vec<&str> =
        path.split('/').filter(|s| !s.is_empty()).collect();

    route(&req, &segments).unwrap_or_else(|err| {
        eprintln!("Error: {:#}", err);
        internal_server_error()
    })
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
