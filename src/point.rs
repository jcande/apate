use std::ops::Sub;
use std::ops::Mul;
use std::ops::Add;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub type Element = f64;

pub type Vec2 = Point<2>;
pub type Vec3 = Point<3>;
pub type Vec4 = Point<4>;

pub type Mat3 = Matrix<3>;
pub type Mat4 = Matrix<4>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<const N: usize> {
    pub coord: [Element; N],
}
impl<const N: usize> Point<N> {
    pub fn new(coord: [Element; N]) -> Self {
        Point {
            coord,
        }
    }

    pub fn zeroes() -> Self {
        Point {
            coord: [0.0; N],
        }
    }

    pub fn magnitude(&self) -> Element {
        let sum_of_squares = self.coord.iter()
            .fold(0.0, |acc, x| {
                acc + x*x
            });

        sum_of_squares.sqrt()
    }

    pub fn scale(&self, scalar: Element) -> Self {
        let calculated = self.coord.iter()
            .map(|x| {
                x * scalar
            })
            .collect::<Vec<Element>>()
            .try_into()
            .expect("iterating over a fixed-size array should yield a fixed-size array");

        Self::new(calculated)
    }

    pub fn normal(&self) -> Self {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            return Self::zeroes();
        }

        self.scale(1.0 / magnitude)
    }

    pub fn dot(&self, other: &Self) -> Element {
        let calculated = self.coord.iter()
            .zip(other.coord.iter())
            .fold(0.0, |acc, (lhs, rhs)| {
                acc + (lhs * rhs)
            });

        calculated
    }

    pub fn cross(&self, other: &Self) -> Self {
        let max = N;
        let calculated = (0..max)
            .map(|i| {
                (self.coord[(i + 1) % max] * other.coord[(i + 2) % max]) +
                (-self.coord[(i + 2) % max] * other.coord[(i + 1) % max])
            })
            .collect::<Vec<Element>>()
            .try_into()
            .expect("iterating over a fixed-size array should yield a fixed-size array");

        Self::new(calculated)
    }
}

impl Vec2 {
    pub fn x(&self) -> Element {
        self.coord[0]
    }

    pub fn y(&self) -> Element {
        self.coord[1]
    }
}

impl Vec3 {
    pub fn x(&self) -> Element {
        self.coord[0]
    }

    pub fn y(&self) -> Element {
        self.coord[1]
    }

    pub fn z(&self) -> Element {
        self.coord[2]
    }

    pub fn TransformCoordinates(vector: &Vec3, transformation: &Mat4) -> Vec3 {
        let x = vector.x() * transformation.raw[0][0]
              + vector.y() * transformation.raw[1][0]
              + vector.z() * transformation.raw[2][0]
              +              transformation.raw[3][0];
        let y = vector.x() * transformation.raw[0][1]
              + vector.y() * transformation.raw[1][1]
              + vector.z() * transformation.raw[2][1]
              +              transformation.raw[3][1];
        let z = vector.x() * transformation.raw[0][2]
              + vector.y() * transformation.raw[1][2]
              + vector.z() * transformation.raw[2][2]
              +              transformation.raw[3][2];
        let w = vector.x() * transformation.raw[0][3]
              + vector.y() * transformation.raw[1][3]
              + vector.z() * transformation.raw[2][3]
              +              transformation.raw[3][3];

        Vec3::new([x/w, y/w, z/w])
    }
}

impl Vec4 {
    pub fn x(&self) -> Element {
        self.coord[0]
    }

    pub fn y(&self) -> Element {
        self.coord[1]
    }

    pub fn z(&self) -> Element {
        self.coord[2]
    }

    pub fn w(&self) -> Element {
        self.coord[3]
    }

}

impl<const N: usize> Add<Point<N>> for Point<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let calculated = self.coord.iter()
            .zip(other.coord.iter())
            .map(|(lhs, rhs)| {
                lhs + rhs
            })
            .collect::<Vec<Element>>()
            .try_into()
            .expect("iterating over a fixed-size array should yield a fixed-size array");

        Self::new(calculated)
    }
}

impl<const N: usize> Sub<Point<N>> for Point<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let calculated = self.coord.iter()
            .zip(other.coord.iter())
            .map(|(lhs, rhs)| {
                lhs - rhs
            })
            .collect::<Vec<Element>>()
            .try_into()
            .expect("iterating over a fixed-size array should yield a fixed-size array");

        Self::new(calculated)
    }
}

type Row<const N: usize> = [Element; N];

// We only support square matrices
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const N: usize> {
    /*
    [ (0, 0), (0, 1), ..., (0, N-1) ]
    [ (1, 0), (1, 1), ..., (1, N-1) ]
    ...
    [ (N-1, 0), (N-1, 1), ..., (N-1, N-1) ]
    */

    raw: [Row::<N>; N],
}

impl<const N: usize> Matrix<N> {
    pub fn new(data: [Row::<N>; N]) -> Self {
        Self {
            raw: data,
        }
    }

    pub fn zeroes() -> Self {
        Self {
            raw: [ [0.0; N]; N ],
        }
    }

    pub fn identity() -> Self {
        let mut z = Self::zeroes();

        for i in 0..N {
            z.raw[i][i] = 1.0;
        }

        z
    }
}

impl Mat4 {
    // https://learn.microsoft.com/en-us/previous-versions/windows/desktop/bb153147(v=vs.85)
    // Matrix is 4x4
    pub fn LookAtLH(cameraPosition: Vec3, cameraTarget: Vec3, cameraUpVector: Vec3) -> Mat4 {
        /*
            zaxis = normal(cameraTarget - cameraPosition)   // Vec3
            xaxis = normal(cross(cameraUpVector, zaxis))    // Vec3
            yaxis = cross(zaxis, xaxis)                     // Vec3

            {
            xaxis.x           yaxis.x           zaxis.x          0
            xaxis.y           yaxis.y           zaxis.y          0
            xaxis.z           yaxis.z           zaxis.z          0
            -dot(xaxis, cameraPosition)  -dot(yaxis, cameraPosition)  -dot(zaxis, cameraPosition)  1
            }
        */
        let zaxis = (cameraTarget.clone() - cameraPosition.clone()).normal();
        let xaxis = cameraUpVector.cross(&zaxis).normal();
        let yaxis = zaxis.cross(&xaxis);

        Mat4::new([
            [ xaxis.coord[0], yaxis.coord[0], zaxis.coord[0], 0.0 ],
            [ xaxis.coord[1], yaxis.coord[1], zaxis.coord[1], 0.0 ],
            [ xaxis.coord[2], yaxis.coord[2], zaxis.coord[2], 0.0 ],
            [ -xaxis.dot(&cameraPosition), -yaxis.dot(&cameraPosition), -zaxis.dot(&cameraPosition), 1.0 ],
        ])
    }

    // left-handed perspective projection matrix.
    // https://learn.microsoft.com/en-us/previous-versions/windows/desktop/bb281727(v=vs.85)
    pub fn PerspectiveFovLH(fieldOfViewY: Element, aspectRatio: Element, znearPlane: Element, zfarPlane: Element) -> Mat4 {
        if false {
        /*
            h = cot(fieldOfViewY/2)
            h = w / aspectRatio => w = h * aspectRatio
            w       0       0                                             0
            0       h       0                                             0
            0       0       zfarPlane/(zfarPlane-znearPlane)              1
            0       0       -znearPlane*zfarPlane/(zfarPlane-znearPlane)  0
         */
        let (s, c) = (fieldOfViewY / 2.0).sin_cos();
        let h = s / c; // cot(fieldOfViewY / 2)
        let w = h * aspectRatio;
        Mat4::new([
            [ w, 0.0, 0.0, 0.0 ],
            [ 0.0, h, 0.0, 0.0 ],
            [ 0.0, 0.0, zfarPlane / (zfarPlane - znearPlane), 1.0 ],
            [ 0.0, 0.0, -znearPlane * zfarPlane / (zfarPlane - znearPlane), 0.0 ],
        ])
        } else {
            // XXX this works but the above doesn't...
            let tan = 1.0 / (fieldOfViewY / 2.0).tan();
            Mat4::new([
                [tan / aspectRatio, 0.0, 0.0, 0.0],
                [0.0, tan, 0.0, 0.0],
                [0.0, 0.0, -zfarPlane / (znearPlane - zfarPlane), 1.0],
                [0.0, 0.0, (znearPlane * zfarPlane) / (znearPlane - zfarPlane), 0.0],
            ])
        }
    }

    // stolen from
    // https://github.com/BabylonJS/Babylon.js/blob/7bb743f955796a84c2cb179c44554aec8e164832/packages/dev/core/src/Maths/math.vector.ts#L4847
    // maybe use https://www.redcrab-software.com/en/Calculator/3x3/Matrix/Rotation-Matrix
    // XXX this function is UNVERIFIED and very likely has bugs in it. Need to compare it with a
    // "known good" function (like the output from that site) to feel better about it.
    pub fn RotationYawPitchRoll(yaw: Element, pitch: Element, roll: Element) -> Mat4 {
        let (sin_roll, cos_roll) = roll.sin_cos();
        let rotation_x = Mat4::new([
                [1.0,   0.0,        0.0,        0.0],
                [0.0,   cos_roll,   sin_roll,   0.0],
                [0.0,  -sin_roll,   cos_roll,   0.0],
                [0.0,   0.0,        0.0,        1.0],
        ]);
        let (sin_pitch, cos_pitch) = pitch.sin_cos();
        let rotation_y = Mat4::new([
                // 0 1 2 3
                [cos_pitch, 0.0,    -sin_pitch, 0.0],
                // 4 5 6 7
                [0.0,       1.0,    0.0,        0.0],
                // 8 9 10 11
                [sin_pitch, 0.0,    cos_pitch,  0.0],
                // 12 13 14 15
                [0.0,       0.0,    0.0,        1.0],
        ]);
        let (sin_yaw, cos_yaw) = yaw.sin_cos();
        let rotation_z = Mat4::new([
                // 0 1 2 3
                [cos_yaw,   sin_yaw,    0.0,    0.0],
                // 4 5 6 7
                [-sin_yaw,  cos_yaw,    0.0,    0.0],
                // 8 9 10 11
                [0.0,       0.0,        1.0,    0.0],
                // 12 13 14 15
                [0.0,       0.0,        0.0,    1.0],
        ]);

        rotation_z*rotation_x*rotation_y
/*
        Matrix.RotationYawPitchRoll = function RotationYawPitchRoll(yaw, pitch, roll) {
            return Matrix.RotationZ(roll).multiply(Matrix.RotationX(pitch)).multiply(Matrix.RotationY(yaw));
        };
        Matrix.Zero = function Zero() {
            return Matrix.FromValues(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        };
        Matrix.RotationX = function RotationX(angle) {
            var result = Matrix.Zero();
            var s = Math.sin(angle);
            var c = Math.cos(angle);
            result.m[0] = 1.0;
            result.m[15] = 1.0;
            result.m[5] = c;
            result.m[10] = c;
            result.m[9] = -s;
            result.m[6] = s;
            return result;
        };
        Matrix.RotationY = function RotationY(angle) {
            var result = Matrix.Zero();
            var s = Math.sin(angle);
            var c = Math.cos(angle);
            result.m[5] = 1.0;
            result.m[15] = 1.0;
            result.m[0] = c;
            result.m[2] = -s;
            result.m[8] = s;
            result.m[10] = c;
            return result;
        };
        Matrix.RotationZ = function RotationZ(angle) {
            var result = Matrix.Zero();
            var s = Math.sin(angle);
            var c = Math.cos(angle);
            result.m[10] = 1.0;
            result.m[15] = 1.0;
            result.m[0] = c;
            result.m[1] = s;
            result.m[4] = -s;
            result.m[5] = c;
            return result;
        };
*/
/*
        let halfRoll = roll * 0.5;
        let halfPitch = pitch * 0.5;
        let halfYaw = yaw * 0.5;

        let (sinRoll, cosRoll) = halfRoll.sin_cos();
        let (sinPitch, cosPitch) = halfPitch.sin_cos();
        let (sinYaw, cosYaw) = halfYaw.sin_cos();

        Vec4::new([
            cosYaw * sinPitch * cosRoll + sinYaw * cosPitch * sinRoll,
            sinYaw * cosPitch * cosRoll - cosYaw * sinPitch * sinRoll,
            cosYaw * cosPitch * sinRoll - sinYaw * sinPitch * cosRoll,
            cosYaw * cosPitch * cosRoll + sinYaw * sinPitch * sinRoll,
        ])
*/
    }

    pub fn translation(x: Element, y: Element, z: Element) -> Mat4 {
        Mat4::new([
              [1.0, 0.0, 0.0, 0.0],
              [0.0, 1.0, 0.0, 0.0],
              [0.0, 0.0, 1.0, 0.0],
              [x, y, z, 1.0],
        ])
    }
}

impl<const N: usize> Mul<Matrix<N>> for Matrix<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let calculated = (0..N)
            .map(|i| -> [Element; N] {
                let row = (0..N)
                    .map(|j| -> Element {
                        let inner = (0..N)
                            .fold(0.0, |acc, k| {
                                acc + (self.raw[i][k] * other.raw[k][j])
                            });

                        inner
                    })
                    .collect::<Vec<Element>>()
                    .try_into()
                    .expect("iterating over a fixed-size array should yield a fixed-sized array");

                row
            })
            .collect::<Vec<Row::<N>>>()
            .try_into()
            .expect("iterating over a fixed-size array should yield a fixed-size array");

        Self::new(calculated)
    }
}
