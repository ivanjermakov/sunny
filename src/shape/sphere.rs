use crate::math::hypot;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Shape for Sphere {
    /// [guide](https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html)
    fn reflect(&self, ray: &Ray) -> Option<Ray> {
        let dir = ray.dir.norm();
        let c = self.center;
        let o = ray.start;
        let l = c - o;
        let t_ca = l.dot(&dir);
        if t_ca < 0. {
            return None;
        }
        let d = hypot(l.mag(), t_ca);
        dbg!(dir, l, t_ca, d);
        if d > self.radius {
            return None;
        }
        let t_hc = hypot(self.radius, d);
        let p1 = o + dir * Vec3::diag(t_ca - t_hc);

        // TODO: reflection

        Some(Ray {
            start: p1,
            dir: Vec3::zero(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::ray::Ray;
    use crate::shape::sphere::Sphere;
    use crate::shape::Shape;
    use crate::vec3::Vec3;

    #[test]
    fn not_intersect_other_direction() {
        let s = Sphere {
            center: Vec3::new(10., 0., 0.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 0., 0.),
            dir: Vec3::new(-1., 0., 0.),
        };

        let ray = s.reflect(&r);

        assert!(ray.is_none())
    }

    #[test]
    fn not_intersect_far() {
        let s = Sphere {
            center: Vec3::new(10., 0., 0.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 0., 0.),
            dir: Vec3::new(1., 1., 0.).norm(),
        };

        let ray = s.reflect(&r);

        assert!(ray.is_none())
    }

    #[test]
    fn intersect_touch() {
        let s = Sphere {
            center: Vec3::new(10., 0., 0.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 1., 0.),
            dir: Vec3::new(1., 0., 0.),
        };

        let ray = s.reflect(&r);

        assert!(ray.unwrap().start.approx_eq(&Vec3::new(10., 1., 0.)))
    }

    #[test]
    fn intersect_through() {
        let s = Sphere {
            center: Vec3::new(10., 0., 0.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 0., 0.),
            dir: Vec3::new(1., 0., 0.),
        };

        let ray = s.reflect(&r);

        assert!(ray.unwrap().start.approx_eq(&Vec3::new(9., 0., 0.)));
    }
}
