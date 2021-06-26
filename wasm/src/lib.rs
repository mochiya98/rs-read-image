extern crate console_error_panic_hook;
extern crate read_image_lib;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = readImage)]
pub fn read_image(data: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    console_error_panic_hook::set_once();
    match read_image_lib::read_image(data) {
        Ok(s) => Ok(s),
        Err(err) => Err(JsValue::from_str(&err.to_string())),
    }
}
