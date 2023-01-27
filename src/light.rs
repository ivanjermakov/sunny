use crate::shape::Shape;

#[derive(Debug)]
pub struct Light {
    pub shape: Box<dyn Shape>,
    pub intensity: f32,
}
