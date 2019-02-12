extern crate rand;
extern crate secp256k1;
use rand::OsRng;
use secp256k1::{Secp256k1};
extern crate wasm_bindgen;



use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn big_computation() {
    let a = 1;
    let b = 1;
    alert(&format!("{} + {} = {}", a,b, a+b));
}


