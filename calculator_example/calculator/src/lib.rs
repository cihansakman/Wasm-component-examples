#[allow(warnings)]
mod bindings;

use bindings::exports::docs::calculator::calculate::{Guest, Op};

//bring the import and function
use bindings::docs::operations::operands::{add, mul};

struct Component;

impl Guest for Component {
    fn eval_exprassion(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add(x, y),
            Op::Mul => mul(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
