# Gotham Header Route Matcher
## Crate: [gotham_header_routematcher](https://crates.io/crates/gotham_header_routematcher)

The `gotham_header_routematcher` crate provides a `RouteMatcher` implementation
for [Gotham](https://gotham.rs) that allows you to match on arbitrary header
values.

```rust
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::State;
use gotham::test::TestServer;
use hyper::StatusCode;
use hyper::header::HeaderValue;

use crate::HeaderEqualsRouteMatcher;

fn host_a_handler(state: State) -> (State, &'static str) {
    (state, "host_a_handler")
}

fn host_b_handler(state: State) -> (State, &'static str) {
    (state, "host_b_handler")
}

fn create_host_router() -> Router {
    build_simple_router(|route| {
        let host_a = HeaderEqualsRouteMatcher::new("host".to_string(), b"a".to_vec());
        let host_b = HeaderEqualsRouteMatcher::new("host".to_string(), b"b".to_vec());
        route.get("/").add_route_matcher(host_a).to(host_a_handler);
        route.get("/").add_route_matcher(host_b).to(host_b_handler);
    })
}

fn check_routes() {
    let test_server = TestServer::new(create_host_router()).unwrap();
    let test_client = test_server.client();

    let test_a = test_client.get("http://localhost/")
        .with_header("host", HeaderValue::from_static("a"))
        .perform()
        .unwrap();
    assert_eq!(test_a.status(), StatusCode::OK);

    let test_a_body = test_a.read_utf8_body().unwrap();
    assert_eq!(test_a_body, "host_a_handler");

    let test_b = test_client.get("http://localhost/")
        .with_header("host", HeaderValue::from_static("b"))
        .perform()
        .unwrap();
    assert_eq!(test_b.status(), StatusCode::OK);

    let test_b_body = test_b.read_utf8_body().unwrap();
    assert_eq!(test_b_body, "host_b_handler");

    let test_c = test_client.get("http://localhost/")
        .with_header("host", HeaderValue::from_static("c"))
        .perform()
        .unwrap();
    assert_eq!(test_c.status(), StatusCode::NOT_FOUND);
}
```