use std::f32::consts::PI;

use crate::ray::Ray;
use crate::shape::plane::Plane;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Camera {
    pub resolution: Vec3,
    pub viewport: Plane,
    pub focal_len: f32,
}

impl Camera {
    // TODO: incorrect center
    /// Create a ray coming from the center of a specified resolution pixel at viewport direction
    pub fn camera_ray(&self, px: Vec3) -> Ray {
        let vp_tr = (((px + Vec3::new(0.5, 0.5, 0.)) * self.viewport.size) / self.resolution)
            .with_z(0.)
            .rotate_z(-PI / 2.)
            .rotate_y(PI / 2.);
        let top = (Vec3::diag(self.viewport.size.y / 2.) * self.viewport.dir).rotate_y(-PI / 2.);
        let left = (Vec3::diag(self.viewport.size.x / 2.) * self.viewport.dir).rotate_z(PI / 2.);
        let top_left = top + left;
        let vp_p = top_left + vp_tr;
        let fp = self.viewport.dir.mul_n(-self.focal_len);
        let dir = (vp_p - fp).norm();
        Ray {
            start: self.viewport.center + fp,
            dir,
        }
    }

    /// Camera field of view in radians
    pub fn fov(&self) -> f32 {
        let diag = self.viewport.size.x.hypot(self.viewport.size.y);
        2. * (diag / (2. * self.focal_len)).atan()
    }
}
