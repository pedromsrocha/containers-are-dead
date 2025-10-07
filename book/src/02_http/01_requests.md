# Making HTTP requests

When making outbound HTTP requests from a Spin component, you must explicitly grant this capability in the `spin.toml` manifest file. WebAssembly's security model requires explicit permission for components to access external resources, and Spin enforces this through capability declarations in the configuration.

Take a look at the `spin.toml` manifest for this exercise, you will notice it only adds one line:

```diff
spin_manifest_version = 2

[application]
name = "requests"
version = "1.0.0"

[[trigger.http]]
route = "/..."
component = "requests"

[component.requests]
source = "../../../target/wasm32-wasip1/release/requests.wasm"
+ allowed_outbound_hosts = ["http://www.randomnumberapi.com"]

[component.requests.build]
command = "cargo build --target wasm32-wasip1 --release"
watch = ["src/**/*.rs", "Cargo.toml"]

[component.requests.tool.spin-test]
source = "tests/target/wasm32-wasip1/release/tests.wasm"
```

`allowed_outbound_hosts` is `spin`s way of declaratively request access to the specified URLs from the host.

## Making Requests

Let's have a look at an example HTTP endpoint that in-turn makes an HTTP request to `https://example.com`:

```rust
use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;

#[http_component]
pub async fn handler(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let req = Request::builder()
        .method(Method::Get)
        .uri("https://example.com")
        .build();

    // Ask the Wasm runtime to send off the request!
    let res: Response = spin_sdk::http::send(req).await?;

    // do something with the response...
}
```

You will notice the `async` keyword and use of `await` in the handler function; `spin` supports automatically yielding back control to the runtime while waiting for the network response. This quality of live feature helps your components to be more resource efficient.

Also note: making HTTP requests currently requires the use of the `spin`-specific HTTP module. This exemplifies the relatively early, in-progress nature of WebAssembly: In the absencse of standards different hosting providers built out custom APIs and capabilities. As you've seen earlier the WebAssembly System Interface (WASI) standards provides standardized interfaces that address this fragmentation but adoption across providers is still in progress.
