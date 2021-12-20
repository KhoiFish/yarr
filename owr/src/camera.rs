use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::utils;
use crate::types::*;

// --------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)]
pub struct Camera {
    origin: Vec3<Float>,
    lower_left_corner: Vec3<Float>,
    horizontal: Vec3<Float>,
    vertical: Vec3<Float>,
    u: Vec3<Float>,
    v: Vec3<Float>,
    w: Vec3<Float>,
    lens_radius: Float
}

impl Camera {
    pub fn new(look_from: &Vec3<Float>, look_at: &Vec3<Float>, up: &Vec3<Float>, fov: Float, aspect_ratio: Float, aperture: Float, focus_dist: Float) -> Self {
        let theta = fov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (*look_from - *look_at).unit_vector();
        let u = up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = *look_from;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = origin - horizontal*0.5 - vertical*0.5 - w*focus_dist;
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius
        }
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray<Float> {
        let rd = utils::random_unitdisk_vec3() * self.lens_radius;
        let offset = self.u*rd.x() + self.v*rd.y();

        Ray::<Float> { 
            orig: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset
        }
    }
}