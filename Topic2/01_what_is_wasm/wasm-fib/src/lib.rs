extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn fibonacci(n: i32) {
    alert(&format!("fibonacci sequence of n is {}", fibonacci_recursive(n)));
}

fn fibonacci_recursive(n: i32) -> u64 {
    if n < 0 {
        panic!("negative number {} is not accepted", n);
    }
    match n {
        0 => panic!("zero is not accepted"),
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci_recursive(n-1) + fibonacci_recursive(n-2)
    }
}


#[wasm_bindgen]
pub fn hello_world(name: &str) {
    alert(&format!("Hello, {}!", name));
}