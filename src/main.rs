extern crate core;

use crate::camera::Camera;
use crate::color::Color;
use crate::material::Material;
use crate::object::Object;
use crate::scene::Scene;
use crate::shape::plane::Plane;
use crate::shape::sphere::Sphere;
use crate::vec3::Vec3;

pub mod camera;
pub mod color;
pub mod image;
pub mod material;
pub mod math;
pub mod object;
pub mod ray;
pub mod scene;
pub mod shape;
pub mod vec3;

const ASP_RATIO: f32 = 16. / 16.;
const WIDTH: f32 = 1080. * 2.;
const VP_WIDTH: f32 = 1.;

const PIXEL_PASS_COUNT: usize = 50;
const REFLECTION_DEPTH: usize = 3;
// const AMBIENT_COLOR: Color = Color::rgb(0.1, 0.1, 0.4);
const AMBIENT_COLOR: Color = Color::mono(0.2);

fn main() {
    let c_pos = Vec3::new(-3., -1., 1.);
    let c_dir = (Vec3::new(0., -1., 1.) - c_pos).norm();
    let camera = Camera {
        resolution: Vec3::new(WIDTH, WIDTH / ASP_RATIO, 0.),
        viewport: Plane {
            center: c_pos,
            size: Vec3::new(VP_WIDTH, VP_WIDTH / ASP_RATIO, 0.),
            dir: c_dir,
        },
        focal_len: 1.,
    };
    let mut objects = vec![];
    let floor = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(1000., 0., 0.),
            radius: 1000.,
        }),
        material: Material {
            roughness: 1.,
            specularity: 1.,
            color: Color::YELLOW,
            luminosity: 0.,
        },
    };
    objects.push(floor);
    let light = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(-100., -100., 100.),
            radius: 70.,
        }),
        material: Material {
            roughness: 0.,
            specularity: 0.,
            color: Color::WHITE,
            luminosity: 10.0,
        },
    };
    objects.push(light);
    let light_s = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(-100., 0., 0.),
            radius: 20.,
        }),
        material: Material {
            roughness: 0.,
            specularity: 0.,
            color: Color::WHITE,
            luminosity: 0.5,
        },
    };
    objects.push(light_s);
    let mut balls = vec![];
    for i in 0..=2 {
        let spec = i as f32 / 2.;
        for j in 0..=2 {
            let rough = j as f32 / 2.;
            let o = Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(-0.4, -j as f32, i as f32),
                    radius: 0.4,
                }),
                material: Material {
                    roughness: rough,
                    specularity: spec,
                    color: Color::RED,
                    luminosity: 0.,
                },
            };
            balls.push(o)
        }
    }

    objects.extend(balls);

    let scene = Scene { camera, objects };
    let image = scene.render();
    image.save_ppm("data/scene.ppm").ok();
}

#[cfg(test)]
mod test {}
