from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some
from ..exports import rust_example_comp

class RustExampleComp(Protocol):

    @abstractmethod
    def eval(self, expr: rust_example_comp.Expression) -> int:
        raise NotImplementedError


