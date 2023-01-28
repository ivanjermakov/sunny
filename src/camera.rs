use std::f32::consts::PI;

use crate::ray::Ray;
use crate::shape::plane::Plane;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Camera {
    pub resolution: Vec3,
    pub viewport: Plane,
    // TODO: FOV
}

impl Camera {
    /// Create a ray coming from the center of a specified resolution pixel at viewport direction
    pub fn camera_ray(&self, px: Vec3) -> Ray {
        let vp_tr = (((px + Vec3::new(0.5, 0.5, 0.)) * self.viewport.size) / self.resolution)
            .with_z(0.)
            .rotate_z(-PI / 2.)
            .rotate_y(PI / 2.);
        let top = (Vec3::diag(self.viewport.size.y / 2.) * self.viewport.dir).rotate_y(-PI / 2.);
        let left = (Vec3::diag(self.viewport.size.x / 2.) * self.viewport.dir).rotate_z(PI / 2.);
        let top_left = top + left;
        Ray {
            start: self.viewport.center + (top_left + vp_tr),
            dir: self.viewport.dir,
        }
    }
}
