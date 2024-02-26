use crate::point::Point;
use crate::point::Vec3;
use crate::point::Mat3;
use crate::point::Mat4;

// XXX don't use this. Throw everything in point

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
    let zaxis = (cameraTarget - cameraPosition).normal();
    unimplemented!("do me");
}

// left-handed perspective projection matrix.
// https://learn.microsoft.com/en-us/previous-versions/windows/desktop/bb281727(v=vs.85)
pub fn PerspectiveFovLH(fieldOfViewY: Element, aspectRatio: Element, znearPlane: Element, zfarPlane: Element) -> Mat3 {
    unimplemented!("do me");
    /*
        h = cot(fieldOfViewY/2)
        h = w / aspectRatio => w = h * aspectRatio
        w       0       0                                             0
        0       h       0                                             0
        0       0       zfarPlane/(zfarPlane-znearPlane)              1
        0       0       -znearPlane*zfarPlane/(zfarPlane-znearPlane)  0
     */
}
