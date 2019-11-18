//! Rust contract that uses conditional compilation.

use near_bindgen::near_bindgen;
use borsh::{BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Incrementer {
    value: u32,
}

#[near_bindgen]
impl Incrementer {
    #[cfg(feature = "myfeature")]
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }

    #[cfg(not(feature = "myfeature"))]
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

fn main() {}
