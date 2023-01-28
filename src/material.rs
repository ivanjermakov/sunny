use std::fmt::Debug;

use crate::color::Color;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Material {
    pub diffusion: f32,
    pub reflection: f32,
    pub color: Color,
    pub luminosity: f32,
}
