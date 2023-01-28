use std::ops;

use crate::math::approx_eq;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0., 0., 0.)
    }

    pub fn diag(v: f32) -> Vec3 {
        Vec3::new(v, v, v)
    }

    pub fn abs(&self) -> Vec3 {
        Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn mag(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot_angle(&self, other: &Vec3, angle: f32) -> f32 {
        self.mag() * other.mag() * angle.cos()
    }

    pub fn angle(&self, other: &Vec3) -> f32 {
        f32::acos(self.dot(other) / (self.mag() * other.mag()))
    }

    pub fn norm(&self) -> Vec3 {
        let mag = self.mag();
        if mag > 0. {
            Vec3::new(self.x / mag, self.y / mag, self.z / mag)
        } else {
            *self
        }
    }

    pub fn dist(&self, other: &Vec3) -> f32 {
        (*self - *other).mag()
    }

    pub fn approx_eq(&self, other: &Vec3) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }

    /// [ref](https://math.stackexchange.com/a/13263/1028553)
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - Vec3::diag(2. * self.dot(n)) * *n
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}
