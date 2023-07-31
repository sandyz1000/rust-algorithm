// START_SPAN use
use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
// END_SPAN use

// START_SPAN ok
fn ok() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}
// END_SPAN ok

// START_SPAN handle
fn handle(_req: Request<Body>) -> Response<Body> {
    ok()
}
// END_SPAN handle

// START_SPAN main
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    eprintln!("Listening on {}", addr);

    // The closure passed to `make_service_fn` is executed each time a new
    // connection is established and returns a future that resolves to a
    // service.
    let make_service = make_service_fn(|_conn| async {
        // The closure passed to `service_fn` is executed each time a request
        // arrives on the connection and returns a future that resolves
        // to a response.
        Ok::<_, Infallible>(service_fn(|req| async {
            // Call the request handler.
            Ok::<_, Infallible>(handle(req))
        }))
    });

    // Start the server.
    if let Err(e) = Server::bind(&addr).serve(make_service).await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
// END_SPAN main
