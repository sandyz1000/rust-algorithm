use std::convert::Infallible;
use std::fmt::Write as _;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use tokio::sync::Mutex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

fn ok_with_text<S: Into<String>>(text: S) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from(text.into()))
        .unwrap()
}

fn created<S: AsRef<str>>(url: S) -> Response<Body> {
    Response::builder()
        .status(StatusCode::CREATED)
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

// START_SPAN list
async fn list(
    ctx: &Context,
    req: &mut Request<Body>,
) -> Result<Response<Body>> {
    allow_method!(req.method(), &Method::GET | &Method::HEAD | &Method::POST);

    if req.method() == Method::POST {
        let body = hyper::body::to_bytes(req.body_mut()).await?;
        let mut text = None;

        for (k, v) in form_urlencoded::parse(&body) {
            if k == "text" {
                text = Some(v.into_owned());
            }
        }

        let text = match text {
            Some(s) => s,
            None => return Ok(bad_request("Missing `text`")),
        };

        let mut v = ctx.data.lock().await;

        v.push(text);

        return Ok(created(format!("/{}", v.len() - 1)));
    }

    let mut output = String::new();

    for (id, text) in ctx.data.lock().await.iter().enumerate() {
        writeln!(&mut output, "{} | {}", id, text).unwrap();
    }

    Ok(ok_with_text(output))
}
// END_SPAN list

// START_SPAN details
async fn details(
    ctx: &Context,
    req: &Request<Body>,
    id: &str,
) -> Result<Response<Body>> {
    allow_method!(req.method(), &Method::GET | &Method::HEAD);

    let id = match id.parse::<usize>() {
        Ok(n) => n,
        Err(e) => return Ok(bad_request(e.to_string())),
    };

    match ctx.data.lock().await.get(id) {
        Some(s) => Ok(ok_with_text(s)),
        None => Ok(not_found()),
    }
}
// END_SPAN details

// START_SPAN route
async fn route(
    ctx: &Context,
    req: &mut Request<Body>,
    segments: &[&str],
) -> Result<Response<Body>> {
    if ctx.debug {
        eprintln!("{:?}", req);
    }

    match segments {
        [] => list(ctx, req).await,
        [id] => details(ctx, req, id).await,
        _ => Ok(not_found()),
    }
}
// END_SPAN route

// START_SPAN handle
async fn handle(ctx: Arc<Context>, mut req: Request<Body>) -> Response<Body> {
    let path = req.uri().path().to_owned();
    let segments: Vec<&str> =
        path.split('/').filter(|s| !s.is_empty()).collect();

    route(&ctx, &mut req, &segments)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Error: {:#}", err);
            internal_server_error()
        })
}
// END_SPAN handle

// START_SPAN context
struct Context {
    debug: bool,
    data: Mutex<Vec<String>>,
}
// END_SPAN context

// START_SPAN main
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    eprintln!("Listening on {}", addr);

    // Initialize the context.
    let ctx = Arc::new(Context {
        debug: true,
        data: Mutex::new(Vec::new()),
    });

    let make_service = make_service_fn(|_conn| {
        // Clone the pointer for each connection.
        let ctx = Arc::clone(&ctx);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                // Clone the pointer for each request.
                let ctx = Arc::clone(&ctx);
                async move { Ok::<_, Infallible>(handle(ctx, req).await) }
            }))
        }
    });

    if let Err(e) = Server::bind(&addr).serve(make_service).await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
// END_SPAN main
