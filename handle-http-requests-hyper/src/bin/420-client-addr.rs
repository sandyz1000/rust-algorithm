use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::time::Instant;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Request, Response, Server, StatusCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

// START_SPAN client_addr
struct ClientAddr(IpAddr);

trait ClientAddrExt {
    fn client_addr(&self) -> Option<IpAddr>;
    fn set_client_addr(&mut self, remote_addr: IpAddr);
}

impl ClientAddrExt for Request<Body> {
    fn client_addr(&self) -> Option<IpAddr> {
        self.extensions().get::<ClientAddr>().map(|a| a.0)
    }

    fn set_client_addr(&mut self, remote_addr: IpAddr) {
        let addr = self
            .headers()
            .get("X-Forwarded-For")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.split(',').next())
            .and_then(|s| s.trim().parse::<IpAddr>().ok())
            .unwrap_or(remote_addr);

        self.extensions_mut().insert(ClientAddr(addr));
    }
}
// END_SPAN client_addr

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

fn index(req: &Request<Body>) -> Result<Response<Body>> {
    allow_method!(req.method(), &Method::GET | &Method::HEAD);

    Ok(ok())
}

fn route(req: &Request<Body>, segments: &[&str]) -> Result<Response<Body>> {
    match segments {
        [] => index(req),
        _ => Ok(not_found()),
    }
}

// START_SPAN handle
fn handle(mut req: Request<Body>, remote_addr: IpAddr) -> Response<Body> {
    let time = Instant::now();

    let path = req.uri().path().to_owned();
    let segments: Vec<&str> =
        path.split('/').filter(|s| !s.is_empty()).collect();

    req.set_client_addr(remote_addr);

    let resp = route(&req, &segments).unwrap_or_else(|err| {
        eprintln!("Error: {:#}", err);
        internal_server_error()
    });

    eprintln!(
        "{} {} {} {} {:?}",
        req.client_addr().unwrap(),
        req.method(),
        req.uri(),
        resp.status(),
        time.elapsed(),
    );

    resp
}
// END_SPAN handle

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    eprintln!("Listening on {}", addr);

    let make_service = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr().ip();
        async move {
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
