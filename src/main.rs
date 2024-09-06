use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::BorrowMut;

use url;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod point;
use crate::point::Point;
use crate::point::Vec3;
mod mesh;
use crate::mesh::Mesh;
mod camera;
use crate::camera::Camera;

mod render;

mod instance;
use crate::instance::Apate_Instance;

// This is recommended for debug builds. Panics will be logged to the console.
extern crate console_error_panic_hook;

// XXX RULE: EVERYTHING is a Point. Vectors don't exist. It is too confusing.

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/*

The good shit:
    https://www.davrous.com/2013/06/13/tutorial-series-learning-how-to-write-a-3d-soft-engine-from-scratch-in-c-typescript-or-javascript/

Big picture:
    each mesh should have an origin, and rotation (the rotation is done entirely in the world transform matrix)
    each tick do the thing
        project each mesh onto something (is this the framebuffer analogue?)
        clip the vertices in a pleasing fashion

    project - ???
        maybe put a square around entire mesh and for each vertex clip out a circle?
        maybe - https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Compositing
    clip - ???

// render(camera, cube)
//      calculate viewMatrix from camera
//          how?
//              with Matrix.LookAtLH
//      calculate a projectionMatrix from width/height and magic
//          is this a constant?
//              built via Matrix.PerspectiveFovLH
//      iterate over each mesh
//          calculate the worldMatrix by multiplying the rotation of the mesh with its position
//          calculate the transformMatrix by multiplying the worldMatrix with the viewMatrix with the projectionMatrix
//          iterate over each vertex in the mesh
//              calculate the projected point using the vertex and the transformMatrix
//              draw the point to the screen

*/

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

    let origin = Vec3::new([0.0, 0.0, 10.0]);
    let camera = Camera::new(origin);
    let poly = mesh::Mesh::mk_tetra_cube();
    let poly = mesh::Mesh::mk_cube();

    log!("smoke weed every day");
    let mut instance = Apate_Instance::new(window, document, canvas, canvas_ctx, camera, vec![poly]);
    instance::firestarter(instance);

    Ok(())
}
