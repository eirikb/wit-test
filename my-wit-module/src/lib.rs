#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn greet() {
        println!("Hello, world!");
    }
}

bindings::export!(Component with_types_in bindings);