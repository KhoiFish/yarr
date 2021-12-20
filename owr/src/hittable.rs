use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::{Material};
use std::rc::Rc;
use crate::types::*;

// --------------------------------------------------------------------------------------------------------------------
// Hit record

pub struct HitRecord {
    pub point: Vec3<Float>,
    pub normal: Vec3<Float>,
    pub t: Float,
    pub front_facing: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(ray: &Ray<Float>, point: &Vec3<Float>, normal: &Vec3<Float>, t: Float, material: Rc<dyn Material>) -> HitRecord {
        let front_face = ray.dir.dot(&normal) < 0.0;
        let new_normal;
        if front_face {
            new_normal = *normal;
        } else {
            new_normal = normal.reverse_dir();
        }

        Self {
            point: *point,
            normal: new_normal,
            t: t,
            front_facing: front_face, 
            material: material
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------
// Hittable trait

pub trait Hittable {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord>; 
}

// --------------------------------------------------------------------------------------------------------------------
// Hittable list

#[derive(Default)]
pub struct HittableList {
    pub list: Vec<Rc<dyn Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut return_option = Option::None;
        let mut closest_so_far = t_max;

        for element in &self.list {
            match element.hit(&r, t_min, closest_so_far) {
                Some(hit_record) => { 
                    closest_so_far = hit_record.t;
                    return_option = Some(hit_record);
                }
                _ => {}
            }
        }

        return_option
    }
}