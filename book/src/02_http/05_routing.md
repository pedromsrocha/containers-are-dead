# Routing

goal: understand how in-component routing works, understand how composition works, understand how inter-component routing works

## Routing Within A Component

So far we have only ever seen a single catch-all (`/...`) endpoint per component. But what if you want to have multiple endpoint? Spin's `Router` API allows you to delegate requests to different handler functions. Instead of manually parsing URLs and HTTP methods in your handler function, the router allows you to declaratively map routes to handler functions:

```rust
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

#[http_component]
fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/users/:id", get_user);
    router.post("/users", create_user);
    router.handle(req)
}

fn get_user(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let user_id = params.get("id").unwrap();
    // look up the user, or something...
}

fn create_user(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    // look up the user, or something...
}
```

The router supports path parameters (`:id`) that are extracted and passed to handlers via the `Params` argument. Each handler function receives the original request and extracted parameters, returning the same `anyhow::Result<impl IntoResponse>` type as the main component handler.

## Routing between multiple Components

Spin applications can be composed of multiple WebAssembly components, each handling different routes. This is configured in the `spin.toml` manifest by defining multiple `[[trigger.http]]` sections, each mapping a route pattern to a specific component:

```toml
[application]
name = "multi-component-app"
version = "1.0.0"

[[trigger.http]]
route = "/api/users/..."
component = "user-service"

[[trigger.http]]
route = "/api/orders/..."
component = "order-service"

[component.user-service]
source = "target/wasm32-wasip1/release/users.wasm"

[component.order-service]
source = "target/wasm32-wasip1/release/orders.wasm"
```

When a request arrives, Spin's HTTP trigger examines the URL path and matches it against the configured routes in order of specificity. The request is then forwarded to the corresponding WebAssembly component.

This allows to to compose applications from modular components - recall the component composition from earlier - that can be developed, tested, and deployed independently.

- point to Rust + Go demo for inter-language composition
  exercise: expand expression evaluator to support assigning values to variables. Store the variables in the spin-sdk KV store.
