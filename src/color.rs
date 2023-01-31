use std::cmp::{max_by, min_by};
use std::ops::{Add, Div};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const BLACK: Color = Color::mono(0.);
    pub const WHITE: Color = Color::mono(1.);
    pub const RED: Color = Color::rgb(1., 0., 0.);
    pub const GREEN: Color = Color::rgb(0., 1., 0.);
    pub const BLUE: Color = Color::rgb(0., 0., 1.);
    pub const CYAN: Color = Color::rgb(0., 1., 1.);
    pub const MAGENTA: Color = Color::rgb(1., 0., 1.);
    pub const YELLOW: Color = Color::rgb(1., 1., 0.);

    pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub const fn mono(k: f32) -> Color {
        Color { r: k, g: k, b: k }
    }

    pub fn with_lightness(&self, l: f32) -> Color {
        if l == 0. {
            return Color::BLACK;
        }
        Color {
            r: (self.r * l),
            g: (self.g * l),
            b: (self.b * l),
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: (self.r).clamp(0., 1.),
            g: (self.g).clamp(0., 1.),
            b: (self.b).clamp(0., 1.),
        }
    }

    pub fn min(&self) -> f32 {
        min_by(
            min_by(self.r, self.g, |a, b| a.partial_cmp(b).unwrap()),
            self.b,
            |a, b| a.partial_cmp(b).unwrap(),
        )
    }

    pub fn max(&self) -> f32 {
        max_by(
            max_by(self.r, self.g, |a, b| a.partial_cmp(b).unwrap()),
            self.b,
            |a, b| a.partial_cmp(b).unwrap(),
        )
    }

    pub fn chroma(&self) -> f32 {
        self.max() - self.min()
    }

    pub fn lightness(&self) -> f32 {
        (self.r + self.g + self.b) / 3.
    }

    pub fn hue(&self) -> f32 {
        if self.chroma() == 0. {
            return 0.;
        }
        let max = self.max();
        let c = self.chroma();
        if self.r == max {
            60. * (2. + (self.g - self.b) / c)
        } else if self.g == max {
            60. * (2. + (self.b - self.r) / c)
        } else {
            60. * (2. + (self.r - self.g) / c)
        }
    }

    pub fn saturation(&self) -> f32 {
        let l = self.lightness();
        if l == 0. || l == 1. {
            0.
        } else {
            (self.max() - l) / min_by(l, 1. - l, |a, b| a.partial_cmp(b).unwrap())
        }
    }

    pub fn mul_n(self, n: f32) -> Color {
        Color {
            r: (self.r * n).div(2.),
            g: (self.g * n).div(2.),
            b: (self.b * n).div(2.),
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: (self.r + rhs.r).div(2.),
            g: (self.g + rhs.g).div(2.),
            b: (self.b + rhs.b).div(2.),
        }
    }
}

#[cfg(test)]
mod test {}
