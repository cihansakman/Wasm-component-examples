# Custom Numpy build with componentize-py

based on the [matrix-math example](https://github.com/bytecodealliance/componentize-py/tree/main/examples/matrix-math)

## NumPy for target wasm32_unknown_wasi
The `componentize-py` team has developed a custom Numpy build that targets `wasm32-unknown-wasi`, allowing it to run within any WASI-supported WebAssembly runtime. Additionally, other native Python libraries compatible with `wasm32-wasi` can be found in this [repository](https://github.com/dicej/wasi-wheels/), which includes custom builds of popular libraries.
## Run with

### Generating the bindings (optional)
While `componentize-py` automatically creates bindings during the componentization process, you may still want to explicitly generate them to view or customize the bindings. This can be done as follows:

```
componentize-py --wit-path wit/ --world matrix-math bindings .
```

Componentize the component and run the Wasm binary executable with wasmtime using cli commands.
```
componentize-py -d wit/ -w matrix-math componentize app -o matrix-math.wasm
wasmtime run matrix-math.wasm '[[1, 2], [4, 5], [6, 7]]' '[[1, 2, 3], [4, 5, 6]]'
```