use std::fmt::Debug;

use crate::color::Color;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Material {
    /// Determines how rays can be reflected from the surface
    /// When 0, acts as a perfect mirror
    /// When 1, rays reflected evenly in every possible direction
    pub roughness: f32,

    /// Determines how much much light gets reflected from the surface
    /// When 0, all reflections are "consumed" and surface color is reflected
    /// When 1, light is reflected as-is, surface color is ignored
    pub specularity: f32,

    pub color: Color,
    /// Surface light emitting property
    pub luminosity: f32,
}
