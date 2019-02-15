extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
mod ecdsa;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let (sk, pk) = ecdsa::generate();
    alert(&format!("Hello, {}!", sk));
}