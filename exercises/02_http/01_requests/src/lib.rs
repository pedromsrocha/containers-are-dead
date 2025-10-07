use serde::Deserialize;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

/// The endpoint should accept the following query parameters
#[derive(Deserialize)]
struct Params {
    /// The smallest random number allowed, defaults to 0
    min: Option<usize>,
    /// The largest random number allowed, defaults to 1000
    max: Option<usize>,
}

#[http_component]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Hint: use the `serde_querystring` crate that is already added as a dependency to parse the `req.query()` string!
    // see that crates docs for details!

    // TODO requests to this epoint: "http://www.randomnumberapi.com/api/v1.0/random?min={}&max={}"
    // and return a single number as a result.

    todo!()
}
