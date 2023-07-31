use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

fn ok_with_text<S: Into<String>>(text: S) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from(text.into()))
        .unwrap()
}

fn permanent_redirect<S: AsRef<str>>(url: S) -> Response<Body> {
    Response::builder()
        .status(StatusCode::PERMANENT_REDIRECT)
        .header(header::LOCATION, url.as_ref())
        .body(Body::empty())
        .unwrap()
}

fn bad_request<S: Into<String>>(text: S) -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
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

macro_rules! redirect_no_trailing_slash {
    ($req:expr) => {{
        let path = $req.uri().path();

        if path.ends_with('/') {
            let path = path.trim_end_matches('/');

            if !path.is_empty() {
                if let Some(q) = $req.uri().query() {
                    let mut path = path.to_owned();

                    path.push('?');
                    path.push_str(q);

                    return Ok(self::permanent_redirect(path));
                }

                return Ok(self::permanent_redirect(path));
            }
        }
    }};
}

// START_SPAN hello
async fn hello(req: &mut Request<Body>) -> Result<Response<Body>> {
    allow_method!(req.method(), &Method::GET | &Method::HEAD | &Method::POST);
    redirect_no_trailing_slash!(req);

    if req.method() == Method::POST {
        let body = hyper::body::to_bytes(req.body_mut()).await?;
        let mut name = None;

        for (k, v) in form_urlencoded::parse(&body) {
            if k == "name" {
                name = Some(v.into_owned());
            }
        }

        let name = match name {
            Some(s) => s,
            None => return Ok(bad_request("Missing `name`")),
        };

        Ok(ok_with_text(format!("Hello, {}!", name)))
    } else {
        Ok(ok_with_text("Hello, World!"))
    }
}
// END_SPAN hello

// START_SPAN route
async fn route(
    req: &mut Request<Body>,
    segments: &[&str],
) -> Result<Response<Body>> {
    match segments {
        ["hello"] => hello(req).await,
        _ => Ok(not_found()),
    }
}
// END_SPAN route

// START_SPAN handle
async fn handle(mut req: Request<Body>) -> Response<Body> {
    // Extract the segments from the URI path.
    let path = req.uri().path().to_owned();
    let segments: Vec<&str> =
        path.split('/').filter(|s| !s.is_empty()).collect();

    route(&mut req, &segments).await.unwrap_or_else(|err| {
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
            Ok::<_, Infallible>(handle(req).await)
        }))
    });

    if let Err(e) = Server::bind(&addr).serve(make_service).await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
