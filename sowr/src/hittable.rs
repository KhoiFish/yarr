use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::{Material};
use std::rc::Rc;

// --------------------------------------------------------------------------------------------------------------------
// Hit record

pub struct HitRecord {
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub t: f64,
    pub front_facing: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(ray: &Ray<f64>, point: &Vec3<f64>, normal: &Vec3<f64>, t: f64, material: Rc<dyn Material>) -> HitRecord {
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
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord>; 
}

// --------------------------------------------------------------------------------------------------------------------
// Hittable list

#[derive(Default)]
pub struct HittableList {
    pub list: Vec<Rc<dyn Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
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