extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Wrapper interface
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello_world(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
