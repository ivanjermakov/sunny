use rayon::prelude::*;

use crate::camera::Camera;
use crate::color::RgbColor;
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
                let y = i / w;
                let x = (y / w) + (i % w);
                let cr = self.camera.camera_ray(Vec3::new(x as f32, y as f32, 0.));
                self.ray_trace(&cr, 0).unwrap_or(RgbColor::BLACK)
            })
            .collect();
        Image {
            resolution: self.camera.resolution,
            pixels: ps,
        }
    }

    pub fn ray_trace(&self, ray: &Ray, depth: usize) -> Option<RgbColor> {
        let max_depth = 2;
        let shadow_samples = 100;
        if depth > max_depth {
            return None;
        }
        if let Some((o, r)) = self.reflect(ray) {
            return if o.material.luminosity > 0. {
                Some(o.material.color)
            } else if o.material.diffusion > 0. {
                let color = self
                    .objects
                    .iter()
                    .filter(|o| o.material.luminosity > 0.)
                    .map(|l| {
                        let hits = (0..shadow_samples)
                            .filter_map(|_| {
                                let light_p = o.shape.random_point_inside();
                                let sh_normal = (r.start - light_p).norm();
                                let sh_ray = Ray {
                                    start: r.start,
                                    dir: sh_normal,
                                };
                                self.reflect(&sh_ray)
                                    .filter(|(o, _)| o.material.luminosity > 0.)
                                    .map(|_| ())
                            })
                            .count();
                        o.material.color.brighten(
                            (hits as f32 * l.material.luminosity / shadow_samples as f32).cbrt(),
                        )
                    })
                    .reduce(|a, b| a + b)
                    .unwrap_or(RgbColor::BLACK);
                Some(color)
            } else {
                self.ray_trace(&r, depth + 1)
            };
        } else {
            None
        }
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
