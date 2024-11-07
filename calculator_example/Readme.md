# Building and running the examples
We use [cargo-component](https://github.com/bytecodealliance/cargo-component) to build Wasm components according to the Wasm component model proposal for Rust and [wac](https://github.com/bytecodealliance/wac) to *compose* components together.

## Build Components with `cargo component`
0. `cargo component new <name> --lib` to create library (reactor) components to provide function interfaces. Otherwise it sets to --command by default to create runnable components.
1. add a dependency `package.metadata.component.target.dependencies` on a Wasm component to `Cargo.toml`
2. reference this dependency like an external crate `bindings::<name>::...` in the source code
3. build the component using `cargo component build --release` command

Additionaly, to use a component from any particular language, the **bindings** must be created by translating a Wasm component's *interface* to a representation that a specific language can understand and `cargo component` does it during the build process.

### Bindings Generation with `cargo component`
`cargo component` generates the bindings directly into the project at `src/bindings.rs` based on the resolved dependencies from `Cargo.toml`.

### `cargo component` WASI support
The `cargo component` targets `wasm32-wasip1` by default. To support WASIp2 it must be adapted to the WASIp2 supported by the component model.

## `wac` to compose components
```
wac plug calculator/target/wasm32-wasip1/release/calculator.wasm --plug adder/target/wasm32-wasip1/release/adder.wasm -o composed.wasm
wac plug command/target/wasm32-wasip1/release/command.wasm --plug composed.wasm -o final.wasm
```