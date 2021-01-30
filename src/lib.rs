use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
pub struct Mirror {
    n: i32
}

#[wasm_bindgen]
impl Mirror {
    #[wasm_bindgen(constructor)]
    pub fn new(_n: i32) -> Mirror {
        Mirror {
            n: _n
        }
    }

    #[wasm_bindgen(method)]
    pub fn talk(&self) -> String {
        return self.to_string();
    }
}

impl fmt::Display for Mirror {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Mirror value: {}", &self.n)
    }
}

#[wasm_bindgen]
pub fn main() {}