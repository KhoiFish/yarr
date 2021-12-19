use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::utils;

// --------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)]
pub struct Camera {
    origin: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: Vec3<f64>,
    lens_radius: f64
}

impl Camera {
    pub fn new(look_from: &Vec3<f64>, look_at: &Vec3<f64>, up: &Vec3<f64>, fov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
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

    pub fn get_ray(&self, s: f64, t: f64) -> Ray<f64> {
        let rd = utils::random_unitdisk_vec3() * self.lens_radius;
        let offset = self.u*rd.x() + self.v*rd.y();

        Ray::<f64> { 
            orig: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset
        }
    }
}