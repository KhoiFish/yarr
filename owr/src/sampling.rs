use crate::hittable::{Hittable, HittableList};
use crate::ray::{Ray};
use crate::types::*;
use crate::color;
use crate::camera;
use crate::utils;

// --------------------------------------------------------------------------------------------------------------------

pub fn shoot_ray(r : &Ray<Float>, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    match world.hit(&r, 0.001, Float::MAX) {
        Some(hit) => {
            match hit.material.scatter(&r, &hit) {
                Some(scatter_result) => {
                    return color::multiply_colors(
                        &shoot_ray(&scatter_result.scattered, world, depth-1),
                        &scatter_result.attenuation
                    );
                }
                _ => {
                    return Color::default();
                }
            }
        }
        _ => {}
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}

// --------------------------------------------------------------------------------------------------------------------

pub fn one_sample(image_x: u32, image_y: u32, params: &RaytracerParams, camera: &camera::Camera, world: &HittableList) -> Color {
    let u = ((image_x as Float) + utils::random_range(0.0, 1.0)) / ((params.image_width - 1) as Float);
    let v = ((image_y as Float) + utils::random_range(0.0, 1.0)) / ((params.image_height - 1) as Float);
    let r = camera.get_ray(u, v);
    
    shoot_ray(&r, &world, params.max_depth)
}

// --------------------------------------------------------------------------------------------------------------------

pub fn multi_sample(image_x: u32, image_y: u32, params: &RaytracerParams, camera: &camera::Camera, world: &HittableList) -> Color {
    let mut pixel_color = Color::default();
    for _s in 0..params.samples_per_pixel {
        pixel_color = pixel_color + one_sample(image_x, image_y, &params, &camera, &world);
    }

    color::normalize_color(&pixel_color, params.samples_per_pixel)
}
