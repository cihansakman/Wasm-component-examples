#[allow(warnings)]
mod bindings;

use bindings::exports::docs::operations::operands::Guest;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    fn mul(a: u32, b: u32) -> u32 {
        a * b
    }
}

bindings::export!(Component with_types_in bindings);
