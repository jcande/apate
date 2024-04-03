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

    let mut points = Vec::<Vec2>::new();
    let mut lines = Vec::<(Vec2, Vec2)>::new();
    for mesh in meshes.iter_mut() {
        mesh.rotation.coord[0] += 0.005;
        mesh.rotation.coord[1] += 0.005;
        mesh.rotation.coord[2] += 0.0001;

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

        for vertex in &mesh.vertices {
            let projected_coord = project(&dims, vertex, &transform_matrix);
            points.push(projected_coord);
        }

        for (a, b) in mesh.lines
            .iter()
            .map(|(a,b)| (&mesh.vertices[*a], &mesh.vertices[*b]))
        {
            let projected_coord_a = project(&dims, a, &transform_matrix);
            let projected_coord_b = project(&dims, b, &transform_matrix);
            lines.push((projected_coord_a, projected_coord_b));
        }
    }

    canvas_ctx.save();

    // Make vertex bubble mask
    canvas_ctx.begin_path();
    canvas_ctx.set_fill_style(&colors[3]);
    for point in points {
        draw_point(canvas_ctx, &point);
    }
    canvas_ctx.fill();
    canvas_ctx.close_path();
    canvas_ctx.clip();

    for (a, b) in lines {
        draw_line(&colors[0], canvas_ctx, &a, &b);
    }

    canvas_ctx.restore();
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

pub fn draw_point(canvas_ctx: &web_sys::CanvasRenderingContext2d, coord: &Vec2) {
    canvas_ctx.move_to(coord.x(), coord.y());
    canvas_ctx.arc(coord.x(), coord.y(), 100.0, 0.0, 2.0 * 3.14159);
}

pub fn draw_line(color: &JsValue, canvas_ctx: &web_sys::CanvasRenderingContext2d, coord_a: &Vec2, coord_b: &Vec2) {
    canvas_ctx.begin_path();
    canvas_ctx.move_to(coord_a.x(), coord_a.y());
    canvas_ctx.line_to(coord_b.x(), coord_b.y());
    canvas_ctx.close_path();
    canvas_ctx.set_stroke_style(color);
    canvas_ctx.stroke();
}
