extern crate core;

use crate::camera::Camera;
use crate::color::Color;
use crate::material::Material;
use crate::object::Object;
use crate::scene::Scene;
use crate::shape::plane::Plane;
use crate::shape::sphere::Sphere;
use crate::vec3::Vec3;
use std::f32::consts::PI;

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
const WIDTH: f32 = 1080. * 1.0;
const VP_WIDTH: f32 = 1.;

const PIXEL_PASS_COUNT: usize = 50;
const REFLECTION_DEPTH: usize = 6;
const AMBIENT_COLOR: Color = Color::rgb(0.1, 0.3, 0.5);

fn main() {
    let camera = Camera {
        resolution: Vec3::new(WIDTH, WIDTH / ASP_RATIO, 0.),
        viewport: Plane {
            center: Vec3::new(-4., 0., 4.),
            size: Vec3::new(VP_WIDTH, VP_WIDTH / ASP_RATIO, 0.),
            dir: Vec3::new(1., 0., -0.25).norm(),
        },
        focal_len: 1.4,
    };
    let diag = camera.viewport.size.x.hypot(camera.viewport.size.y);
    let fov = 2. * (diag / (2. * camera.focal_len)).atan();
    dbg!(fov * (180. / PI));
    let floor = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(0., 0., -1000.),
            radius: 1000.,
        }),
        material: Material {
            roughness: 0.8,
            color: Color::WHITE,
            luminosity: 0.,
        },
    };
    let back_wall = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(12., 0., 2.),
            radius: 4.,
        }),
        material: Material {
            roughness: 0.0,
            color: Color::WHITE,
            luminosity: 0.,
        },
    };
    let scene = Scene {
        camera,
        objects: vec![
            floor,
            back_wall,
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(0., -40., 110.),
                    radius: 100.,
                }),
                material: Material {
                    roughness: 0.,
                    color: Color::WHITE,
                    luminosity: 1.0,
                },
            },
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(6., 2., 2.),
                    radius: 0.5,
                }),
                material: Material {
                    roughness: 0.,
                    color: Color::RED,
                    luminosity: 5.,
                },
            },
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(4., 0., 1.),
                    radius: 1.,
                }),
                material: Material {
                    roughness: 0.,
                    color: Color::BLUE,
                    luminosity: 0.,
                },
            },
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(2., 1.5, 0.5),
                    radius: 0.5,
                }),
                material: Material {
                    roughness: 0.5,
                    color: Color::GREEN,
                    luminosity: 0.,
                },
            },
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(2., -1.5, 0.8),
                    radius: 0.8,
                }),
                material: Material {
                    roughness: 1.,
                    color: Color::RED,
                    luminosity: 0.,
                },
            },
        ],
    };

    let image = scene.render();
    image.save_ppm("data/scene.ppm").ok();
}

#[cfg(test)]
mod test {}
