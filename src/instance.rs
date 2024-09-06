use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::point::Vec2;

// lifted from the `console_log` example
#[wasm_bindgen(module = "/src/debug.js")]
extern "C" {
    #[wasm_bindgen()]
    fn dbg_break();
}

pub struct SystemContext {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub canvas_element: web_sys::HtmlCanvasElement,
    pub canvas_ctx: web_sys::CanvasRenderingContext2d,
}

pub trait Simulation {
    fn go(&mut self, ctx: &SystemContext, dims: Vec2);
}

pub struct ObjInstance<Obj> {
    sys_ctx: SystemContext,

    obj: Obj,

    frames: usize,
}

impl<Obj: Simulation + 'static> ObjInstance<Obj> {
    pub fn new(
        window: web_sys::Window,
        document: web_sys::Document,
        canvas_element: web_sys::HtmlCanvasElement,
        canvas_ctx: web_sys::CanvasRenderingContext2d,
        obj: Obj,
    ) -> Self {
        Self {
            sys_ctx: SystemContext {
                window,
                document,
                canvas_element,
                canvas_ctx,
            },

            obj: obj,

            frames: 0,
        }
    }

    pub fn start_fire(mut self) {
        fn animation_frame_thunk(callback: &Closure<dyn FnMut()>) {
            // I don't like that we pull out a web_sys::Window from thin-air but I don't know if we can
            // really do it any other way.
            let window = web_sys::window().expect("can't find window");
            window.request_animation_frame(callback
                                           .as_ref()
                                           .unchecked_ref())
                .expect("`request_animation_frame` failed (this is terrible)");
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
            self.new_frame();

            // Queue up another go.
            animation_frame_thunk(
                base_context.borrow()
                    .as_ref().expect("unable to reference `base_context` (this is bad)"));
        }));

        // Now let's get this party started RIGHT
        animation_frame_thunk(
            ref_context.borrow()
                .as_ref().expect("it better be here"));
    }

    fn new_frame(&mut self) {
        let dims = Vec2::new([
                             self.sys_ctx.canvas_element.width().into(),
                             self.sys_ctx.canvas_element.height().into()]);
        self.sys_ctx.canvas_ctx.clear_rect(0.0, 0.0, dims.x(), dims.y());

        self.obj.go(&self.sys_ctx, dims);

        self.frames += 1;
    }
}
