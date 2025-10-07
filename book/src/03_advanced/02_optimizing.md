# Optimizing

## Optimizing for Size

Smaller modules load faster, improving cold start performance and in turn reducing the latency between request arrival and function execution - a critical factor for user-facing applications where every millisecond counts.

> Competition! At the end of this workshop we will compare our final *optimized* database examples to see who got the smallest!

Let's start with our current databases example. A debug build produces a WebAssembly module of roughly *19 MB*: clearly room for improvement.

We can drastically improve the size by compiling in release mode with optimizations, if we additionally add this config to our *root* `Cargo.toml` file we can usually squeeze some bytes more:

```toml
[profile.release]
codegen-units = 1 # don't parallelize compilation of crates, this can make optimizations more effective
lto = true # use "fat" link time optimization that operates on the entire dependency graph, again potentially making optimizations more effective
opt-level = "z" # instruct the compiler to aggressively optimize for small code size, sometimes "s" can be a better option here
strip = true # strip debug symbols from the release binary
```

We can instruct `spin` to build our component in release mode by specifying the `--release` flag and pointing it to use the release build like so:

```diff
spin_manifest_version = 2

[application]
name = "databases"
version = "1.0.0"

[[trigger.http]]
route = "/..."
component = "databases"

[component.databases]
-source = "../../../target/wasm32-wasip1/debug/databases.wasm"
+source = "../../../target/wasm32-wasip1/release/databases.wasm"
sqlite_databases = ["default"]

[component.databases.build]
-command = "cargo build -p databases --target wasm32-wasip1"
+command = "cargo build -p databases --target wasm32-wasip1 --release"
watch = ["src/**/*.rs", "Cargo.toml"]

[component.databases.tool.spin-test]
source = "../../../target/wasm32-wasip1/debug/databases_tests.wasm"
```

We get a release binary that comes in at *443 KB*, but we can go further. Binaryen a WebAssembly optimizer and compiler toolchain provides a widely used tool called `wasm-opt` that can preprocess Wasm modules and components.

`wasm-opt` is a bit annyoing to install, but you can download it from their GitHub releases [here](https://github.com/WebAssembly/binaryen/releases). Alternatively it appears to be in some package repositories either under the `binaryen` or `wasm-opt` name. Again if you use `nix` the flake in this repo already provides `wasm-opt`.

By running `wasm-opt` on our binary like so `wasm-opt target/wasm32-wasip1/release/databases.wasm -O3 -o optimized.wasm` (`-O3` standards for all optimizations at their highest aggressiveness) we get the binary size down to *335 KB*.

## Optimizing for Performance

Size optimization is particularly important for serverless deployments, but nothing beats optimizing for speed. Unfortunately, there are not many WebAssembly specific solutions out there. Lack of profiling tooling has been identified as a major blocker; that unfortunately isn't resolved yet.

In spite of this, some tooling can be adapted to work with Wasm, for example `criterion` the Rust benchmarking framework already works inside WebAssembly today. Wasmtime supports profiling the entire runtime (including your WebAssembly code but also the runtimes code).

One interesting WebAssembly specific performance optimization tool exist that shouldn't go unmentioned: [`wizer`] is a so-called "preinitializer" for WebAssembly modules. It takes your module or component, runs its initialization function, and then snapshots the bytes of the initialized state into a new Wasm module. Depending on your exact application this weird sounding trick can actually meaningfully improve the startup.

[`criterion`]: https://github.com/bheisler/criterion.rs
[`wizer`]: https://github.com/bytecodealliance/wizer
