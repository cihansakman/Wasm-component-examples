"""
An example world for the component to target.
"""
from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class Operation(Enum):
    ADD = 0
    SUB = 1
    MUL = 2
    DIV = 3

@dataclass
class Expression:
    first: int
    operation: Operation
    second: int

