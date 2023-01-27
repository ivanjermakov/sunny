use crate::camera::Camera;
use crate::light::Light;
use crate::object::Object;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}
