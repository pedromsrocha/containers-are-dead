# Components

In the previous section, you experienced the friction of passing strings to WebAssembly functions - requiring manual pointer/length pairs and unsafe memory operations. This fundamental limitation exists on purpose to keep the core WebAssembly specification simple, portable and easy to implement.

On top of this core specification the WebAssembly Component Model introduces a rich type system that supports complex data structures like strings, records, variants, lists, and options. Components can define interfaces using WebAssembly Interface Types (WIT), enabling type-safe communication between WebAssembly modules and their hosts without manual memory management.

Where core WebAssembly modules export functions with basic numeric parameters, components export interfaces with high-level types that are automatically marshaled by the runtime. Below you can see an example of such an interface:

```wit
package wasi:random@0.2.7;

/// WASI Random is a random data API.
///
/// It is intended to be portable at least between Unix-family platforms and
/// Windows.
@since(version = 0.2.0)
interface random {
    // other functions omitted for brevity...

    /// Return a cryptographically-secure random or pseudo-random `u64` value.
    @since(version = 0.2.0)
    get-random-u64: func() -> u64;
}
```

The above interface definition is an excerpt of the real `wasi:random/random` interface that allows WebAssembly access to cryptographically secure random numbers from the host. Notice couple important pieces:

- `package wasi:random@0.2.7;` declares the _namespace_ (`wasi`) and _name_ (`random`) of the current package, as well as the current versionn (`0.2.7`).
- `interface` denotes a collection of functions and associated types and defines behaviour shareable with the outside world. You can think of it as a `trait` in Rust.
- `get-random-u64: func() -> u64;` declares that - to conform to the `random` interface - one must export a function called `get-random-u64` that takes no arguments and returns a single `u64`.
- `@since(version = 0.2.0)` is a _feature gate_ that indicated the annotated function is _stable_ since package version `0.2.0`. The component model has a first-class story for evolving and updating interfaces.

To find out what "`wasi`" is, keep on reading!

## WebAssembly System Interface (WASI)

WebAssembly System Interface (WASI) defines a set of standard interfaces for common system operations like file I/O, networking, and random number generation. As of the writing of this workshop the interfaces provided by WASI are the following:

- `wasi:clocks` Reading the current time and measuring elapsed time.
- `wasi:random` Obtaining pseudo-random, and cryptographically secure data.
- `wasi:filesystem` Accessing host filesystems. Has functions for opening, reading, and writing files, and for working with directories.
- `wasi:sockets` Adds TCP & UDP sockets and domain name lookup.
- `wasi:cli` Command-Line Interface (CLI) environment. Provides Stdio, Command-line arguments, and the concept of a `main` function.
- `wasi:http` Sending and receiving HTTP requests and responses.

Today Rust applications can easily access most these interface by simply compiling to the [`wasm32-wasip2`] target. Rust standard library types such as [`std::time::SystemTime`], [`std::net::TcpStream`], or [`std::process::Command`] will automatically make use of the WASI interfaces.

## Wasm Component in Rust

### Prerequisites

The Rust ecosystem has strong support for building WebAssembly Components. In addition to the `wasm32-wasip2` target, most functionality is provided by the [`cargo-component`] crate. In the following we will use it to build a WebAssembly component in Rust.

First install the tool from crates.io:

```bash
cargo install cargo-component
```

> Note: If you use `nix`, `cargo-component` is already provided by the dev shell flake in this repo!

### Project Structure

Now, looking at the exercise for this section you will notice a couple new things.

```plaintext
01_setup
├── src
│   └── lib.rs
├── wit
│   └── world.wit
├── tests
└── Cargo.toml
```

Let's go over the difference, starting with `Cargo.toml`:

#### `Cargo.toml`

```toml
[package]
name = "components"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[package.metadata.component]
package = "workshop:example"

[package.metadata.component.target]
path = "wit"

[package.metadata.component.target.dependencies]
"wasi:random" = "0.2.7"
```

We have added a few extra sections to the `Cargo.toml` file.

- The `[package.metadata.component]` section specifies _namespace_ and _name_ of package we're building. This is equivalent `package wasi:random@0.2.7;` syntax we've seen above.
- `[package.metadata.component.target]` tells `cargo component` about the WIT file in the `wit` directory. We'll go over this next.
- `[package.metadata.component.target.dependencies]` declares that this module depends on the `wasi:random` WASI package. This is required for `cargo component` to automatically build bindings to this import for us.

#### `./wit/world.wit`

`world.wit` is a _WebAssembly Interface Types_ declaration that defines the `interface` our component is going to export. This is the same syntax you have seen above:

```wit
package workshop:example;

/// An example world for the component to target.
world example {
    import wasi:random/random@0.2.7;

    export add-random: func(num: u64) -> u64;
}
```

This file is quite a bit simpler though, we simply declare we will be _importing_ the `wasi:random/random` interface and _exporting_ a function called `add-random` that takes in a `u64` and returns a `u64`.

### Rust Bindings

We can use `cargo component` to automatically generate **type-safe** bindings to our imports. Even better it will generate a Rust `trait` that **requires** our component to be adhere to the interface we promised above. Let's have a look!

First generate the bindings using this command:

```sh
cargo component bindings
```

It has generates a `bindings.rs` file in your `src` folder. This file looks pretty daunting but here is the gist:

- bindings to all interfaces we imported are available as submodules. In our case that is `bindings::random::random` to access the random number interface.
- `bindings::Guest` is a `trait` that we have to implement in order to be compliant with the interface we promised above.
- `bindings::export` is a macro that lets us declare a Rust type implementing `Guest` as _the_ implementation for this interface. You call it like so `bindings::export!(<YourTypeName> with_types_in bindings)` (substitue `<YourTypeName>` for the actualy name of your type).

## Exercise

The exercise for this section is quite simple again, you just need to provide an implementation for the `Guest` trait that uses the functions provided by `wasi:random/random` to add a random number to the provided one.

[`wasm32-wasip2`]: https://doc.rust-lang.org/nightly/rustc/platform-support/wasm32-wasip2.html
[`wasi`]: https://docs.rs/wasi/latest/wasi/
[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[`std::net::TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`cargo-component`]: https://crates.io/crates/cargo-component
