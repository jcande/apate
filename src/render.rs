use crate::point::Vec2;
use crate::point::Vec3;
use crate::point::Mat4;
use crate::Mesh;
use crate::camera::Camera;

use crate::instance::Simulation;
use crate::instance::SystemContext;

use wasm_bindgen::prelude::*; // XXX this makes me sad
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
/*
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
*/

pub struct SimulationState {
    camera: Camera,
    meshes: Vec<Mesh>,
}

impl SimulationState {
    pub fn new(origin: Vec3, poly: Mesh) -> Self {
        Self {
            camera: Camera::new(origin),
            meshes: vec![poly],
        }
    }
}

// maybe break this up into input(), step(), and render()?
impl Simulation for SimulationState {
    fn go(&mut self, ctx: &SystemContext, dims: Vec2) {
        // no input to grab
        tic(&dims, &self.camera, &mut self.meshes);
        render(&ctx.canvas_ctx, &dims, &self.camera, &mut self.meshes);
    }
}

pub fn tic(_dims: &Vec2, _camera: &Camera, meshes: &mut Vec<Mesh>) {
    for mesh in meshes.iter_mut() {
        mesh.rotation.coord[0] += 0.005;
        mesh.rotation.coord[1] += 0.005;
        /*
        mesh.rotation.coord[2] += 0.0005;
        */
    }
}

pub fn render(canvas_ctx: &web_sys::CanvasRenderingContext2d, dims: &Vec2, camera: &Camera, meshes: &mut Vec<Mesh>) {
    let colors = [
        JsValue::from_str(&format!("#{:0>6x}", 0xff00ff)), // pink?
        JsValue::from_str(&format!("#{:0>6x}", 0xff0000)), // red
        JsValue::from_str(&format!("#{:0>6x}", 0x00ff00)), // lime green
        JsValue::from_str(&format!("#{:0>6x}", 0x00bfff)), // deep-sky blue
        JsValue::from_str(&format!("#{:0>6x}", 0xf0ead6)), // yellowish-white/eggshell
        JsValue::from_str(&format!("#{:0>6x}", 0x6f4e37)), // coffee
        JsValue::from_str(&format!("#{:0>6x}", 0xb00b1e)),
        JsValue::from_str(&format!("#{:0>6x}", 0xc0ffee)),
    ];
    let line_color = &colors[0];
    let bubble_color = &colors[3];
    /*
    let line_color = &JsValue::from_str(&format!("#{:0>6x}", 0xffffff));
    let bubble_color = &JsValue::from_str(&format!("#{:0>6x}", 0x000000));
    */

    // calculate view_matrix from camera
    let view_matrix = Mat4::look_at_lh(camera.origin, camera.target, camera.up);

    // calculate a projection_matrix from width/height and magic
    let width = dims.x();
    let height = dims.y();
    let projection_matrix = Mat4::perspective_fov_lh(0.78, width / height, 0.01, 1.0);

    let mut points = Vec::<Vec2>::new();
    let mut lines = Vec::<(Vec2, Vec2)>::new();
    for mesh in meshes.iter_mut() {
        // calculate the world_matrix by multiplying the rotation of the mesh with its position
        let world_matrix = Mat4::rotation_yaw_pitch_roll(
                            mesh.rotation.y(),
                            mesh.rotation.x(),
                            mesh.rotation.z()) *
                                Mat4::translation(
                                    mesh.origin.x(),
                                    mesh.origin.y(),
                                    mesh.origin.z());

        let transform_matrix = world_matrix * view_matrix.clone() * projection_matrix.clone();

        for vertex in &mesh.vertices {
            let projected_coord = Vec2::project(&dims, vertex, &transform_matrix);
            points.push(projected_coord);
        }

        for (a, b) in mesh.lines
            .iter()
            .map(|(a,b)| (&mesh.vertices[*a], &mesh.vertices[*b]))
        {
            let projected_coord_a = Vec2::project(&dims, a, &transform_matrix);
            let projected_coord_b = Vec2::project(&dims, b, &transform_matrix);
            lines.push((projected_coord_a, projected_coord_b));
        }
    }

    canvas_ctx.save();

    /*
    canvas_ctx.set_fill_style(line_color);
    canvas_ctx.fill_rect(0.0, 0.0, dims.x(), dims.y());
    */

    // Make vertex bubble mask
    canvas_ctx.begin_path();
    canvas_ctx.set_fill_style(bubble_color);
    for point in points {
        draw_point(canvas_ctx, &point);
    }
    canvas_ctx.fill();
    canvas_ctx.close_path();
    canvas_ctx.clip();

    for (a, b) in lines.iter() {
        draw_line(line_color, canvas_ctx, a, b);
    }

    canvas_ctx.restore();
}

pub fn draw_point(canvas_ctx: &web_sys::CanvasRenderingContext2d, coord: &Vec2) {
    canvas_ctx.move_to(coord.x(), coord.y());
    canvas_ctx.arc(coord.x(), coord.y(), 100.0, 0.0, 2.0 * 3.14159).expect("better be defined");
}

pub fn draw_line(color: &JsValue, canvas_ctx: &web_sys::CanvasRenderingContext2d, coord_a: &Vec2, coord_b: &Vec2) {
    canvas_ctx.set_stroke_style(color);
    canvas_ctx.set_line_width(10.0);

    canvas_ctx.begin_path();
    canvas_ctx.move_to(coord_a.x(), coord_a.y());
    canvas_ctx.line_to(coord_b.x(), coord_b.y());
    canvas_ctx.close_path();

    canvas_ctx.stroke();
}
