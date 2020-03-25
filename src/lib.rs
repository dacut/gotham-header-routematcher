use gotham::router::non_match::RouteNonMatch;
use gotham::router::route::matcher::RouteMatcher;
use gotham::state::{FromState, State};
use hyper::{HeaderMap, StatusCode};

#[cfg(test)]
mod unittest;

/// Check whether a header is present and contains a specified value.
#[derive(Clone)]
pub struct HeaderEqualsRouteMatcher {
    header_name: String,
    header_value: Vec<u8>,
}

impl HeaderEqualsRouteMatcher {
    /// Create a new `HeaderEqualsRouteMatcher`
    pub fn new(header_name: String, header_value: Vec<u8>) -> Self {
        HeaderEqualsRouteMatcher {
            header_name,
            header_value,
        }
    }
}

impl RouteMatcher for HeaderEqualsRouteMatcher {
    /// Determines if the `Request` was made with a header that equals the
    /// value exactly.
    fn is_match(&self, state: &State) -> Result<(), RouteNonMatch> {
        match HeaderMap::borrow_from(state).get(&self.header_name) {
            None => Err(RouteNonMatch::new(StatusCode::NOT_FOUND)),
            Some(hv) => if self.header_value == hv.as_bytes() {
                Ok(())
            } else {
                Err(RouteNonMatch::new(StatusCode::NOT_FOUND))
            },
        }
    }
}
