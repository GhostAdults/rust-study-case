extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::slice;

#[wasm_bindgen]
extern {
    pub fn alert(s:&str);
}

#[wasm_bindgen]
pub fn greet(name: &str){
    alert(&format!("Hi,its {}!", name));
}

#[wasm_bindgen]
extern "C" {
    pub static MEMORY: wasm_bindgen::JsValue;
}

#[wasm_bindgen]
pub fn get_memory() -> JsValue {
    MEMORY.clone()
}

#[wasm_bindgen]
pub fn alloc(len: usize) -> *mut f64 {
    let mut buffer = Vec::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

#[wasm_bindgen]
pub fn sum_array(ptr: *const f64, len: usize) -> f64 {
    let data = unsafe { slice::from_raw_parts(ptr, len) };
    data.iter().sum()
}
