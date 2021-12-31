use crate::ray::Ray;
use crate::utils::random_float;
use crate::vec3::Vec3;
use crate::material::{Material};
use crate::aabb::Aabb;
use crate::texture;
use crate::material;
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
        let normal_and_direction = HitRecord::get_normal_and_direction(&ray, &normal);

        Self {
            point: *point,
            normal: normal_and_direction.0,
            t,
            u,
            v,
            front_facing: normal_and_direction.1, 
            material
        }
    }

    pub fn get_normal_and_direction(ray: &Ray<Float>, normal: &Vec3<Float>) -> (Vec3<Float>, bool) {
        let front_facing = ray.dir.dot(&normal) < 0.0;
        let new_normal;
        if front_facing {
            new_normal = *normal;
        } else {
            new_normal = normal.reverse_dir();
        }

        (new_normal, front_facing)
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

// --------------------------------------------------------------------------------------------------------------------
// XZ Rect

pub struct XZRect {
    pub x0: Float,   
    pub x1: Float,
    pub z0: Float,   
    pub z1: Float,
    pub k: Float,
    pub material: Arc<dyn Material>,
}

impl XZRect {
    pub fn new(x0: Float, x1: Float, z0: Float, z1: Float, k: Float, material: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let t = (self.k - r.orig.y()) / r.dir.y();
        if t < t_min || t > t_max {
            return Option::None;
        }

        let x = r.orig.x() + t*r.dir.x();
        let z = r.orig.z() + t*r.dir.z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return Option::None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        Some(HitRecord::new(&r, &r.at(t), &Vec3::new(0.0, 1.0, 0.0), t, u, v, self.material.clone()))
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<Aabb> {
        Some(Aabb {
            min: Vec3::new(self.x0, self.z0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.z1, self.k + 0.0001)
        })
    }
}

unsafe impl Sync for XZRect {}
unsafe impl Send for XZRect {}

// --------------------------------------------------------------------------------------------------------------------
// XZ Rect

pub struct YZRect {
    pub y0: Float,   
    pub y1: Float,
    pub z0: Float,   
    pub z1: Float,
    pub k: Float,
    pub material: Arc<dyn Material>,
}

impl YZRect {
    pub fn new(y0: Float, y1: Float, z0: Float, z1: Float, k: Float, material: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let t = (self.k - r.orig.x()) / r.dir.x();
        if t < t_min || t > t_max {
            return Option::None;
        }

        let y = r.orig.y() + t*r.dir.y();
        let z = r.orig.z() + t*r.dir.z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return Option::None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        Some(HitRecord::new(&r, &r.at(t), &Vec3::new(1.0, 0.0, 0.0), t, u, v, self.material.clone()))
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<Aabb> {
        Some(Aabb {
            min: Vec3::new(self.y0, self.z0, self.k - 0.0001),
            max: Vec3::new(self.y1, self.z1, self.k + 0.0001)
        })
    }
}

unsafe impl Sync for YZRect {}
unsafe impl Send for YZRect {}

// --------------------------------------------------------------------------------------------------------------------
// Box

pub struct Box {
    min: Vec3<Float>,
    max: Vec3<Float>,
    sides: HittableList,
}

impl Box {
    pub fn new(p0: &Vec3::<Float>, p1: &Vec3::<Float>, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::default();

        sides.list.push(Arc::new(
            XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material.clone())));
        sides.list.push(Arc::new(
            XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material.clone())));

        sides.list.push(Arc::new(
            XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material.clone())));
        sides.list.push(Arc::new(
            XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material.clone())));

        sides.list.push(Arc::new(
            YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material.clone())));
        sides.list.push(Arc::new(
            YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material.clone())));
        
        Self {
            min: *p0,
            max: *p1,
            sides
        }
    }
}

impl Hittable for Box {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        self.sides.hit(&r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<Aabb> {
        Some(Aabb {
            min: self.min,
            max: self.max
        })
    }
}

unsafe impl Sync for Box {}
unsafe impl Send for Box {}

// --------------------------------------------------------------------------------------------------------------------
// Translate

pub struct Translate {
    hittable: Arc<dyn Hittable>,
    displacement: Vec3<Float>
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, displacement: Vec3<Float>) -> Self {
        Self {
            hittable,
            displacement
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let moved_r = Ray {
            orig: r.orig - self.displacement,
            dir: r.dir,
            time: r.time
        };

        let hit_option = self.hittable.hit(&moved_r, t_min, t_max);
        if hit_option.is_none() {
            return Option::None;
        }

        let mut hit = hit_option.unwrap();
        let normal_and_direction = HitRecord::get_normal_and_direction(&moved_r, &hit.normal);
        hit.point = hit.point + self.displacement;
        hit.normal = normal_and_direction.0;
        hit.front_facing = normal_and_direction.1;

        Some(hit)
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<Aabb> {
        let bbox = self.hittable.bounding_box(time0, time1);
        if bbox.is_none() {
            return Option::None;
        }

        return Some(Aabb::new(&bbox.unwrap(), &self.displacement));
    }
}

unsafe impl Sync for Translate {}
unsafe impl Send for Translate {}

// --------------------------------------------------------------------------------------------------------------------
// Rotate Y

pub struct RotateY {
    hittable: Arc<dyn Hittable>,
    sin_theta: Float,
    cos_theta: Float,
    bbox_option: Option<Aabb>
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable>, angle: Float) -> Self {
        let radians = Float::to_radians(angle);
        let sin_theta = Float::sin(radians);
        let cos_theta = Float::cos(radians);
        let mut bbox_option = hittable.bounding_box(0.0, 1.0);
        let mut min = Vec3::new( Float::MAX, Float::MAX, Float::MAX);
        let mut max = Vec3::new( Float::MIN, Float::MIN, Float::MIN);

        if bbox_option.is_some() {
            let bbox = bbox_option.unwrap();
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as Float * bbox.max.x() + (1-i) as Float * bbox.min.x();
                        let y = j as Float * bbox.max.y() + (1-j) as Float * bbox.min.y();
                        let z = k as Float * bbox.max.z() + (1-k) as Float * bbox.min.z();
        
                        let new_x =  cos_theta*x + sin_theta*z;
                        let new_z = -sin_theta*x + cos_theta*z;

                        let tester = Vec3::new(new_x, y, new_z);
                        for c in 0..3 {
                            min[c] = Float::min(min[c], tester[c]);
                            max[c] = Float::max(max[c], tester[c]);
                        }
                    }
                }
            }

            bbox_option = Some(Aabb {
                min,
                max
            })
        }

        Self {
            hittable,
            sin_theta,
            cos_theta,
            bbox_option
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;

        origin[0] = self.cos_theta*r.orig[0] - self.sin_theta*r.orig[2];
        origin[2] = self.sin_theta*r.orig[0] + self.cos_theta*r.orig[2];

        direction[0] = self.cos_theta*r.dir[0] - self.sin_theta*r.dir[2];
        direction[2] = self.sin_theta*r.dir[0] + self.cos_theta*r.dir[2];

        let rotated_r = Ray {
            orig: origin,
            dir: direction, 
            time: r.time
        };

        let hit_option = self.hittable.hit(&rotated_r, t_min, t_max);
        if hit_option.is_none() {
            return Option::None;
        }

        let mut hit = hit_option.unwrap();

        let mut p = hit.point.clone();
        let mut normal = hit.normal.clone();

        p[0] =  self.cos_theta*hit.point[0] + self.sin_theta*hit.point[2];
        p[2] = -self.sin_theta*hit.point[0] + self.cos_theta*hit.point[2];

        normal[0] =  self.cos_theta*hit.normal[0] + self.sin_theta*hit.normal[2];
        normal[2] = -self.sin_theta*hit.normal[0] + self.cos_theta*hit.normal[2];

        let normal_and_direction = HitRecord::get_normal_and_direction(&rotated_r, &normal);
        hit.point = p;
        hit.normal = normal_and_direction.0;
        hit.front_facing = normal_and_direction.1;

        Some(hit)
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<Aabb> {
        self.bbox_option
    }
}

unsafe impl Sync for RotateY {}
unsafe impl Send for RotateY {}

// --------------------------------------------------------------------------------------------------------------------
// Constant medium

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: Float
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: Float, texture: Arc<dyn texture::Texture>) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(material::Isotropic::new(texture)),
            neg_inv_density: -1.0 / density
        }
    }

    pub fn new_with_constant_color(boundary: Arc<dyn Hittable>, density: Float, color: &Vec3<Float>) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(material::Isotropic::new_from_color(&color)),
            neg_inv_density: -1.0 / density
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let hit1_option = self.boundary.hit(&r, Float::MIN, Float::MAX);
        if hit1_option.is_none() {
            return Option::None;
        }

        let mut rec1 = hit1_option.unwrap();
        let hit2_option = self.boundary.hit(&r, rec1.t + 0.0001, Float::MAX);
        if hit2_option.is_none() {
            return Option::None;
        }
        let mut rec2 = hit2_option.unwrap();

        if rec1.t < t_min { rec1.t = t_min };
        if rec2.t > t_max { rec2.t = t_max };

        if rec1.t >= rec2.t {
            return Option::None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * Float::ln(random_float());

        if hit_distance > distance_inside_boundary {
            return Option::None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let point = r.at(t);
        let normal = Vec3::new(1.0, 0.0, 0.0);
        let front_facing = true;


        Some( HitRecord {
            t,
            u: 0.0,
            v: 0.0,
            point,
            normal,
            front_facing,
            material: self.phase_function.clone()
        })
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}

unsafe impl Sync for ConstantMedium {}
unsafe impl Send for ConstantMedium {}