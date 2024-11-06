* wasmtime-py does not currently support running components build with componentize-py. This is because wasmtime-py does not yet support resources, which components built with componentize-py always use, since componentize-py unconditionally imports most of the wasi:cli world.

First, generate the bindings to be able to call the component from a Python host application.

```
# Get an add component that does not import the WASI CLI world
$ wget https://github.com/bytecodealliance/component-docs/raw/main/component-model/examples/example-host/add.wasm
$ python3 -m wasmtime.bindgen add.wasm --out-dir add

```
The generated package add has all of the requisite exports/imports for the component and is annotated with types to assist with type-checking and self-documentation as much as possible. Inside the package is a Root class with an add function that calls the component's exported add function. We can now write a Python program that calls add:

```
from add import Root
from wasmtime import Store

def main():
    store = Store()
    component = Root(store)
    print("1 + 2 = ", component.add(store, 1, 2))

if __name__ == '__main__':
    main()

```