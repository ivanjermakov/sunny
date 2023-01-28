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

fn main() {
    let light = Object {
        shape: Box::new(Sphere {
            center: Vec3::new(-2., 4., 4.),
            radius: 1.,
        }),
        material: Material {
            diffusion: 0.,
            reflection: 1.,
            color: Color::WHITE,
            luminosity: 10.,
        },
    };
    let scene = Scene {
        camera: Camera {
            resolution: Vec3::new(6. * 100., 4. * 100., 0.),
            viewport: Plane {
                center: Vec3::new(0., 0., 0.),
                size: Vec3::new(6., 4., 0.),
                dir: Vec3::new(1., 0., 0.),
            },
        },
        objects: vec![
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(2., 0., 0.),
                    radius: 1.,
                }),
                material: Material {
                    diffusion: 0.,
                    reflection: 1.,
                    color: Color::WHITE,
                    luminosity: 0.,
                },
            },
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(1., 0.5, -0.5),
                    radius: 0.5,
                }),
                material: Material {
                    diffusion: 0.,
                    reflection: 1.,
                    color: Color::WHITE,
                    luminosity: 0.,
                },
            },
        ],
    };

    let i = scene.render();
    i.save_ppm("data/scene.ppm").ok();
}

#[cfg(test)]
mod test {}
