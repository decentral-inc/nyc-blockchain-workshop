extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
mod ecdsa;
use ecdsa::generate;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub fn hello_world(name: &str) {
    let (sk,pk) = generate();
    alert(&format!("Hello, {}! your secret key is {}", name, sk));
}