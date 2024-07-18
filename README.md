My struggle to make wasmtime work with wit-bindgen.

Based on https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_0_hello_world/index.html

### Pre-requisites

- Install Rust: https://www.rust-lang.org/tools/install
- Install cargo-component: `cargo install cargo-component`

### Run

- `make build-module`
- `make run-module`

Error I get:
> Error: component imports instance `wasi:cli/environment@0.2.0`, but a matching implementation was not found in the
> linker
>
> Caused by:
> 0: instance export `get-environment` has the wrong type
> 1: function implementation is missing


Just to prove the module works run:

- `make run-module-node`
