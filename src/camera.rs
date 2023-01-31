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
    /// Create a ray coming from the center of a specified resolution pixel at viewport direction
    pub fn camera_ray(&self, px: Vec3) -> Ray {
        let vp_tr = ((px + Vec3::new(0.5, 0.5, 0.)) * self.viewport.size) / self.resolution;
        let z = Vec3::new(0., 0., 1.).norm();
        let y = z.cross(&self.viewport.dir).norm();
        let top = self
            .viewport
            .dir
            .cross(&y)
            .mul_n(self.viewport.size.y / 2. - vp_tr.y);
        let left = y.mul_n(self.viewport.size.x / 2. - vp_tr.x);
        let vp_p = top + left;
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
