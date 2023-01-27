use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Ray {
    pub start: Vec3,
    pub dir: Vec3,
}
