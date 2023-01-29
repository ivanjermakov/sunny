use crate::material::Material;
use std::fmt::Debug;

use crate::shape::Shape;

#[derive(Debug)]
pub struct Object {
    pub shape: Box<dyn Shape>,
    pub material: Material,
}
