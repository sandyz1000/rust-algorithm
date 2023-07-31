# How to handle HTTP requests with Rust and Hyper

+++
uuid: "ee22d20c-9133-47dd-b489-438e162ac35e"
date: "2021-09-06T00:00:00Z"
description: |
  Recipes to build an asynchronous HTTP server with Hyper, including request
  routing, form handling, and cookie-based authentication.
published: true
+++

A router matches requests against registered routes and invokes the associated
request handler to return a response. There are many routers available, but
they are often tied to a whole framework. Instead, this article explains how to
handle HTTP requests with [Hyper](https://hyper.rs/), how to route requests with
Rust pattern matching, and how to handle query parameters, forms, cookies, and
more. It assumes you are familiar with the basics of Linux, HTTP, and Rust. For
full code samples, see the [source
files](https://git.dzx.fr/gdzx/rustweb/src/branch/master/content/posts/how-to-handle-http-requests-with-rust-and-hyper/).

{{ toc() }}

## Introduction

{% set path = "Cargo.toml" %}

The first step is to create a minimal HTTP server that you can build on for the
rest of this article:

{% filter code(lang="console") %}
$ cargo new --bin web
     Created binary (application) `web` package
{% endfilter %}

Add the crates [`cookie`], [`form_urlencoded`], [`hyper`] and [`tokio`] as
dependencies (enable the `full` feature set so you can experiment freely):

[`cookie`]: https://docs.rs/cookie/
[`form_urlencoded`]: https://docs.rs/form_urlencoded/
[`hyper`]: https://docs.rs/hyper/
[`tokio`]: https://docs.rs/tokio/

{% filter code(lang="toml", title=path) %}
	{{- include(path=path) }}
{% endfilter %}

{% set path = "src/bin/000-introduction.rs" %}

Import the required items:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="use") }}
{% endfilter %}

The following function builds a successful [`Response`] with the status `200
OK` and an empty [`Body`]:

[`Response`]: https://docs.rs/hyper/0.14.12/hyper/struct.Response.html
[`Body`]: https://docs.rs/hyper/0.14.12/hyper/struct.Body.html

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="ok") }}
{% endfilter %}

A handler is a function that takes a [`Request`] and returns a `Response`.
Reuse the previous function to create `handle`, the primary request handler:

[`Request`]: https://docs.rs/hyper/0.14.12/hyper/struct.Request.html

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

The `main` function instantiates an HTTP server listening on `127.0.0.1:3000`
which invokes `handle` for each incoming request:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="main") }}
{% endfilter %}

You can start this server using Cargo:

{% filter code(lang="console") %}
$ cargo run --bin 000-introduction
Listening on 127.0.0.1:3000
{% endfilter %}

Then, query it with cURL (the `-i` option includes the response headers in the
output):

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/
HTTP/1.1 200 OK
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

## HTTP

The target of an HTTP request corresponds to a resource (e.g., a list of tasks
for `/tasks`, the first one for `/tasks/1`). A method defines an action on a
particular resource (e.g., `POST /tasks` to create a new task, `GET /tasks/1`
to view the first one). When a resource does not exist, or the method for that
resource is not allowed, the server returns an error to the client.

### Resources

{% set path = "src/bin/110-target.rs" %}

If there is no request handler associated with a resource, the server returns
`404 Not Found`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="not_found") }}
{% endfilter %}

Create an handler `index` that always responds with `200 OK`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="index") }}
{% endfilter %}

Update `handle` to route requests for `/` to `index`, or return `404 Not Found`
for any other target:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

A request for `/` returns `200 OK` as expected:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/
HTTP/1.1 200 OK
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

For any other target, the server returns `404 Not Found`:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/foo
HTTP/1.1 404 Not Found
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

### Methods

The method defines the semantics of a request for a particular resource. A
handler associated with a resource receives all the requests for this target,
regardless of the method. Therefore, it is the responsibility of the handler to
allow or deny specific methods, but as an additional guard, the server can deny
methods it does not implement before calling any handler.

#### Method not allowed

{% set path = "src/bin/121-method-not-allowed.rs" %}

A handler receiving a request with a forbidden method returns `405 Method Not
Allowed` with the `Allow` header to indicate the list of valid methods for that
resource:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="method_not_allowed") }}
{% endfilter %}

Update `index` to restrict methods other than `GET` and `HEAD`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="index") }}
{% endfilter %}

A `POST` request for `/` returns `405 Method Not Allowed`:

{% filter code(lang="console") %}
$ curl -i -X POST http://localhost:3000/
HTTP/1.1 405 Method Not Allowed
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

#### Not implemented

{% set path = "src/bin/122-method-not-implemented.rs" %}

When a method is not allowed in any of the request handlers, the server can
return `501 Not Implemented`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="not_implemented") }}
{% endfilter %}

Since `index` only accepts `GET` or `HEAD`, you can update `handle` to return
this status for any other method:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

With the method `POST`, the server returns `501 Not Implemented`:

{% filter code(lang="console") %}
$ curl -i -X POST http://localhost:3000/
HTTP/1.1 501 Not Implemented
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

### Errors

{% set path = "src/bin/130-errors.rs" %}

When the target is unknown, the server returns `404 Not Found`, which is an
example of a client error. On the contrary, being unable to connect to the
database is an example of server error, and it returns `500 Internal Server
Error` to the client:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="internal_server_error") }}
{% endfilter %}

In a resource handler, you may have to parse request parameters, query a
database, render templates, all of which can produce errors. Therefore, the
request handlers return a [`Result<T, E>`], where `T` is a `Response`. Client
errors are regular responses with the `Ok` variant, but server errors use the
`Err` variant.

[`Result<T, E>`]: https://doc.rust-lang.org/std/result/enum.Result.html

Define a generic `Result<T>` type for all the handlers (you can also use a type
defined in crates such as [`anyhow`]):

[`anyhow`]: https://docs.rs/anyhow/

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="types") }}
{% endfilter %}

Then, update `index` to return this type:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="index") }}
{% endfilter %}

Create the handler `route`, responsible for routing the requests as `handle`
did previously, but instead returning a `Result`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route") }}
{% endfilter %}

`handle` has to return a `Response` for each request, so any error from `route`
needs to be turned into a `500 Internal Server Error` response:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

A request for `/` returns `200 OK`:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/
HTTP/1.1 200 OK
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

A request for `/error` returns `500 Internal Server Error`.

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/error
HTTP/1.1 500 Internal Server Error
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

The error message is written to the console:

{% filter code(lang="console") %}
$ cargo run --bin 130-errors
Listening on 127.0.0.1:3000
Error: This is an error
{% endfilter %}

### Macros

{% set path = "src/bin/140-macros.rs" %}

You may have to repeat some operations in multiple handlers. `405 Method Not
Allowed` responses must have an `Allow` header, so you also have to return the
list of methods in the pattern (unless you can tolerate a slight deviation from
RFC7231).

To reduce code duplication and automatically generate the list of allowed
methods, you can create a [declarative
macro](https://doc.rust-lang.org/reference/macros-by-example.html):

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="allow_method") }}
{% endfilter %}

Update `index` to make use of it:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="index") }}
{% endfilter %}

With the method `POST` for `/`, the server returns `405 Method Not Allowed`
including the list of allowed methods:

{% filter code(lang="console") %}
$ curl -i -X POST http://localhost:3000/
HTTP/1.1 405 Method Not Allowed
allow: GET, HEAD
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

[Procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html)
may provide a more efficient implementation, which I leave as an exercise for
the reader...

## Routing

A route associates an HTTP target with a resource. Up until now, `route` relies
on a naive string comparison with the target. By leveraging Rust pattern
matching, it is possible to match and extract parameters from dynamic routes.

### Path segmentation

{% set path = "src/bin/210-segments.rs" %}

A URL is formed of `/` separated components. A simple way to match a URL is to
split it into segments. For instance, `/hello/world` is "equivalent" to the
segments `["hello", "world"]`.

First, add a new function to build a `200 OK` response with an UTF-8 encoded
plain text body:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="ok_with_text") }}
{% endfilter %}

Add the handler `hello`, that returns a custom "Hello, World!" based on its
argument `name`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="hello") }}
{% endfilter %}

Update `route` to accept segments as a parameter and match against them:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route") }}
{% endfilter %}

Update `handle` to split the path into segments (dropping empty components
between duplicate `/`):

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

It is impossible to distinguish `/hello` and `/hello/` from the segments alone,
but you will see how to handle this situation in [§ Handlers § Trailing
slashes](#trailing-slashes).

### Normalization

{% set path = "src/bin/220-normalization.rs" %}

Akin to filesystem paths, URL paths can contain `.` and `..` components, such
that `/world/../hello/.` is equivalent to `/hello`. Transforming a path into
its shortest equivalent, by eliminating these components, is called
normalization. Behind a reverse proxy and with well-behaved clients, you may
already receive normalized URLs, otherwise, you can add normalization to
`handle`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

Test with `/world/../hello/.`:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/world/../hello/.
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 13
date: Thu, 1 Jan 1970 00:00:00 GMT

Hello, World!%
{% endfilter %}

### Segment matching

As opposed to static URLs, dynamic URLs contain parameters. For example,
`/tasks/3` and `/tasks/42` correspond to the same pattern `/tasks/<id>`, where
the id segment is dynamic. These parameters must be extracted from a routing
handler, and passed to a resource handler as arguments.

#### Match a single segment

{% set path = "src/bin/231-extract-segment.rs" %}

You can use Rust pattern matching to bind the second segment to the variable
`name` and pass it as an argument to `hello`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route") }}
{% endfilter %}

Try `/hello/Jane`:

{% filter code(lang="console") %}
curl -i http://localhost:3000/hello/Jane
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 12
date: Thu, 1 Jan 1970 00:00:00 GMT

Hello, Jane!%
{% endfilter %}

#### Match multiple segments

{% set path = "src/bin/232-extract-segments.rs" %}

You can also bind multiple segments (forming a sub-slice) to a variable with
the `..` placeholder (matching zero or more elements). To extract a path, you
can join these segments with `/`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route") }}
{% endfilter %}

Try `/hello/r/rust`:

{% filter code(lang="console") %}
curl -i http://localhost:3000/hello/r/rust
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 14
date: Thu, 1 Jan 1970 00:00:00 GMT

Hello, r/rust!%
{% endfilter %}

#### Additional patterns

There are a number of
[patterns](https://doc.rust-lang.org/reference/patterns.html) that you can play
with:

{% filter code(lang="rust") %}
match segments {
	// Or pattern.
	[] | ["index.html" | "index.htm"] => ...,

	// One trailing segment.
	["hello1", _] => ...,

	// One or more trailing segment.
	["hello+", s @ ..] if !s.empty() => ...,

	// Zero or more trailing segments.
	["hello*", ..] => ...,

	// Last segment.
	["hello$", .., last] => ...,

	// Custom guard.
	s if is_match(s) => ...,

	_ => Ok(not_found()),
}
{% endfilter %}

### Nested routing

{% set path = "src/bin/240-nested-route.rs" %}

If you can extract multiple segments from a URL, you can pass them to a
sub-routing handler:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route_hello") }}
{% endfilter %}

From `route`, call `route_hello` with the remaining segments:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route") }}
{% endfilter %}

## Handlers

The routing handlers extract segments from the target and pass them as a string
to the resource handlers. Then, it is the responsibility of the resource
handler to parse these parameters into their expected type.

### Trailing slashes

{% set path = "src/bin/310-redirect.rs" %}

Since both `/hello` and `/hello/` correspond to the same segment `hello`, a
request to any of these targets is routed to the `hello` handler:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/hello
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 13
date: Thu, 1 Jan 1970 00:00:00 GMT

Hello, World!%
{% endfilter %}

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/hello/
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 13
date: Thu, 1 Jan 1970 00:00:00 GMT

Hello, World!%
{% endfilter %}

Although these two URLs point to the same resource, they are not strictly
identical. The page is duplicated, which is undesirable from an SEO
perspective. Additionally, it would be extremely surprising if `/hello` and
`/hello/` pointed to a different resource. Therefore, each handler should
enforce a canonical URL with or without a trailing `/`.

From an historical point of view, a trailing `/` indicates a directory, whereas
no trailing `/` indicates a file. For directories, the server returns the
content of `./index.html`. For example, a static website might have the
following layout:

{% filter code %}
website
├── index.html
├── posts
│   ├── index.html
│   └── my-first-post
│       ├── image.jpg
│       └── index.html
└── static
    └── main.css
{% endfilter %}

Each folder encapsulates a piece of content. From
`/website/posts/my-first-post/`, you can link to the image easily with
`./image.jpg`.

Dynamic web servers can return whatever they want for any URL, but I try to
follow these rules:

- For static assets, no trailing slashes.
- For HTML pages, trailing slashes.
- For API endpoints, no trailing slashes.

Feel free to follow any rules you want, as long as you stay consistent. To
redirect the client, use a `308 Permanent Redirect` and a `Location` header for
the target URL:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="permanent_redirect") }}
{% endfilter %}

If you want to enforce no trailing slashes, you can redirect the client when
the path ends with `/`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="hello") }}
{% endfilter %}

Now, `/hello/` redirects permanently to `/hello`:

{% filter code(lang="console") %}
$ curl -i --head http://localhost:3000/hello/
HTTP/1.1 308 Permanent Redirect
location: /hello
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

You can handle the redirection in a routing handler (or even the reverse proxy)
if all the resource handlers use the same convention, but if at some point you
want to serve something that resembles a file, like an Atom feed, then it
becomes an issue.

Finally, to reduce code duplication, you can use the following macros:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="redirect_macros") }}
{% endfilter %}

### Path parameters

{% set path = "src/bin/320-parse-parameter.rs" %}

The URL segments are string slices, so you have to parse them into their
expected type. For an invalid request, the server returns `400 Bad Request`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="bad_request") }}
{% endfilter %}

In the following example, the parameter is parsed as an `usize` that indicates
the language id for the response. If the parameter is not a number, the handler
returns `400 Bad Request`, if it is out-of-range, it returns `404 Not Found`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="hello") }}
{% endfilter %}

Update `route` to pass the id to `hello`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route") }}
{% endfilter %}

With 3, it works as expected:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/hello/3
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 4
date: Thu, 1 Jan 1970 00:00:00 GMT

Hola%
{% endfilter %}

With 42, it returns `404 Not Found`, since the index is out-of-range:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/hello/42
HTTP/1.1 404 Not Found
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

Finally, if the parameter contains characters other than digits, the server
returns `400 Bad Request` with an error message:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/hello/world
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 29
date: Thu, 1 Jan 1970 00:00:00 GMT

invalid digit found in string%
{% endfilter %}

### Query parameters

{% set path = "src/bin/330-query-parameter.rs" %}

Besides the method and the target, a request can contain URL encoded query
parameters after the delimiter `?`, such as the parameter `lang` with the value
`en` in `/hello?lang=en`.

These parameters are accessible with `req.uri().query()`, and you can parse
them as a key/value list with the crate `form_urlencoded`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="hello") }}
{% endfilter %}

Try `/hello?lang=hi`:

{% filter code(lang="console") %}
$ curl -i 'http://localhost:3000/hello?lang=hi'
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 7
date: Thu, 1 Jan 1970 00:00:00 GMT

Namaste%
{% endfilter %}

### Form parameters

{% set path = "src/bin/340-form-parameters.rs" %}

HTML forms commonly use the same URL encoding to send their data, except when
the `method` attribute on the `form` element is set to `POST`. In this case,
the parameters are submitted through the body of the request.

Reading from the body is an asynchronous operation, so you need to make
`hello`, `route`, and `handle` asynchronous by:

- Replacing `fn` with `async fn`.
- Calling them with `.await`.

Update `hello` to allow `POST` and extract the `name` parameter from the
request body (you need a mutable reference to the request for this operation
consumes the body):

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="hello") }}
{% endfilter %}

Update `route` to pass a `&mut Request`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route") }}
{% endfilter %}

In `handle`, mark the request as mutable and pass a mutable reference to
`route`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

With the method `GET`:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/hello
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 13
date: Thu, 1 Jan 1970 00:00:00 GMT

Hello, World!%
{% endfilter %}

With the method `POST`, pass a name with the `-d` option:

{% filter code(lang="console") %}
$ curl -i -X POST -d name=Jane http://localhost:3000/hello
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 12
date: Thu, 1 Jan 1970 00:00:00 GMT

Hello, Jane!%
{% endfilter %}

Without a name, you get `400 Bad Request`:

{% filter code(lang="console") %}
$ curl -i -X POST -d foo=bar http://localhost:3000/hello
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 14
date: Thu, 1 Jan 1970 00:00:00 GMT

Missing `name`%
{% endfilter %}

## Middlewares

With a router, middlewares are needed to perform operations before or after the
handler associated with a route is invoked. With an explicit routing approach,
you can perform any operation directly.

### Logging

{% set path = "src/bin/410-logging.rs" %}

Update `handle` to log some information about the connection, the request, and
the response:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

You need to edit `main` to pass the remote IP address to `handle` (add `move`
on the inner closure and `async` blocks):

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="main") }}
{% endfilter %}

After these changes, make some requests and inspect the logs:

{% filter code(lang="console") %}
$ cargo run --bin 410-logging
Listening on 127.0.0.1:3000
127.0.0.1:36180 GET / 200 OK 127.046µs
127.0.0.1:36182 GET /foo 404 Not Found 22.246µs
127.0.0.1:36184 POST /foo 501 Not Implemented 8.398µs
{% endfilter %}

### Headers

{% set path = "src/bin/420-client-addr.rs" %}

Behind a reverse proxy, the remote address corresponds to the IP address of the
proxy itself. The real client IP address can be transmitted in the header
`X-Forwarded-For`. You can extend the request to extract and save this address
in its extension map:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="client_addr") }}
{% endfilter %}

Update `handle` to use this extension:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

Make a regular request:

{% filter code(lang="console") %}
$ curl http://localhost:3000/ > /dev/null
{% endfilter %}

Then, make another request with the header `X-Forwarded-For`, as if the server
was behind a reverse proxy:

{% filter code(lang="console") %}
$ curl -H 'X-Forwarded-For: 192.0.2.1' http://localhost:3000/ > /dev/null
{% endfilter %}

You can see both addresses in the log:

{% filter code(lang="console") %}
$ cargo run --bin 420-client-addr
Listening on 127.0.0.1:3000
127.0.0.1 GET / 200 OK 174.476µs
192.0.2.1 GET / 200 OK 246.638µs
{% endfilter %}

### Cookies

{% set path = "src/bin/430-cookies.rs" %}

A server can set cookies for a client with the header `Set-Cookie`. On each
subsequent request, the client sends back the cookies in the header `Cookie`.
The crate `cookie` manages cookies in a [`CookieJar`]: you can lookup their
values, add new ones, modify them, and most importantly, get the delta after
you made changes.

[`CookieJar`]: https://docs.rs/cookie/0.15.1/cookie/struct.CookieJar.html

Extend `Request` to extract the original cookies from the headers and return a
`CookieJar`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="cookies") }}
{% endfilter %}

This example uses a cookie to perform authentication. It relies on a session
cookie named `session_id` with a special value for administrator sessions:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="session_id") }}
{% endfilter %}

In practice, you would generate a new id for each session, and store it in a
database with the associated user, privileges, expiration time, etc. If a
client tries to access a restricted page without the appropriate permission,
the server responds with `403 Forbidden`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="forbidden") }}
{% endfilter %}

The `admin` handler authenticates the client by comparing the session cookie
value with the admin session id:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="admin") }}
{% endfilter %}

To login, the client must send a valid password to receive the admin session
cookie:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="login") }}
{% endfilter %}

Update `route` to forward requests to `admin` and `login`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route") }}
{% endfilter %}

Without a valid session id, the admin page returns `403 Forbidden`:

{% filter code(lang="console") %}
curl -i http://localhost:3000/admin
HTTP/1.1 403 Forbidden
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

To get the admin cookie, `POST` the password to the login endpoint (a web
browser would then save this cookie and follow the redirection):

{% filter code(lang="console") %}
curl -i -X POST -d password=hunter2 http://localhost:3000/login
HTTP/1.1 303 See Other
location: /admin
set-cookie: session_id=aec070645fe53ee3b3763059376134f0; HttpOnly; SameSite=Lax; Path=/
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

Finally, request the admin page with this cookie:

{% filter code(lang="console") %}
curl -i --cookie 'session_id=aec070645fe53ee3b3763059376134f0' http://localhost:3000/admin
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 13
date: Thu, 1 Jan 1970 00:00:00 GMT

Authenticated%
{% endfilter %}

## Context

{% set path = "src/bin/510-context.rs" %}

In a real web application, the handlers would need to access external or shared
resources: the application settings, a database, a templating engine, a job
queue. These resources can be shared with the handlers through a context passed
along with the request.

To share the same data between multiple threads, you need to use an [`Arc<T>`]
pointer, where `T` is your `Context`. It contains a boolean setting to enable
debug mode and a `Vec<String>` that emulates a table in a database with a
single text column. To allow mutable access, place the `Vec` inside a
`tokio::sync::Mutex`:

[`Arc<T>`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`tokio::sync::Mutex`]: https://docs.rs/tokio/1.11.0/tokio/sync/struct.Mutex.html

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="context") }}
{% endfilter %}

For the method `GET`, the `list` handler returns the list of rows; for the
method `POST`, it adds a new row based on the form parameter `text`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="list") }}
{% endfilter %}

The `details` handler returns the text from the given row:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="details") }}
{% endfilter %}

Update `route` to call these two resource handlers and pass a mutable request:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="route") }}
{% endfilter %}

Update `handle` to pass a mutable request to `route`:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="handle") }}
{% endfilter %}

In `main`, the context is initialized, put inside an `Arc`, then it is cloned
and shared with the handlers:

{% filter code(lang="rust", title=path) %}
	{{- include(path=path) | span(id="main") }}
{% endfilter %}

Initially, there is no data:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

With the method `POST`, you can add new rows:

{% filter code(lang="console") %}
$ curl -i -X POST -d text=hello http://localhost:3000/
HTTP/1.1 201 Created
location: /0
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

{% filter code(lang="console") %}
$ curl -i -X POST -d text=hola http://localhost:3000/
HTTP/1.1 201 Created
location: /1
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

{% filter code(lang="console") %}
$ curl -i -X POST -d text=aloha http://localhost:3000/
HTTP/1.1 201 Created
location: /2
content-length: 0
date: Thu, 1 Jan 1970 00:00:00 GMT
{% endfilter %}

The new rows appear in the list:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 11
date: Thu, 1 Jan 1970 00:00:00 GMT

0 | hello
1 | hola
2 | aloha
{% endfilter %}

You can view a single one given its id:

{% filter code(lang="console") %}
$ curl -i http://localhost:3000/0
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 5
date: Thu, 1 Jan 1970 00:00:00 GMT

hello%
{% endfilter %}

## Conclusion

If you decide to build a web server from scratch, I think Rust and Hyper are
solid options against micro-frameworks such as Flask. Hyper provides the
building blocks for asynchronous, safe, "low-level" HTTP handling, and Rust
pattern matching is powerful enough to replace a router. Starting from here,
you can add anything to the resource handlers. For example, this website relies
on the same techniques, with the addition of content and asset management, an
SQLite database, HTML templating, etc.
