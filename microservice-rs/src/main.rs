extern crate hyper;
extern crate maud;
extern crate futures;
extern crate url;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::collections::HashMap;
use std::error::Error;
use std::env;
use std::io;

use hyper::{Request, Response, StatusCode, Method};
use postgres::Error;
// use hyper::header::{Header, ContentLength, ContentType};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use hyper::service::{service_fn, Service};
// use hyper::server::conn::http1;

// use futures::Stream;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use maud::html;
mod models;
mod schema;
mod utils;
mod webapi;
use utils::FutureResult;
use models::{Message, NewMessage};

const DEFAULT_DATABASE_URL: &str = "postgresql://postgres@localhost:5432";
type FutRes = Box<dyn FutureResult<hyper::Response<()>, hyper::Error, Ok=Ok(), Err=Err>>;


struct Microservice;

struct TimeRange {
    before: Option<i64>,
    after: Option<i64>,
}

fn parse_form(form_chunk: Chunk) -> Box<dyn FutureResult<NewMessage, hyper::Error, Ok = Ok, Err=Err>> {
    let mut form = url::form_urlencoded::parse(form_chunk.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();

    if let Some(message) = form.remove("message") {
        let username = form.remove("username").unwrap_or(String::from("anonymous"));
        futures::future::ok(NewMessage { username, message })
    } else {
        futures::future::err(hyper::Error::from(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing field 'message",
        )))
    }
}

fn write_to_db(
    new_message: NewMessage,
    db_connection: &mut PgConnection,
) -> Box<dyn FutureResult<i64, hyper::Error, Ok=..., Err=...>> {
    use schema::messages;
    let timestamp = diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(messages::timestamp)
        .get_result(db_connection);

    match timestamp {
        Ok(timestamp) => futures::future::ok(timestamp),
        Err(error) => {
            error!("Error writing to database: {}", error.description());
            futures::future::err(hyper::Error::from(
                io::Error::new(io::ErrorKind::Other, "service error"),
            ))
        }
    }
}

fn make_error_response(error_message: &str) -> FutRes {
    let payload = json!({
        "error": error_message
    }).to_string();
    let response = Response::new(payload);
    // response.status()
    response.headers().append("Content-Length", payload.len() as u64);
        response.headers().append("Content-Type", "application-json");
    // .with_status(StatusCode::InternalServerError)
    // .with_header(ContentLength(payload.len() as u64))
    // .with_header(ContentType::json())
    // .with_body(payload);
    debug!("{:?}", response);
    futures::future::ok(response)
}

fn make_post_response(
    result: Result<i64, hyper::Error>
) -> FutRes {
    match result {
        Ok(timestamp) => {
            let payload = json!({
                "timestamp": timestamp
            }).to_string();
            let response = Response::new(payload);
            response.headers().append("Content-Length", payload.len() as u64);
            response.headers().append("Content-Type", "application-json");
            // .with_header(ContentLength(payload.len() as u64))
            // .with_header(ContentType::json())
            // .with_body(payload);
            debug!("{:?}", response);
            futures::future::ok(response)
        }
        Err(error) => make_error_response(error.description()),
    }
}

fn parse_query(query: &str) -> Result<TimeRange, String> {
    let args = url::form_urlencoded::parse(&query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let before = args.get("before").map(|value| value.parse::<i64>());
    if let Some(Err(ref error)) = before {
        return Err(format!("Error parsing 'before': {}", error));
    }

    let after = args.get("after").map(|value| value.parse::<i64>());
    if let Some(Err(ref error)) = after {
        return Err(format!("Error parsing 'after': {}", error));
    }

    Ok(TimeRange {
        before: before.map(|b| b.unwrap()),
        after: after.map(|a| a.unwrap()),
    })
}

fn query_db(time_range: TimeRange, db_connection: &PgConnection) -> Option<Vec<Message>> {
    use schema::messages;
    let TimeRange { before, after } = time_range;

    let mut query = messages::table.into_boxed();

    if let Some(before) = before {
        query = query.filter(messages::timestamp.lt(before as i64))
    }

    if let Some(after) = after {
        query = query.filter(messages::timestamp.gt(after as i64))
    }

    let query_result = query.load::<Message>(db_connection);

    match query_result {
        Ok(result) => Some(result),
        Err(error) => {
            error!("Error querying DB: {}", error);
            None
        }
    }
}

fn render_page(messages: Vec<Message>) -> String {
    (html! {
        head {
            title {"microservice"}
            style {"body { font-family: monospace }"}
        }
        body {
            ul {
                @for message in &messages {
                    li {
                        (message.username) " (" (message.timestamp) "): " (message.message)
                    }
                }
            }
        }
    }).into_string()
}

fn make_get_response(messages: Option<Vec<Message>>) -> FutRes {
    // 
    let response = match messages {
        Some(messages) => {
            let body = render_page(messages);
            let response = Response::new(body);
            response.headers().append("Content-Length", body.len() as u64);
            response.headers().append("Content-Type", "HTML");
            response
        }
        None => Response::new("").with_status(StatusCode::InternalServerError),
    };
    debug!("{:?}", response);
    futures::future::ok(response)
}

fn connect_to_db() -> Option<PgConnection> {
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DATABASE_URL));
    match PgConnection::establish(&database_url) {
        Ok(connection) => Some(connection),
        Err(error) => {
            error!("Error connecting to database: {}", error.description());
            None
        }
    }
}

impl Service<Request<()>> for Microservice {
    // type Response = Response;
    // type Error = hyper::Error;
    // type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request<()>) -> Self::Future {
        debug!("{:?}", request);
        let db_connection = match connect_to_db() {
            Some(connection) => connection,
            None => {
                return Box::new(futures::future::ok(
                    Response::new("").with_status(StatusCode::InternalServerError),
                ))
            }
        };
        match (request.method(), request.path()) {
            (&Method::Post, "/") => {
                let mut db_connection_mut = db_connection;
                let future = request
                    .body()
                    .concat2()
                    .and_then(parse_form)
                    .and_then(move |new_message| write_to_db(new_message, &mut db_connection_mut))
                    .then(make_post_response);
                Box::new(future)
            }
            (&Method::Get, "/") => {
                let time_range = match request.query() {
                    Some(query) => parse_query(query),
                    None => Ok(TimeRange {
                        before: None,
                        after: None,
                    }),
                };
                let response = match time_range {
                    Ok(time_range) => make_get_response(query_db(time_range, &db_connection)),
                    Err(error) => make_error_response(&error),
                };
                Box::new(response)
            }
            _ => Box::new(futures::future::ok(
                Response::new("").with_status(StatusCode::NotFound),
            )),
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    
    // let server = hyper::server::Http::new()
    //     .bind(&address, move || Ok(Microservice))
    //     .unwrap();
    // info!("Running microservice at {}", address);
    // server.run().unwrap();
    
    env_logger::init();
    // This address is localhost
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    
    // Bind to the port and listen for incoming TCP connections
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
        // .await point allows the Tokio runtime to pull the task off of the thread until the task
        // has work to do. In this case, a connection arrives on the port we are listening on and
        // the task is woken up, at which point the task is then put back on a thread, and is
        // driven forward by the runtime, eventually yielding a TCP stream.
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            let service = service_fn(move |req| response_examples(req));
            info!("Running microservice at {}", addr);
            
            // Handle the connection from the client using HTTP1 and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
