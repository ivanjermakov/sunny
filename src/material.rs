use std::fmt::Debug;

use crate::color::RgbColor;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Material {
    /// Determines how rays can be reflected from the surface
    /// When 0, acts as a perfect mirror
    /// When 1, rays reflected evenly in every possible direction
    pub diffusion: f32,
    pub color: RgbColor,
    /// Surface light emitting property
    pub luminosity: f32,
}
