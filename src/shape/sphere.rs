use crate::math::sq_diff_root;
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
        let l = self.center - ray.start;
        let t_ca = l.dot(&ray.dir);
        if t_ca < 0. {
            return None;
        }
        let d = sq_diff_root(l.mag(), t_ca);
        if d > self.radius {
            return None;
        }
        let t_hc = sq_diff_root(self.radius, d);
        let p1 = ray.with_param(t_ca - t_hc);

        let normal = (p1 - self.center).norm();
        let dir = (p1 - ray.start).reflect(&normal).norm();

        Some(Ray { start: p1, dir })
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;

    use crate::ray::Ray;
    use crate::shape::sphere::Sphere;
    use crate::shape::Shape;
    use crate::vec3::Vec3;

    #[test]
    fn not_intersect_other_direction() {
        let s = Sphere {
            center: Vec3::new(2., 0., 1.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 0., 1.),
            dir: Vec3::new(-1., 0., 0.),
        };

        let ray = s.reflect(&r);

        assert!(ray.is_none())
    }

    #[test]
    fn not_intersect_far() {
        let s = Sphere {
            center: Vec3::new(2., 0., 1.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 0., 1.),
            dir: Vec3::new(1., 1., 0.).norm(),
        };

        let ray = s.reflect(&r);

        assert!(ray.is_none())
    }

    #[test]
    fn intersect_touch() {
        let s = Sphere {
            center: Vec3::new(2., 0., 1.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 1., 1.),
            dir: Vec3::new(1., 0., 0.),
        };

        let ray = s.reflect(&r);

        assert!(ray.unwrap().start.approx_eq(&Vec3::new(2., 1., 1.)))
    }

    #[test]
    fn intersect_through() {
        let s = Sphere {
            center: Vec3::new(2., 0., 1.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 0., 1.),
            dir: Vec3::new(1., 0., 0.),
        };

        let ray = s.reflect(&r);

        assert!(ray.unwrap().start.approx_eq(&Vec3::new(1., 0., 1.)));
    }

    #[test]
    fn reflect_touch() {
        let s = Sphere {
            center: Vec3::new(2., 0., 1.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 1., 1.),
            dir: Vec3::new(1., 0., 0.),
        };

        let ray = s.reflect(&r).unwrap();

        assert!(ray.dir.approx_eq(&Vec3::new(1., 0., 0.)))
    }

    #[test]
    fn reflect_through() {
        let s = Sphere {
            center: Vec3::new(2., 0., 1.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., 0., 1.),
            dir: Vec3::new(1., 0., 0.),
        };

        let ray = s.reflect(&r).unwrap();

        assert!(ray.dir.approx_eq(&Vec3::new(-1., 0., 0.)));
    }

    #[test]
    fn reflect_diag() {
        let s = Sphere {
            center: Vec3::new(2., 0., 1.),
            radius: 1.0,
        };
        let r = Ray {
            start: Vec3::new(0., f32::cos(PI / 4.), 1.),
            dir: Vec3::new(1., 0., 0.),
        };

        let ray = s.reflect(&r).unwrap();

        assert!(ray.dir.approx_eq(&Vec3::new(0., 1., 0.)));
    }
}
