from .exports import rust_example_comp
import importlib_resources
import pathlib
import wasmtime

class Root:
    
    def __init__(self, store: wasmtime.Store) -> None:
        file = importlib_resources.files() / ('root.core0.wasm')
        if isinstance(file, pathlib.Path):
            module = wasmtime.Module.from_file(store.engine, file)
        else:
            module = wasmtime.Module(store.engine, file.read_bytes())
        instance0 = wasmtime.Instance(store, module, []).exports(store)
        lift_callee0 = instance0["component:rust-comp/rust-example-comp#eval"]
        assert(isinstance(lift_callee0, wasmtime.Func))
        self.lift_callee0 = lift_callee0
    def rust_example_comp(self) -> rust_example_comp.RustExampleComp:
        return rust_example_comp.RustExampleComp(self)
