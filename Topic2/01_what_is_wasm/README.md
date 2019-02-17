# Topic 2-1. What is WebAssembly(WASM)?

![webassembly](https://upload.wikimedia.org/wikipedia/commons/thumb/c/c6/Web_Assembly_Logo.svg/1200px-Web_Assembly_Logo.svg.png)

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

```bash
cargo build
```   
    

- Start new rust project with    
  
```bash
cargo new wasm-tutorial
```

- Example TOML file, however we won't be using this

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

This file above is the `lib.rs` built after making cargo lib project located in `./nyc-blockchain-workshop/Topic2/01_what_is_wasm/wasm-tutorial/src/lib.rs`  


```bash
cd wasm-tutorial
cargo build
cargo run
```

Should see  
  
```
Lings-MBP:wasm-tutorial lmeng$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/wasm-tutorial`
Hello, world!
```


   


## Bind wasm code to javascript


### Building Javascript wrapper interface 

To enable the functions created from the rust on browser, we need to build a wrapper interface to be run on js file. You can think of this as *importing javascript function to rust file*.

```rust
extern create wasm_bindgen;
use wasm_bindgen::prelude::*;

// Wrapper interface
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}
```

### Producing Rust functions that Javascript can call

With wasm-bindgen, you can also export rust code with javascript functions. Only rust code is supported to be compiled into wasm, which means rust wrapper is not available. 
```rust
#[wasm_bindgen]
pub fn hello_world(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

## Building wasm into node package that can be published

Ensure you have `wasm-pack` installed  
  
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```


To publish a node.js package, let us register an account to npm.
```bash
npm adduser your_npm_username
```

If you have an NPM account already, use the username and email associated with that. Otherwise just register for a new one. Please remember your password.   
  

Now javascript is a compiled language with the wasm. build the package by running this command.
```bash
wasm-pack build --scope your_npm_username
```

IMPORTANT: Before you can publish, you must verify your email.   

In Chrome go to [https://www.npmjs.com/](https://www.npmjs.com/)  and hit login, and login with the credentials you just made. 

Then run command
```bash
wasm-pack publish --access=public
```

## Use node package in node.js apps

Start with boiler plate

```bash
npm init wasm-app
```

then install published pacakge, ensure there is `@` appended to the left of your username in cmd below  

```
npm install @your_npm_username/your_package_name
```

Result of success should look like  

```
Lings-MBP:wasm-tutorial lmeng$ npm install @lingqingmeng/wasm-tutorial
npm notice created a lockfile as package-lock.json. You should commit this file.
npm WARN wasm-app@1.0.0 No description
npm WARN wasm-app@1.0.0 No repository field.

+ @lingqingmeng/wasm-tutorial@0.1.0
added 1 package and audited 1 package in 1.574s
found 0 vulnerabilities
```
   

then import it in js files  

```
import * as wasm from "@your_npm_username/your_package_name;

wasm.greet("world");

```





### Reference



