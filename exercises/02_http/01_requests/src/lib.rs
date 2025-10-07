use serde::Deserialize;
use serde_querystring::ParseMode;
use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;

#[derive(Deserialize)]
struct Params {
    min: Option<usize>,
    max: Option<usize>,
}

#[http_component]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    let params: Params = serde_querystring::from_str(req.query(), ParseMode::UrlEncoded)?;

    let req = Request::builder()
        .method(Method::Get)
        .uri(format!(
            "http://www.randomnumberapi.com/api/v1.0/random?min={}&max={}",
            params.min.unwrap_or(0),
            params.max.unwrap_or(1000)
        ))
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
