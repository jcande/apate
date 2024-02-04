use url;
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

    let window = web_sys::window()
        .ok_or(JsValue::from_str("no global window exists"))?;
    let href = window.location().href()?;
    let url = url::Url::parse(&href)
        .or(Err(JsValue::from_str("unable to parse url")))?;

    let document = window
        .document()
        .ok_or(JsValue::from_str("should have a document on window"))?;
    // the intent is to grab it and then we can expand/contract the canvas with this.
    let container = document.get_element_by_id("illusion-div")
        .ok_or(JsValue::from_str("unable to locate domino container \"illusion-div\" in document"))?
        .dyn_into::<web_sys::HtmlElement>()?;
    let canvas = document.get_element_by_id("phantasm")
        .ok_or(JsValue::from_str("unable to locate domino canvas \"phantasm\" in document"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let canvas_ctx = canvas
        .get_context("2d")?
        .ok_or(JsValue::from_str("unable to retrieve 2d context from domino canvas"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    log!("smoke weed every day");

        let x = 100.0;
        let y = 100.0;
        let tile_width = 50.0;
        let tile_height = 50.0;
        canvas_ctx.save();
        {
            // Set this coordinate to be the origin.
            canvas_ctx.translate(x, y)
                .expect("oh god how can this fail?");

            // Draw a line
            canvas_ctx.begin_path();
                    canvas_ctx.move_to(0.0, 0.0);
                    canvas_ctx.line_to(tile_width, 0.0);
                    canvas_ctx.line_to(tile_width / 2.0, tile_height / 2.0);
                    canvas_ctx.line_to(0.0, 0.0);
            canvas_ctx.close_path();

            // This is dumb. Can we really not give it a more direct value?
            let color = 0x0000ff;
            let s = format!("#{:0>6x}", color);
            let color = JsValue::from_str(&s);
            canvas_ctx.set_fill_style(&color);

            canvas_ctx.fill();
        }
        canvas_ctx.restore();

    Ok(())
}
