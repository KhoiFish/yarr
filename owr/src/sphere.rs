use crate::types::*;
use crate::vec3::*;
use crate::ray::{Ray};
use crate::hittable::{Hittable, HitRecord};
use crate::material::{Material};
use crate::aabb::Aabb;
use crate::utils;

use std::sync::Arc;

// --------------------------------------------------------------------------------------------------------------------

pub struct Sphere {
    pub center: Vec3<Float>,
    pub radius: Float,
    pub material: Arc<dyn Material>
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a*c);
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
        let uv = utils::get_sphere_uv(&n);
        
        Some(HitRecord::new(
            &r, 
            &p, 
            &n, 
            t,
            uv.0,
            uv.1,
            self.material.clone()))
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<Aabb> {
        Some(Aabb {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius)
        })
    }
}

// --------------------------------------------------------------------------------------------------------------------

pub struct MovingSphere {
    pub center0: Vec3<Float>,
    pub center1: Vec3<Float>,
    pub time0: Float,
    pub time1: Float,
    pub radius: Float,
    pub material: Arc<dyn Material>
}

impl MovingSphere {
    pub fn center(&self, time: Float) -> Vec3::<Float> {
        self.center0 + ((self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0)))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = r.orig - self.center(r.time);
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a*c);
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
        let n = (p - self.center(r.time)) * (1.0/self.radius);
        let uv = utils::get_sphere_uv(&n);
        
        Some(HitRecord::new(
            &r, 
            &p, 
            &n, 
            t,
            uv.0,
            uv.1,
            self.material.clone()))
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<Aabb> {
        let box0 = Aabb {
            min: self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(time0) + Vec3::new(self.radius, self.radius, self.radius)
        };

        let box1 = Aabb {
            min: self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(time1) + Vec3::new(self.radius, self.radius, self.radius)
        };

        Some(Aabb::surrounding_box(&box0, &box1))
    }
}