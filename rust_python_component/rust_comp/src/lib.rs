#[allow(warnings)]
mod bindings;

use bindings::exports::component::rust_comp::rust_example_comp::{Expression, Guest, Operation};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn eval(expr: Expression) -> u32 {
        let (l, r) = (expr.first, expr.second);
        match expr.operation {
            Operation::Add => l + r,
            Operation::Sub => l - r,
            Operation::Mul => l * r,
            //Operation::Div => l / r,
        }
    }
}

bindings::export!(Component with_types_in bindings);
