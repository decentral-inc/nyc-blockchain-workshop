# Topic 2-1. What is WebAssembly(WASM)?

<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c6/Web_Assembly_Logo.svg/1200px-Web_Assembly_Logo.svg.png" width="200" height="200"/>

WebAssembly is a new assembly language close that's lower level (think Assembly) and executeable all types of major browsers. WebAssembly is designed to maintain the versionless, feature-tested, and backwards-compatible nature of the web.   

It started as an upgrade to javascript, which is the only programming language on the web browser to be compiled on native level. However, all programming languages now try to support it to be compiled in the assembly language so that they can be compiled to the web without using javascript.

Rust is one of them but has more advantages than other languages with its memory-safe compiler without garbage collection. Compiled wasm from Rust has less wasm code and memory-safe on development.

To witness the power of WebAssembly in rust, check out 

[https://github.com/rustwasm/awesome-rust-and-webassembly](https://github.com/rustwasm/awesome-rust-and-webassembly)

# How to use wasm in rust

In this topic, we will make node package with wasm.

- Set nightly version as default
```bash
rustup default nightly
```

- Start new rust project with `cargo new wasm-tutorial --lib`

- Set Cargo.toml as below:
```toml
[features]
default = ["console_error_panic_hook"]

[package]
name = "wasm-tutorial"
version = "0.1.0"
authors = ["hskang9 <hskang9@gmail.com>"]
edition = "2018"


[lib]
crate-type = ["cdylib"] // only static library is supported on wasm for now


[dependencies]
cfg-if = "0.1.2"
wasm-bindgen = "=0.2.34" # this version only supports webpack currently(02/15/19)

# The `console_error_panic_hook` crate provides better debugging of panics by
# logging them with `console.error`. This is great for development, but requires
# all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
# code size when deploying.
console_error_panic_hook = { version = "0.1.1", optional = true }

# `wee_alloc` is a tiny allocator for wasm that is only ~1K in code size
# compared to the default allocator's ~10K. It is slower than the default
# allocator, however.
#
# Unfortunately, `wee_alloc` requires nightly Rust when targeting wasm for now.
wee_alloc = { version = "0.4.2", optional = true }
```


## Bind wasm code to javascript


### Building Javascript wrapper interface 

To enable the functions created from the rust on browser, we need to build a wrapper interface to be run on js file. You can think of this as *importing javascript function to rust file*. In `lib.rs` file,

```rust
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Wrapper interface
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}
```

### Producing Rust functions that Javascript can call

With wasm-bindgen, you can also export rust code with javascript functions. Only pure rust code is supported to be compiled into wasm, which means rust wrapper library is not available to export to js. 
```rust
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello person named, {}!", name));
}
```

## Building wasm into node package that can be published

To publish a node.js package, let us register an account to npm.
```bash
npm adduser your_npm_username
```

Now javascript is a compiled language with the wasm. build the package by running this command.
```bash
wasm-pack build --scope your_npm_username
```

Then run command
```bash
wasm-pack publish --access=public
```

## Use node package in node.js apps

Start with boiler plate, by installing `wasm-app` via CLI, which is this code generator [https://github.com/rustwasm/create-wasm-app](https://github.com/rustwasm/create-wasm-app)

```bash
npm init wasm-app
```

then install published pacakge 

```
npm install @your_npm_username/your_package_name
```

then import it in js files
```
import * as wasm from "@your_npm_username/your_package_name";

wasm.greet("world");

```

# Fibonacci in rust 

This package implements fibonacci in wasm. You should be in the dir    
`/nyc-blockchain-workshop/Topic2/01_what_is_wasm`   
  

```
cargo new wasm-fib --lib
cd wasm-fib
npm adduser your_npm_username
cat src/lib.rs
```
  
Replace with lib.rs in parent directory’s src folder  

```
cargo build
wasm-pack build --scope your_npm_username
wasm-pack publish --access=public
cd ../site/
npm i @your_npm_username/wasm-fib --save
```
  
Add in lines to index.js again, put in a variety of fib numbers   

```
npm start
```

In Chrome navigate to localhost:8080







