use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::camera::Camera;
use crate::mesh::Mesh;
use crate::point::Vec2;

use crate::render;

// This is recommended for debug builds. Panics will be logged to the console.
extern crate console_error_panic_hook;

// XXX RULE: EVERYTHING is a Point. Vectors don't exist. It is too confusing.

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// lifted from the `console_log` example
#[wasm_bindgen(module = "/src/debug.js")]
extern "C" {
    #[wasm_bindgen()]
    fn dbg_break();
}

struct System_Context {
    window: web_sys::Window,
    document: web_sys::Document,
    canvas_element: web_sys::HtmlCanvasElement,
    canvas_ctx: web_sys::CanvasRenderingContext2d,
}

struct Scene {
    camera: Camera,
    objects: Vec<Mesh>,
}

pub struct Apate_Instance {
    sys_ctx : System_Context,

    // space?
    scene: Scene,

    frames: usize,
}

impl Apate_Instance {
    pub fn new(
        window: web_sys::Window,
        document: web_sys::Document,
        canvas_element: web_sys::HtmlCanvasElement,
        canvas_ctx: web_sys::CanvasRenderingContext2d,
        camera: Camera,
        objects: Vec<Mesh>,
    ) -> Self {
        Apate_Instance {
            sys_ctx: System_Context {
                window,
                document,
                canvas_element,
                canvas_ctx,
            },

            scene: Scene {
                camera,
                objects,
            },

            frames: 0,
        }
    }

    pub fn new_frame(&mut self) {
        log!("new frame alert: {}", self.frames);
        let dims = Vec2::new([
                             self.sys_ctx.canvas_element.width().into(),
                             self.sys_ctx.canvas_element.height().into()]);
        self.sys_ctx.canvas_ctx.clear_rect(0.0, 0.0, dims.x(), dims.y());
        render::go(
            &self.sys_ctx.canvas_ctx,
            dims,
            &self.scene.camera,
            &mut self.scene.objects
            );
        log!("frame {} done\n\n\n", self.frames);
if self.frames == 136 {
        dbg_break();
}

        self.frames += 1;
    }
}

pub fn firestarter(mut instance: Apate_Instance) {
    fn animation_frame_thunk(callback: &Closure<dyn FnMut()>) {
        // I don't like that we pull out a web_sys::Window from thin-air but I don't know if we can
        // really do it any other way.
        let window = web_sys::window().expect("can't find window");
        window.request_animation_frame(callback
                                       .as_ref()
                                       .unchecked_ref())
            .expect("I have no fucking clue");
    }

    // The idea here is that `base_context` is the "real" one and is initially constructed as some
    // empty shell. Later on, we use the reference (i.e., `ref_context`) to populate the underlying
    // structure. The way we do this is to construct a closure inside the option which references
    // `base_context`. Once this value is constructed, we kick-start everything using the reference
    // before dropping it. This is tricky. You probably don't understand it just from reading this.
    let base_context = Rc::new(RefCell::new(None));
    let ref_context = base_context.clone();

    *ref_context.borrow_mut() = Some(Closure::new(move || {
        // NB this is the logic that gets invoked each animation frame!
        instance.new_frame();
        animation_frame_thunk(
            base_context.borrow()
                .as_ref().expect("it better be here"));
    }));

    // Now let's get this party started RIGHT
    animation_frame_thunk(
        ref_context.borrow()
            .as_ref().expect("it better be here"));
}
