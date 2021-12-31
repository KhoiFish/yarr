use crate::types::*;
use crate::vec3::{Vec3};
use crate::ray::{Ray};
use std::mem;

// --------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Aabb {
    pub min: Vec3::<Float>,
    pub max: Vec3::<Float>,
}

impl Aabb {
    pub fn default() -> Self {
        Self {
            min: Vec3::<Float>::default(),
            max: Vec3::<Float>::default()
        }
    }

    pub fn new(other: &Self, offset: &Vec3<Float>) -> Self {
        Self {
            min: other.min + *offset,
            max: other.max + *offset
        }
    }

    pub fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> bool {
        for i in 0..3 {
            if r.dir[i].abs() < 1e-8 {
                // Skip, avoid divide by zero
                continue;
            }

            let inv_d = 1.0 / r.dir[i];
            let mut t0 = (self.min[i] - r.orig[i]) * inv_d;
            let mut t1 = (self.max[i] - r.orig[i]) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }

            let new_min = Float::max(t0, t_min);
            let new_max = Float::min(t1, t_max);
            if new_max <= new_min {
                return false;
            }
        }
        
        return true;
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let min = Vec3::<Float>::new(
            Float::min(box0.min.x(), box1.min.x()),
            Float::min(box0.min.y(), box1.min.y()),
            Float::min(box0.min.z(), box1.min.z()));
    
        let max = Vec3::<Float>::new(
            Float::max(box0.max.x(), box1.max.x()),
            Float::max(box0.max.y(), box1.max.y()),
            Float::max(box0.max.z(), box1.max.z()));
    
        Aabb { min, max }
    }
}
