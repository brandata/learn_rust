extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn say_hello_from_rust() {
    log("Hello from rust!");
}

#[wasm_bindgen]
pub struct DougsClient {

}

#[wasm_bindgen]
impl DougsClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("New was hit!");
        Self {

        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        
    }
}