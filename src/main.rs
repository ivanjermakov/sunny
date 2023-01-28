mod camera;
mod light;
mod object;
pub mod ray;
mod scene;
mod shape;
mod vec3;
mod math;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::camera::Camera;
    use crate::light::Light;
    use crate::object::Object;
    use crate::scene::Scene;
    use crate::shape::plane::Plane;
    use crate::shape::sphere::Sphere;
    use crate::vec3::Vec3;

    #[test]
    fn test() {
        let sphere = Object {
            shape: Box::new(Sphere {
                center: Vec3::new(2., 0., 0.),
                radius: 1.,
            }),
        };
        let light = Light {
            shape: Box::new(Sphere {
                center: Vec3::new(0., 2., 2.),
                radius: 1.,
            }),
            intensity: 10.,
        };
        let scene = Scene {
            camera: Camera {
                resolution: Vec3::new(600., 400., 0.),
                viewport: Plane {
                    center: Vec3::new(0., 0., 0.),
                    size: Vec3::new(3., 2., 0.),
                    dir: Vec3::new(1., 0., 0.),
                },
            },
            objects: vec![sphere],
            lights: vec![light],
        };
        println!("{scene:#?}")
    }
}
