use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Ray {
    pub start: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn with_param(&self, t: f32) -> Vec3 {
        self.start + self.dir * Vec3::diag(t)
    }

    pub fn offset(&self, off: f32) -> Ray {
        Ray {
            start: self.start + self.dir * Vec3::diag(off),
            dir: self.dir,
        }
    }
}
