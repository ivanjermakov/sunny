use crate::shape::plane::Plane;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Camera {
    pub resolution: Vec3,
    pub viewport: Plane,
    // TODO: FOV
}
