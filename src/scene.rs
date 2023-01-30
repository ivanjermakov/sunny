use rayon::prelude::*;

use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::object::Object;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::{AMBIENT_COLOR, PIXEL_PASS_COUNT, REFLECTION_DEPTH};

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
                (0..PIXEL_PASS_COUNT)
                    .filter_map(|_| {
                        let cr = self.camera.camera_ray(Vec3::new(x as f32, y as f32, 0.));
                        self.ray_trace(&cr, 0)
                    })
                    .enumerate()
                    .fold(Color::BLACK, |c1, (i, c2)| {
                        let i_f = i as f32;
                        let n_f = i_f + 1.;
                        Color::rgb(
                            (c1.r * i_f + c2.r) / n_f,
                            (c1.g * i_f + c2.g) / n_f,
                            (c1.b * i_f + c2.b) / n_f,
                        )
                    })
            })
            .collect();
        Image {
            resolution: self.camera.resolution,
            pixels: ps,
        }
    }

    pub fn ray_trace(&self, ray: &Ray, depth: usize) -> Option<Color> {
        if depth >= REFLECTION_DEPTH {
            return Some(Color::BLACK);
        }
        if let Some((Object { material: m, .. }, ref_n, ref_r)) = self.reflect(ray) {
            if m.luminosity > 0. {
                let angle = (-ray.dir).cos_angle(&ref_n) * 0.5 + 0.5;
                return Some(m.color.with_lightness(m.luminosity * angle));
            };
            let next = Ray {
                start: ref_r.start,
                dir: (ref_r.dir + Vec3::rand().mul_n(m.roughness)).norm(),
            };
            // TODO: fresnel reflection
            // TODO: optimize inside-reflected rays
            self.ray_trace(&next, depth + 1).map(|rc| {
                m.color
                    .mul(1. - m.specularity)
                    .with_lightness(rc.lightness())
                    + rc.mul(m.specularity)
            })
        } else {
            let angle = (ray.dir).cos_angle(&self.camera.viewport.dir) * 0.5 + 0.5;
            Some(AMBIENT_COLOR.with_lightness(1. - angle))
        }
    }

    pub fn reflect(&self, ray: &Ray) -> Option<(&Object, Vec3, Ray)> {
        let mut c_len = f32::MAX;
        let mut closest: Option<(&Object, Vec3, Ray)> = None;
        for o in &self.objects {
            if let Some((reflection, norm)) = o.shape.reflect(ray) {
                let len = ray.start.dist(&reflection.start);
                if len < c_len {
                    c_len = len;
                    closest = Some((o, norm, reflection));
                }
            }
        }
        closest
    }
}
