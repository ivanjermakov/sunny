use std::f32::consts::PI;

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
        let mut ps = vec![];
        for y in 0..h {
            for x in 0..w {
                let cr = self.camera.camera_ray(Vec3::new(x as f32, y as f32, 0.));
                let bounces = self
                    .ray_trace(&cr, vec![])
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>();
                let color = if let Some((o, r)) = bounces.first() {
                    if o.material.luminosity > 0. {
                        let prev_r = bounces.get(1).map(|(_, r)| r).unwrap_or(&cr);
                        let b = r.dir.angle(&prev_r.dir) / PI;
                        o.material.color.brightness(b)
                    } else {
                        Color::BLACK
                    }
                } else {
                    Color::BLACK
                };

                ps.push(color);
            }
        }
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
        if let Some((o, r)) = self.reflect(ray) {
            rays.push((o, r));
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
