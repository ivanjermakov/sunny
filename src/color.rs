use std::cmp::{max, min};
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const BLACK: RgbColor = RgbColor::mono(0);
    pub const WHITE: RgbColor = RgbColor::mono(255);
    pub const RED: RgbColor = RgbColor::rgb(255, 0, 0);
    pub const GREEN: RgbColor = RgbColor::rgb(0, 255, 0);
    pub const BLUE: RgbColor = RgbColor::rgb(0, 0, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> RgbColor {
        RgbColor { r, g, b }
    }

    pub const fn mono(k: u8) -> RgbColor {
        RgbColor { r: k, g: k, b: k }
    }

    pub fn brighten(&self, b: f32) -> RgbColor {
        RgbColor {
            r: (self.r as f32 * b).clamp(0., 255.) as u8,
            g: (self.g as f32 * b).clamp(0., 255.) as u8,
            b: (self.b as f32 * b).clamp(0., 255.) as u8,
        }
    }

    pub fn min(&self) -> u8 {
        min(min(self.r, self.g), self.b)
    }

    pub fn max(&self) -> u8 {
        max(max(self.r, self.g), self.b)
    }

    pub fn chroma(&self) -> u8 {
        self.max() - self.min()
    }

    pub fn lightness(&self) -> u8 {
        (self.r + self.g + self.b) / 3
    }

    pub fn hue(&self) -> u8 {
        if self.chroma() == 0 {
            return 0;
        }
        let max = self.max();
        let c = self.chroma();
        if self.r == max {
            60 * (2 + (self.g - self.b) / c)
        } else if self.g == max {
            60 * (2 + (self.b - self.r) / c)
        } else {
            60 * (2 + (self.r - self.g) / c)
        }
    }

    pub fn saturation(&self) -> u8 {
        let l = self.lightness();
        if l == 0 || l == 1 {
            0
        } else {
            (self.max() - l) / min(l, 1 - l)
        }
    }
}

impl Add for RgbColor {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        RgbColor {
            r: (self.r + rhs.r).clamp(0, 255),
            g: (self.g + rhs.g).clamp(0, 255),
            b: (self.b + rhs.b).clamp(0, 255),
        }
    }
}

pub struct HslColor {
    pub h: u8,
    pub s: u8,
    pub l: u8,
}

impl From<RgbColor> for HslColor {
    fn from(value: RgbColor) -> Self {
        HslColor {
            h: value.hue(),
            s: value.saturation(),
            l: value.lightness(),
        }
    }
}
