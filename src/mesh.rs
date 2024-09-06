use crate::point::Vec3;

pub type Line = (usize, usize);

pub struct Mesh {
    pub origin: Vec3,
    pub rotation: Vec3,
    pub vertices: Vec<Vec3>,
    pub lines: Vec<Line>,
}

impl Mesh {
    pub fn new(origin: Vec3, vertices: Vec<Vec3>, lines: Vec<Line>) -> Self {
        // TODO ensure every offset in lines fits within |vertices|
        Mesh {
            origin,
            //rotation: Vec3::zeroes(),
            rotation: Vec3::new([3.14159 / 4.0, 3.14159 / 4.0, 0.0]),
            vertices,
            lines,
        }
    }

    pub fn mk_cube() -> Mesh {
        Self::new(
            Vec3::zeroes(),
            vec![
                Vec3::new([-1.0, 1.0, 1.0]),
                Vec3::new([1.0, 1.0, 1.0]),
                Vec3::new([-1.0, -1.0, 1.0]),
                Vec3::new([-1.0, -1.0, -1.0]),
                Vec3::new([-1.0, 1.0, -1.0]),
                Vec3::new([1.0, 1.0, -1.0]),
                Vec3::new([1.0, -1.0, 1.0]),
                Vec3::new([1.0, -1.0, -1.0]),
            ],

            vec![
                // inner cube
                (0, 1),
                (0, 2),
                (0, 4),

                (1, 0),
                (1, 5),
                (1, 6),

                (2, 0),
                (2, 3),
                (2, 6),

                (3, 2),
                (3, 4),
                (3, 7),

                (4, 0),
                (4, 3),
                (4, 5),

                (5, 1),
                (5, 4),
                (5, 7),

                (6, 1),
                (6, 2),
                (6, 7),

                (7, 3),
                (7, 5),
                (7, 6),

            ],
        )
    }

    pub fn mk_tetra_cube() -> Mesh {
        Self::new(
            Vec3::zeroes(),
            vec![
                Vec3::new([-1.0, 1.0, 1.0]),
                Vec3::new([1.0, 1.0, 1.0]),
                Vec3::new([1.0, -1.0, 1.0]),
                Vec3::new([-1.0, -1.0, 1.0]),
                Vec3::new([0.0, 1.0 + 2.0_f64.sqrt(), 0.0]),
                Vec3::new([0.0, -1.0 - 2.0_f64.sqrt(), 0.0]),
                Vec3::new([1.0 + 2.0_f64.sqrt(), 0.0, 0.0]),
                Vec3::new([-1.0 - 2.0_f64.sqrt(), 0.0, 0.0]),
                Vec3::new([0.0, 1.0, -1.0 - 2.0_f64.sqrt()]),
                Vec3::new([0.0, -1.0, -1.0 + 2.0_f64.sqrt()]),
                Vec3::new([1.0, 0.0, -1.0 - 2.0_f64.sqrt()]),
                Vec3::new([-1.0, 0.0, -1.0 + 2.0_f64.sqrt()]),
                Vec3::new([0.0, 0.0, 2.0]),
            ],

            vec![
                // Square face 1
                (0, 1), (1, 2), (2, 3), (3, 0),
                // Square face 2
                (4, 5), (6, 7), (8, 9), (10, 11),
                // Connecting lines between faces
                (0, 4), (1, 6), (2, 8), (3, 10),
                // Inner square
                (4, 6), (6, 8), (8, 10), (10, 4),
                /*
                // Lines from vertices to center
                (0, 12), (1, 12), (2, 12), (3, 12),
                // Lines from vertices to center
                (4, 12), (5, 12), (6, 12), (7, 12),
                // Lines from vertices to center
                (8, 12), (9, 12), (10, 12), (11, 12),
                */
            ],
        )
    }

    pub fn mk_cube_dbg() -> Mesh {
        Self::new(
            Vec3::zeroes(),
            vec![

                Vec3::new([-1.0, 1.0, 1.0]),
                Vec3::new([-1.0, -1.0, 1.0]),
            ],

            vec![
                (0, 1),
                (1, 0),
            ],
        )
    }

}
