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
const WIDTH: f32 = 1080. * 2.0;
const VP_WIDTH: f32 = 1.;

const PIXEL_PASS_COUNT: usize = 100;
const REFLECTION_DEPTH: usize = 6;
const AMBIENT_COLOR: Color = Color::rgb(0.1, 0.1, 0.4);

fn main() {
    let c_pos = Vec3::new(-1., -1., 1.).norm().mul_n(5.);
    let c_dir = (Vec3::new(0., 0., 0.) - c_pos).norm();
    let camera = Camera {
        resolution: Vec3::new(WIDTH, WIDTH / ASP_RATIO, 0.),
        viewport: Plane {
            center: c_pos,
            size: Vec3::new(VP_WIDTH, VP_WIDTH / ASP_RATIO, 0.),
            dir: c_dir,
        },
        focal_len: 1.5,
    };
    let mut objects = vec![];

    let floor = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(0., 0., -11.),
            radius: 10.,
        }),
        material: Material {
            roughness: 0.8,
            specularity: 0.6,
            color: Color::WHITE,
            luminosity: 0.,
        },
    };
    objects.push(floor);

    let super_floor = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(0., 0., -10010.),
            radius: 10000.,
        }),
        material: Material {
            roughness: 1.,
            specularity: 0.5,
            color: Color::mono(0.2),
            luminosity: 0.,
        },
    };
    objects.push(super_floor);

    let light = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(0., 0., 100.),
            radius: 70.,
        }),
        material: Material {
            roughness: 0.,
            specularity: 0.,
            color: Color::WHITE,
            luminosity: 5.0,
        },
    };
    objects.push(light);

    let light_a = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(0., -10., 0.),
            radius: 2.,
        }),
        material: Material {
            roughness: 0.,
            specularity: 0.,
            color: Color::RED,
            luminosity: 400.0,
        },
    };
    objects.push(light_a);

    let o = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(0., 0., 0.),
            radius: 1.,
        }),
        material: Material {
            roughness: 0.1,
            specularity: 1.,
            color: Color::GREEN,
            luminosity: 0.,
        },
    };
    objects.push(o);

    let scene = Scene { camera, objects };
    let image = scene.render();
    image.save_ppm("data/scene.ppm").ok();
}

#[cfg(test)]
mod test {}
