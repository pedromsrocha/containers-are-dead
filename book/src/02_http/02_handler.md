# HTTP Handlers

`spin` allows you to annotate a function with the `#[http_component]` attribute. This marks it as the
HTTP request entry point. This handler receives a `Request` object containing the HTTP request data and must return a `Result` with a type that implements `IntoResponse` (such as a `Response` or string). The async variant allows for non-blocking operations like making outbound HTTP requests or database calls, where the runtime can yield control while waiting for I/O operations to complete.

```rust
#[http_component]
pub fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
  //...
}

// or as we've seen earlier:
#[http_component]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
  //...
}
```

When `spin` receives a request matching the route pattern defined in `spin.toml` it will forward the request to the entry point set up above. In our case that pattern is `/...` which matches all paths:

```toml
[[trigger.http]]
route = "/..."
component = "handler"
```

## Error Handling

When Rust code panics inside a WebAssembly component, it triggers a WebAssembly _trap_[^1]. Unlike traditional server applications where this would crash the entire system, WebAssembly's sandbox means all we get is a rather ugly stack trace in the terminal.

Instead of relying on panics, HTTP endpoint should use "proper" error handling using `Result`s. `spin` HTTP endpoints conveniently already require us to return `anyhow::Result<impl IntoResponse>`. This means we can propagate server-internal errors (errors that we want to show up as 5xx status code responses) simply using the `?` operator.

_Client errors_ i.e. errors we want to return in response to invalid user input still require us to build and return `Response`s ourselves.

- bring in the evaluator from previous chapter and hook it up to the HTTP handler

[^1]: A trap is similar to a segfault. It cannot be caught and will cause your Wasm component to be terminated.
