use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::time::Instant;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

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

fn not_implemented() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
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

// START_SPAN index
fn index(req: &Request<Body>) -> Result<Response<Body>> {
    allow_method!(req.method(), &Method::GET | &Method::HEAD);

    Ok(ok())
}
// END_SPAN index

// START_SPAN route
fn route(req: &Request<Body>, segments: &[&str]) -> Result<Response<Body>> {
    if !matches!(req.method(), &Method::GET | &Method::HEAD) {
        return Ok(not_implemented());
    }

    match segments {
        [] => index(req),
        _ => Ok(not_found()),
    }
}
// END_SPAN route

// START_SPAN handle
fn handle(req: Request<Body>, remote_addr: IpAddr) -> Response<Body> {
    let time = Instant::now();

    let path = req.uri().path().to_owned();
    let segments: Vec<&str> =
        path.split('/').filter(|s| !s.is_empty()).collect();

    let resp = route(&req, &segments).unwrap_or_else(|err| {
        eprintln!("Error: {:#}", err);
        internal_server_error()
    });

    eprintln!(
        "{} {} {} {} {:?}",
        remote_addr,
        req.method(),
        req.uri(),
        resp.status(),
        time.elapsed(),
    );

    resp
}
// END_SPAN handle

// START_SPAN main
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    eprintln!("Listening on {}", addr);

    let make_service = make_service_fn(|conn: &AddrStream| {
        // Get the remote IP address.
        let remote_addr = conn.remote_addr().ip();
        // Move it to this async block.
        async move {
            // Move it to the closure, and move it again to the inner async
            // block.
            Ok::<_, Infallible>(service_fn(move |req| async move {
                Ok::<_, Infallible>(handle(req, remote_addr))
            }))
        }
    });

    if let Err(e) = Server::bind(&addr).serve(make_service).await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
// END_SPAN main
