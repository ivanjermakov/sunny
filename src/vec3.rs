use std::f32::consts::PI;
use std::ops;

use rand::random;

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

    /// Create a random unit vector in range (-1, 1)
    pub fn rand() -> Vec3 {
        Vec3::new(1., 0., 0.)
            .rotate_z(random::<f32>() * 2. * PI)
            .rotate_x(random::<f32>() * 2. * PI)
            .norm()
    }

    pub fn with_x(&self, x: f32) -> Vec3 {
        Vec3::new(x, self.y, self.z)
    }

    pub fn with_y(&self, y: f32) -> Vec3 {
        Vec3::new(self.x, y, self.z)
    }

    pub fn with_z(&self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
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

    pub fn cos_angle(&self, other: &Vec3) -> f32 {
        self.dot(other) / (self.mag() * other.mag())
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

    /// Matrix:
    /// |   1,   0,   0|   | x |
    /// |   0, cos,-sin| x | y |
    /// |   0, sin, cos|   | z |
    pub fn rotate_x(&self, angle: f32) -> Vec3 {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec3 {
            x: 1. * self.x + 0. * self.y + 0. * self.z,
            y: 0. * self.x + cos * self.y + -sin * self.z,
            z: 0. * self.x + sin * self.y + cos * self.z,
        }
    }

    /// Matrix:
    /// | cos,   0, sin|   | x |
    /// |   0,   1,   0| x | y |
    /// |-sin,   0, cos|   | z |
    pub fn rotate_y(&self, angle: f32) -> Vec3 {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec3 {
            x: cos * self.x + 0. * self.y + sin * self.z,
            y: 0. * self.x + 1. * self.y + 0. * self.z,
            z: -sin * self.x + 0. * self.y + cos * self.z,
        }
    }

    /// Matrix:
    /// | cos,-sin,   0|   | x |
    /// | sin, cos,   0| x | y |
    /// |   0,   0,   1|   | z |
    pub fn rotate_z(&self, angle: f32) -> Vec3 {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec3 {
            x: cos * self.x + -sin * self.y + 0. * self.z,
            y: sin * self.x + cos * self.y + 0. * self.z,
            z: 0. * self.x + 0. * self.y + 1. * self.z,
        }
    }

    pub fn rotate_around(&self, angle: f32, axis: &Vec3) -> Vec3 {
        self.mul_n(angle.cos())
            + self.cross(axis).mul_n(angle.sin())
            + axis.mul_n(axis.dot(self) * (1. - angle.cos()))
    }

    pub fn mul_n(&self, n: f32) -> Vec3 {
        *self * Vec3::diag(n)
    }

    pub fn approx_eq(&self, other: &Vec3) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }

    /// [ref](https://math.stackexchange.com/a/13263/1028553)
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - Vec3::diag(2. * self.dot(n)) * *n
    }

    pub fn orth(&self) -> Vec3 {
        Vec3 {
            x: self.x.copysign(self.z),
            y: self.y.copysign(self.z),
            z: -self.z.copysign(self.x) - self.z.copysign(self.y),
        }
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

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod test {
    use crate::vec3::Vec3;

    #[test]
    fn orth() {
        let v = Vec3::new(1., 0., 0.);
        let o = v.orth();
        assert!(v.mag().eq(&o.mag()))
    }
}
