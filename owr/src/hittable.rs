use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::{Material};
use crate::aabb::Aabb;
use std::sync::Arc;
use crate::{types::*, log_print};

// --------------------------------------------------------------------------------------------------------------------
// Hit record

pub struct HitRecord {
    pub point: Vec3<Float>,
    pub normal: Vec3<Float>,
    pub t: Float,
    pub u: Float,
    pub v: Float,
    pub front_facing: bool,
    pub material: Arc<dyn Material>
}

impl HitRecord {
    pub fn new(ray: &Ray<Float>, point: &Vec3<Float>, normal: &Vec3<Float>, t: Float, u: Float, v: Float, material: Arc<dyn Material>) -> HitRecord {
        let front_facing = ray.dir.dot(&normal) < 0.0;
        let new_normal;
        if front_facing {
            new_normal = *normal;
        } else {
            new_normal = normal.reverse_dir();
        }

        Self {
            point: *point,
            normal: new_normal,
            t,
            u,
            v,
            front_facing, 
            material
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------
// Hittable trait

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord>; 
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<Aabb>;
}

// --------------------------------------------------------------------------------------------------------------------
// Hittable list

#[derive(Default)]
pub struct HittableList {
    pub list: Vec<Arc<dyn Hittable>>
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

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<Aabb> {
        if self.list.is_empty() {
            return Option::None;
        }

        let mut ret_option = Option::None;
        for element in &self.list {
            match element.bounding_box(time0, time1) {
                // Match against previous box finds
                Some(new_box) => {
                    match ret_option {
                        // We already found a box, now build a surrounding box
                        Some(saved_box) => { ret_option = Some(Aabb::surrounding_box(&new_box, &saved_box)) }

                        // First one found
                        _ => { ret_option = Some(new_box) }
                    }
                }

                // If there are any elements without a bounding box, bail
                _ => { 
                    log_print!("Found an object without a bounding box, this is weird.");
                    return Option::None; 
                }
            }
        }

        return ret_option;
    }
}

// Pull in these traits so we can enable multi-threading
unsafe impl Sync for HittableList {}
unsafe impl Send for HittableList {}

// --------------------------------------------------------------------------------------------------------------------
// XY Rect

pub struct XYRect {
    pub x0: Float,   
    pub x1: Float,
    pub y0: Float,   
    pub y1: Float,
    pub k: Float,
    pub material: Arc<dyn Material>,
}

impl XYRect {
    pub fn new(x0: Float, x1: Float, y0: Float, y1: Float, k: Float, material: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let t = (self.k - r.orig.z()) / r.dir.z();
        if t < t_min || t > t_max {
            return Option::None;
        }

        let x = r.orig.x() + t*r.dir.x();
        let y = r.orig.y() + t*r.dir.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return Option::None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        Some(HitRecord::new(&r, &r.at(t), &Vec3::new(0.0, 0.0, 1.0), t, u, v, self.material.clone()))
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<Aabb> {
        Some(Aabb {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001)
        })
    }
}

unsafe impl Sync for XYRect {}
unsafe impl Send for XYRect {}
