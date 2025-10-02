use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
pub async fn handle_foo(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let req = Request::builder()
        .method(Method::Get)
        .uri("http://www.randomnumberapi.com/api/v1.0/random?min=100&max=1000")
        .build();

    let res: Response = spin_sdk::http::send(req).await?;

    let mut result = str::from_utf8(res.body()).unwrap();
    result = result.trim_start_matches('[');
    result = result.trim_end_matches("]\n");

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(result.to_string())
        .build())
}
