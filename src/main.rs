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
    let asp_ratio = 16. / 9.;
    let width = 2560.;
    let vp_width = 6.;
    let scene = Scene {
        camera: Camera {
            resolution: Vec3::new(width, width / asp_ratio, 0.),
            viewport: Plane {
                center: Vec3::new(0., 0., 3.),
                size: Vec3::new(vp_width, vp_width / asp_ratio, 0.),
                dir: Vec3::new(1., 0., -0.2).norm(),
            },
        },
        objects: vec![
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(0., 0., 110.),
                    radius: 100.,
                }),
                material: Material {
                    diffusion: 0.,
                    reflection: 1.,
                    color: Color::WHITE,
                    luminosity: 10.,
                },
            },
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(0., 0., -110.),
                    radius: 110.,
                }),
                material: Material {
                    diffusion: 0.,
                    reflection: 1.,
                    color: Color::WHITE,
                    luminosity: 10.,
                },
            },
            Object {
                shape: Box::new(Sphere {
                    center: Vec3::new(8., 0., 1.),
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
                    center: Vec3::new(6., 1., 0.5),
                    radius: 0.5,
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
                    center: Vec3::new(6., -1., 0.8),
                    radius: 0.8,
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

    let image = scene.render();
    image.save_ppm("data/scene.ppm").ok();
}

#[cfg(test)]
mod test {}
