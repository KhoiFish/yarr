use crate::vec3::*;
use crate::ray::{Ray};
use crate::hittable::{Hittable, HitRecord};
use crate::material::{Material};
use std::rc::Rc;

// --------------------------------------------------------------------------------------------------------------------
// Sphere definition & implementation

pub struct Sphere {
    pub center: Vec3<f64>,
    pub radius: f64,
    pub material: Rc<dyn Material>
}

// --------------------------------------------------------------------------------------------------------------------
// Hittable trait implementation

impl Hittable for Sphere {
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant : f64 = (half_b * half_b) - (a*c);
        if discriminant < 0.0 {
            return Option::None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies within our range, t-min/t-max
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return Option::None;
            }
        }

        let t = root;
        let p = r.at(t);
        let n = (p - self.center) * (1.0/self.radius);
        
        Some(HitRecord::new(&r, &p, &n, t, self.material.clone()))
    }
}