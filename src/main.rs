use wasm_bindgen::prelude::*;

// This is recommended for debug builds. Panics will be logged to the console.
extern crate console_error_panic_hook;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log!("smoke weed every day");
    Ok(())
}
