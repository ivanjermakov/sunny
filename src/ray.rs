use std::f32::consts::PI;

use rand::random;

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

    pub fn diffuse(&self, n: usize) -> Vec<Ray> {
        (0..n)
            .map(|_| {
                let th = random::<f32>() * PI;
                let fi = random::<f32>() * (PI / 2.);
                self.dir.rotate_x(th).rotate_y(fi)
            })
            .map(|dir| Ray {
                start: self.start,
                dir,
            })
            .collect()
    }
}
