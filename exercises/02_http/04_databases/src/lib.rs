use anyhow::{Context, bail};
use fallible_iterator::FallibleIterator;
use serde::{Deserialize, Serialize};
use serde_querystring::ParseMode;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};
use std::str::Chars;
use std::str::FromStr;

// curl --request POST --data '10 / 2' http://localhost:3000/
// curl --request POST --data '10 / 2' http://localhost:3000?session=<ID>

#[derive(Deserialize)]
struct QueryParams {
    session: Option<i64>,
}

#[derive(Serialize)]
struct EvalResponse {
    session: i64,
    result: Option<f64>,
}

// This component should:
// 1. parse and evaluate input expressions as before
// 2. it should NOT store assigned <ident>=>value pairs in the KV store but instead in the provided SQLite database's variables table.
//    see the `migration.sql` file for a detailed shape of the table.
// 3. It should support session ID to handle multiple concurrent users. A session ID may be provided via the query parameters (see the struct above)
//    and IF PRESENT should be used to select the matching rows the table. IF NOT PRESENT we should generate one (e.g. using the sessions table).
// 4. Variables stored in one session MUST NOT be present in another session
// If you get stuck, ask a neighbor or grab a trainer!
#[http_component]
pub fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    todo!()
}
