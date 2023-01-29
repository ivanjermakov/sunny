use rayon::prelude::*;

use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::object::Object;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn render(&self) -> Image {
        let w = self.camera.resolution.x as i32;
        let h = self.camera.resolution.y as i32;
        let ps = (0..w * h)
            .into_par_iter()
            .map(|i| {
                let y = i / w as i32;
                let x = (y / w) as i32 + i % w;
                let cr = self.camera.camera_ray(Vec3::new(x as f32, y as f32, 0.));
                let bounces = self.ray_trace(&cr, vec![]);
                if let Some((o, r)) = bounces.first() {
                    if o.material.luminosity > 0. {
                        let prev_r = bounces.get(1).map(|(_, r)| r).unwrap_or(&cr);
                        let cos = r.dir.cos_angle(&prev_r.dir);
                        let b = (1. - (cos + 1.) / 2.).cbrt();
                        o.material.color.brightness(b)
                    } else {
                        Color::BLACK
                    }
                } else {
                    Color::BLACK
                }
            })
            .collect();
        Image {
            resolution: self.camera.resolution,
            pixels: ps,
        }
    }

    pub fn ray_trace<'a>(
        &'a self,
        ray: &Ray,
        mut rays: Vec<(&'a Object, Ray)>,
    ) -> Vec<(&Object, Ray)> {
        if rays.len() > 8 {
            return vec![];
        }
        if let Some((o, r)) = self.reflect(ray) {
            rays.insert(0, (o, r));
            if o.material.luminosity > 0. {
                // don't reflect from light emitting objects
                return rays;
            }
            return self.ray_trace(&r, rays);
        }
        rays
    }

    pub fn reflect(&self, ray: &Ray) -> Option<(&Object, Ray)> {
        let mut c_len = f32::MAX;
        let mut closest: Option<(&Object, Ray)> = None;
        for o in &self.objects {
            if let Some(reflection) = o.shape.reflect(ray) {
                let len = ray.start.dist(&reflection.start);
                if len < c_len {
                    c_len = len;
                    closest = Some((o, reflection))
                }
            }
        }
        closest
    }
}
