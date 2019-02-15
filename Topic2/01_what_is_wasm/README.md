# Topic 2-1. What is WebAssembly(WASM)?

![webassembly](https://upload.wikimedia.org/wikipedia/commons/thumb/c/c6/Web_Assembly_Logo.svg/1200px-Web_Assembly_Logo.svg.png)

WebAssembly is a new assembly language close to metal which is shared all types of major browsers. 

It started as an upgrade to javascript, which is the only programming language on the web browser to be compiled on native level. However, all programming languages now try to support it to be compiled in the assembly language so that they can be compiled to the web without using javascript.

Rust is one of them but has more advantages than other languages with its memory-safe compiler without garbage collection. Compiled wasm from Rust has less wasm code and memory-safe on development.

To witness the power of WebAssembly in rust, check out 

[awesome-wasm](https://github.com/rustwasm/awesome-rust-and-webassembly)

# How to use wasm in rust

In this topic, we will make node package with wasm.

- Set nightly version as default
```bash
rustup default nightly
```

- Start new rust project with `Cargo new wasm-tutorial`

- Set Cargo.toml as below:
```toml
[package]
name = "wasm-tutorial"
version = "0.1.0"
authors = ["hskang9 <hskang9@gmail.com>"]
edition = "2018"


[lib]
crate-type = ["cdylib"] // only static library is supported on wasm for now


[dependencies]
wasm-bindgen="0.2"
```


## Bind wasm code to javascript


### Building Javascript wrapper interface 

To enable the functions created from the rust on browser, we need to build a wrapper interface to be run on js file. You can think of this as *importing javascript function to rust file*.

```rust
extern create wasm_bindgen;
use wasm_bindgen::prelude::*;

// Wrapper interface
extern {
    fn alert(s: &str);
}
```

### Producing Rust functions that Javascript can call

With wasm-bindgen, you can also export rust code with javascript functions.
```rust
#[wasm_bindgen]
pub fn hello_world(name: &str) {
    alert(&fromat!("Hello, {}!", name));
}
```

## Building wasm into node package that can be published

Now javascript is a compiled language with the wasm. build the package by running this command.
```bash
wasm-pack build 
```

Then go to pkg folder and you can use npm commands for this.
```bash
cd pkg/

npm publish --access=public
```





