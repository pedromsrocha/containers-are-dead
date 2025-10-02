use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

// curl --request POST --data '10 / 2' http://localhost:3000/

#[http_component]
fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.delete("/clear", clear);
    router.delete("/clear/:var", delete);
    router.handle(req)
}

fn delete(_: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let var = params.get("var").unwrap();

    let store = spin_sdk::key_value::Store::open_default()?;
    store.delete(var).unwrap();

    Ok(Response::builder().status(200).build())
}

fn clear(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let store = spin_sdk::key_value::Store::open_default()?;

    for k in store.get_keys().unwrap() {
        store.delete(&k).unwrap();
    }

    Ok(Response::builder().status(200).build())
}
