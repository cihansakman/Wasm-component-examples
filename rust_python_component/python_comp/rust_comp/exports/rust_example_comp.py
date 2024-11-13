from ..intrinsics import _clamp
from dataclasses import dataclass
from enum import Enum
import wasmtime

from typing import TYPE_CHECKING
if TYPE_CHECKING:
  from .. import Root

class Operation(Enum):
    ADD = 0
    SUB = 1
    MUL = 2

@dataclass
class Expression:
    first: int
    operation: Operation
    second: int

class RustExampleComp:
    component: 'Root'
    
    def __init__(self, component: 'Root') -> None:
        self.component = component
    def eval(self, caller: wasmtime.Store, expr: Expression) -> int:
        record = expr
        field = record.first
        field0 = record.operation
        field1 = record.second
        ret = self.component.lift_callee0(caller, _clamp(field, 0, 4294967295), (field0).value, _clamp(field1, 0, 4294967295))
        assert(isinstance(ret, int))
        return ret & 0xffffffff
    