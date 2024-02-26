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
            rotation: Vec3::zeroes(),
            vertices,
            lines,
        }
    }

    pub fn mk_cube_og() -> Mesh {
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

            /*
                Vec3::new([-1.0, 1.0, 1.0]),
                Vec3::new([1.0, 1.0, 1.0]),
                Vec3::new([-1.0, -1.0, 1.0]),
                Vec3::new([-1.0, -1.0, -1.0]),
                Vec3::new([-1.0, 1.0, -1.0]),
                Vec3::new([1.0, 1.0, -1.0]),
                Vec3::new([1.0, -1.0, 1.0]),
                Vec3::new([1.0, -1.0, -1.0]),
            */

            /*
                Vec3::new([1.0,1.0,1.0]),
                Vec3::new([1.0,-1.0,1.0]),
                Vec3::new([-1.0,1.0,1.0]),
                Vec3::new([-1.0,-1.0,1.0]),
                // XXX negative z makes me sad
                Vec3::new([1.0,1.0,-1.0]),
                Vec3::new([1.0,-1.0,-1.0]),
                Vec3::new([-1.0,1.0,-1.0]),
                Vec3::new([-1.0,-1.0,-1.0]),
            */

                /*
                Vec3::new([-100.0, 100.0, 100.0]),
                Vec3::new([100.0, 100.0, 100.0]),
                Vec3::new([-100.0, -100.0, 100.0]),
                Vec3::new([-100.0, -100.0, -100.0]),
                Vec3::new([-100.0, 100.0, -100.0]),
                Vec3::new([100.0, 100.0, -100.0]),
                Vec3::new([100.0, -100.0, 100.0]),
                Vec3::new([100.0, -100.0, -100.0]),
                */

            ],

            vec![
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

    pub fn mk_cube() -> Mesh {
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
