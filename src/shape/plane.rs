use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Plane {
    pub center: Vec3,
    pub size: Vec3,
    pub dir: Vec3,
}
