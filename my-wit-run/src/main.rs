use wasmtime::component::*;
use wasmtime::{Engine, Store};

bindgen!({
    path: "../my-wit-module/wit/host.wit",
});

struct MyState {
    name: String,
}

impl HelloWorldImports for MyState {
    fn name(&mut self) -> String {
        self.name.clone()
    }
}

fn main() -> wasmtime::Result<()> {
    // Compile the `Component` that is being run for the application.
    let engine = Engine::default();
    let component = Component::from_file(&engine, "../my-wit-module/target/wasm32-wasip1/debug/my_wit_module.wasm")?;

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated `add_to_linker`
    // method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
    // trait. In this case the `T`, `MyState`, is stored directly in the
    // structure so no projection is necessary here.
    let mut linker = Linker::new(&engine);
    HelloWorld::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let mut store = Store::new(
        &engine,
        MyState {
            name: "me".to_string(),
        },
    );
    let (bindings, _) = HelloWorld::instantiate(&mut store, &component, &linker)?;

    // Here our `greet` function doesn't take any parameters for the component,
    // but in the Wasmtime embedding API the first argument is always a `Store`.
    bindings.call_greet(&mut store)?;
    Ok(())
}