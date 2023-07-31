use std::convert::Infallible;
use std::net::SocketAddr;

use cookie::{Cookie, CookieJar};
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Request, Response, Server, StatusCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

// START_SPAN cookies
trait CookiesExt {
    fn cookies(&self) -> CookieJar;
}

impl CookiesExt for Request<Body> {
    fn cookies(&self) -> CookieJar {
        let mut jar = CookieJar::new();

        // Iterate on the Cookie header instances.
        for value in self.headers().get_all(header::COOKIE) {
            // Get the name-value pairs separated by semicolons.
            let it = match value.to_str() {
                Ok(s) => s.split(';').map(str::trim),
                Err(_) => continue,
            };

            // Iterate on the pairs.
            for s in it {
                // Parse and add the cookie to the jar.
                if let Ok(c) = Cookie::parse(s.to_owned()) {
                    jar.add_original(c);
                }
            }
        }

        jar
    }
}
// END_SPAN cookies

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

// START_SPAN forbidden
fn forbidden() -> Response<Body> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Body::empty())
        .unwrap()
}
// END_SPAN forbidden

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

// START_SPAN session_id
const ADMIN_SESSION_ID: &str = "aec070645fe53ee3b3763059376134f0";
// END_SPAN session_id

// START_SPAN admin
async fn admin(req: &mut Request<Body>) -> Result<Response<Body>> {
    allow_method!(req.method(), &Method::GET | &Method::HEAD);
    redirect_no_trailing_slash!(req);

    // Match session_id against ADMIN_SESSION_ID.
    if req.cookies().get("session_id").map(Cookie::value)
        != Some(ADMIN_SESSION_ID)
    {
        return Ok(forbidden());
    }

    Ok(ok_with_text("Authenticated"))
}
// END_SPAN admin

// START_SPAN login
async fn login(req: &mut Request<Body>) -> Result<Response<Body>> {
    allow_method!(req.method(), &Method::POST);
    redirect_no_trailing_slash!(req);

    let body = hyper::body::to_bytes(req.body_mut()).await?;
    let mut password = None;

    for (k, v) in form_urlencoded::parse(&body) {
        if k == "password" {
            password = Some(v.into_owned());
        }
    }

    let password = match password {
        Some(s) => s,
        None => return Ok(bad_request("Missing `password`")),
    };

    if password != "hunter2" {
        return Ok(forbidden());
    }

    // Get the cookie jar.
    let mut jar = req.cookies();

    // Build a session cookie.
    let cookie = Cookie::build("session_id", ADMIN_SESSION_ID)
        .path("/")
        .secure(false) // Do not require HTTPS.
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .finish();

    // Add it to the jar.
    jar.add(cookie);

    // Prepare the response to redirect to /admin.
    let mut resp = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/admin");

    // Set the changed cookies.
    for cookie in jar.delta() {
        resp = resp.header(header::SET_COOKIE, cookie.to_string());
    }

    // Return with an empty body.
    Ok(resp.body(Body::empty()).unwrap())
}
// END_SPAN login

// START_SPAN route
async fn route(
    req: &mut Request<Body>,
    segments: &[&str],
) -> Result<Response<Body>> {
    match segments {
        ["admin"] => admin(req).await,
        ["login"] => login(req).await,
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
