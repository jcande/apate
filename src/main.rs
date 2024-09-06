use wasm_bindgen::prelude::*;

mod point;
use crate::point::Vec3;
mod mesh;
use crate::mesh::Mesh;
mod camera;

mod render;
use crate::render::SimulationState;

mod instance;
use crate::instance::ObjInstance;

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
    /*
    let href = window.location().href()?;
    let url = url::Url::parse(&href)
        .or(Err(JsValue::from_str("unable to parse url")))?;
    */

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
    /*
    canvas.set_width(160 /*640*/);
    canvas.set_height(100 /*480*/);
    */
    canvas.set_width(container.offset_width().try_into().expect("god has forsaken you"));
    canvas.set_height(container.offset_height().try_into().expect("god has forsaken you"));

    let canvas_ctx = canvas
        .get_context("2d")?
        .ok_or(JsValue::from_str("unable to retrieve 2d context from domino canvas"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    log!("smoke weed every day");
    let origin = Vec3::new([0.0, 0.0, 10.0]);
    let _poly = Mesh::mk_tetra_cube();
    let poly = Mesh::mk_cube();
    let render_state = SimulationState::new(origin, poly);
    ObjInstance::<SimulationState>::new(
            window, document, canvas, canvas_ctx, render_state)
        .start_fire();

    Ok(())
}
