use crate::point::Point;
use crate::point::Vec2;
use crate::point::Vec3;
use crate::point::Vec4;
use crate::point::Mat4;
use crate::Mesh;
use crate::camera::Camera;

use wasm_bindgen::prelude::*; // XXX this makes me sad
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// render(camera, cube)
//      calculate viewMatrix from camera
//      calculate a projectionMatrix from width/height and magic
//      iterate over each mesh
//          calculate the worldMatrix by multiplying the rotation of the mesh with its position
//          calculate the transformMatrix by multiplying the worldMatrix with the viewMatrix with the projectionMatrix
//          iterate over each vertex in the mesh
//              calculated the projected point (x,y,z coords) using the vertex and the transformMatrix
//              draw the point to the screen
pub fn go(canvas_ctx: &web_sys::CanvasRenderingContext2d, dims: Vec2, camera: &Camera, meshes: &mut Vec<Mesh>) {
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

    // calculate view_matrix from camera
    let view_matrix = Mat4::LookAtLH(camera.origin, camera.target, camera.up);

    // calculate a projection_matrix from width/height and magic
    let width = dims.x();
    let height = dims.y();
    let projection_matrix = Mat4::PerspectiveFovLH(0.78, width / height, 0.01, 1.0);

    for mesh in meshes.iter_mut() {
        mesh.rotation.coord[0] += 0.01;
        mesh.rotation.coord[1] += 0.01;
        //mesh.rotation.coord[2] += 0.01;

        // calculate the world_matrix by multiplying the rotation of the mesh with its position
        let world_matrix = Mat4::RotationYawPitchRoll(
                            mesh.rotation.y(),
                            mesh.rotation.x(),
                            mesh.rotation.z()) *
                                Mat4::translation(
                                    mesh.origin.x(),
                                    mesh.origin.y(),
                                    mesh.origin.z());

        let transform_matrix = world_matrix * view_matrix.clone() * projection_matrix.clone(); // VERIFIED

let mut i = 0;
        for vertex in &mesh.vertices {
log!("loop iteration: {}", i);
            let projected_coord = project(&dims, vertex, &transform_matrix);
            //draw_point(&colors[i], canvas_ctx, &projected_coord);
            draw_point(&colors[3], canvas_ctx, &projected_coord);
i = (i + 1) % colors.len();
        }

        for (a, b) in mesh.lines
            .iter()
//            .map(|(a,b)| (&mesh.vertices[*a], &mesh.vertices[*b]))
        {
log!("line: ({}, {})", a, b);
let a = &mesh.vertices[*a];
let b = &mesh.vertices[*b];
// The idea is to take the dot so the scale (by default 100.0) never surpasses the OTHER vertex
            let projected_coord_a = project(&dims, a, &transform_matrix);
            let projected_coord_b = project(&dims, b, &transform_matrix);
            let diff = projected_coord_b - projected_coord_a;
            let unit = diff.normal();

            // kinda works (og solution)
            let a_max = projected_coord_a + unit.scale(100.0);
            let b_max = projected_coord_b - unit.scale(100.0);
            /*
            // in progress
            let a_scale = unit.scale(100.0).dot(&projected_coord_b);
            let a_max = projected_coord_a + unit.scale(a_scale);
            let b_scale = unit.scale(100.0).dot(&projected_coord_a);
            let b_max = projected_coord_b - unit.scale(b_scale);
log!("scale_a: {:?}, scale_b: {:?}", a_scale, b_scale);
            */

log!("a: {:?}, b: {:?}, unit: {:?}, max: {:?}", projected_coord_a, projected_coord_b, unit, a_max);
log!("diff: {:?}, |scale|: {}, |a_max|: {}", diff, (projected_coord_b - a_max).magnitude(), a_max.magnitude());
            draw_line(&colors[0], canvas_ctx, &projected_coord_a, &a_max);
//            draw_line(&colors[0], canvas_ctx, &projected_coord_b, &b_max);
        }
    }
}

pub fn project(dims: &Vec2, coord: &Vec3, trans: &Mat4) -> Vec2 {
    let width = dims.x();
    let height = dims.y();
    let point = Vec3::TransformCoordinates(coord, trans);
    // The transformed coordinates will be based on coordinate system
    // starting on the center of the screen. But drawing on screen normally starts
    // from top left. We then need to transform them again to have x:0, y:0 on top left.
    let x = point.x() * width + width / 2.0;
    let y = -point.y() * height + height / 2.0;
    return Vec2::new([x, y]);
}

pub fn draw_point(color: &JsValue, canvas_ctx: &web_sys::CanvasRenderingContext2d, coord: &Vec2) {
//    canvas_ctx.save();
    {
        canvas_ctx.begin_path();
        canvas_ctx.arc(coord.x(), coord.y(), 100.0, 0.0, 2.0 * 3.14159);
        canvas_ctx.set_fill_style(color);
        canvas_ctx.fill();
        canvas_ctx.set_stroke_style(&JsValue::from_str(&format!("#{:0>6x}", 0x000000)));
        canvas_ctx.stroke();
    }
//    canvas_ctx.restore();

/*
        // drawPoint calls putPixel but does the clipping operation before
        Device.prototype.drawPoint = function (point) {
            // Clipping what's visible on screen
            if (point.x >= 0 && point.y >= 0 && point.x < this.workingWidth 
                                             && point.y < this.workingHeight) {
                // Drawing a yellow point
                this.putPixel(point.x, point.y, new BABYLON.Color4(1, 1, 0, 1));
            }
        };

        */
}

pub fn draw_line(color: &JsValue, canvas_ctx: &web_sys::CanvasRenderingContext2d, coord_a: &Vec2, coord_b: &Vec2) {
        canvas_ctx.begin_path();
        canvas_ctx.move_to(coord_a.x(), coord_a.y());
        canvas_ctx.line_to(coord_b.x(), coord_b.y());
        canvas_ctx.close_path();
        canvas_ctx.set_stroke_style(color);
        canvas_ctx.stroke();
}
