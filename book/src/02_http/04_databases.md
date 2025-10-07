# Databases

Key value stores are simple and easy to use, but they have two big disadvantages: no support for more complex data types and queries AND they are global. Meaning different users of your API could override each other's variables!

For "proper" persistent data storage `spin` also supports SQL databases like [`SQLite`]. To use SQL databases in Spin, you need to declare access in your `spin.toml` manifest and then use the bindings from the `spin_sdk` crate. In our case that is the [`spin_sdk::sqlite`] module.

## Database Configuration

First, add database access to your `spin.toml`:

```diff
[component.databases]
source = "../../../target/wasm32-wasip1/debug/databases.wasm"
+sqlite_databases = ["default"]
```

## Connecting to the Database

Once you've configured database access in your manifest, you can establish a connection using the `Connection::open_default()` method. This opens the SQLite database that Spin provides to your component:

```rust
use spin_sdk::sqlite::{Connection, Value};

let connection = Connection::open_default()?;

// do something with the connection...
```

## Inserting Data

You can insert data into your database using the [`execute`] method that takes a SQL query string and an array of parameters. Parameters use `?` placeholders in the query and are provided as `Value` enum variants to ensure type safety.

```rust
use spin_sdk::sqlite::{Connection, Value};

let connection = Connection::open_default()?;

let execute_params = [
    Value::Text("john".to_string()),
    Value::Integer(20)
];

// Insert a new user passing the parameters we set up
let session_result = connection.execute(
    "INSERT INTO users (name, age) VALUES (?, ?)",
    &execute_params,
)?;
```

## Retrieving Data

The [`execute`] method returns a [`QueryResult`] that you can iterate over to access query results (rows and columns). Each row provides typed access to column values through the [`get()`] method, which handles the conversion from [`SQLite`]'s storage types to Rust types.

```rust
use spin_sdk::sqlite::{Connection, Value};

let connection = Connection::open_default()?;

let rowset = connection.execute(
    "SELECT name, age FROM users WHERE age > ?",
    &[Value::Integer(18)],
)?;

for row in rowset.rows() {
    let name: String = row.get("name").unwrap();
    let age: i64 = row.get("age").unwrap();
    println!("User: {}, Age: {}", name, age);
}
```

[`SQLite`]: https://www.sqlite.org
[`spin_sdk::sqlite`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/index.html
[`execute`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.Connection.html#method.execute
[`QueryResult`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.QueryResult.html
[`get()`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.RowResult.html#method.get
