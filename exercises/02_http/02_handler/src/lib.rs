use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

// This component should:
// 1. accept HTTP POST requests only
// 2. read the input from the request body
// 3. pass that input into the calculator we implemented earlier, and return the evaluated result
// you can check your work using curl: curl --request POST --data '10 / 2' http://localhost:3000/
#[http_component]
pub fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    todo!()
}
