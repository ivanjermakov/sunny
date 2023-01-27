use crate::ray::Ray;
use crate::shape::Shape;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Plane {
    pub center: Vec3,
    pub size: Vec3,
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> bool {
        todo!()
    }
}
