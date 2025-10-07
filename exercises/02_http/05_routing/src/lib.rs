use serde::Deserialize;
use serde_querystring::ParseMode;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};

// curl --request POST --data '10 / 2' http://localhost:3000/

#[derive(Deserialize)]
struct QueryParams {
    session: i64,
}

#[http_component]
fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.delete("/clear", clear);
    router.delete("/clear/:var", delete);
    router.handle(req)
}

fn delete(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let var = params.get("var").unwrap();
    let params: QueryParams = serde_querystring::from_str(req.query(), ParseMode::UrlEncoded)?;

    let connection = Connection::open_default()?;

    connection.execute(
        "DELETE FROM variables WHERE session_id = (?) AND key = (?)",
        &[Value::Integer(params.session), Value::Text(var.to_string())],
    )?;

    Ok(Response::builder().status(200).build())
}

fn clear(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let params: QueryParams = serde_querystring::from_str(req.query(), ParseMode::UrlEncoded)?;

    let connection = Connection::open_default()?;

    connection.execute(
        "DELETE FROM variables WHERE session_id = (?)",
        &[Value::Integer(params.session)],
    )?;

    Ok(Response::builder().status(200).build())
}
