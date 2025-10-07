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

// This endpoint should delete the <ident>=>value mapping match the key `var`
// from the SQLite database
// If you get stuck, ask a neighbor or grab a trainer!
fn delete(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    todo!()
}

// This endpoint should clear ALL <ident>=>value mappings for the SESSION
// If you get stuck, ask a neighbor or grab a trainer!
fn clear(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    todo!()
}
