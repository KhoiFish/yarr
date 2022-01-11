use crate::hittable::{Hittable};
use crate::ray::{Ray};
use crate::vec3::Vec3;
use crate::types::*;
use crate::color;
use crate::camera;
use crate::utils;

use rayon::prelude::*;
use std::sync::Arc;
extern crate image;

#[cfg(feature = "progress-ui")]
use indicatif::{ParallelProgressIterator, ProgressIterator, ProgressBar};

// --------------------------------------------------------------------------------------------------------------------

pub fn shoot_ray(r : &Ray<Float>, background: &Vec3<Float>, world: &Arc<dyn Hittable>, depth: u32) -> Vec3<Float> {
    if depth <= 0 {
        return Vec3::<Float>::default();
    }

    match world.hit(&r, 0.001, Float::MAX) {
        // Hit something
        Some(hit) => {
            match hit.material.scatter(&r, &hit) {
                Some(scatter_result) => {
                    let emitted = hit.material.emitted(hit.u, hit.v, &hit.point);
                    return emitted + shoot_ray(&scatter_result.scattered, &background, world, depth-1) * scatter_result.attenuation;
                }
                _ => {
                    return hit.material.emitted(hit.u, hit.v, &hit.point);
                }
            }
        }
        // No hits
        _ => {
            return *background;
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------

pub fn one_sample(image_x: u32, image_y: u32, params: &RaytracerParams, camera: &camera::Camera, world: &Arc<dyn Hittable>) -> Vec3<Float> {
    let u = ((image_x as Float) + utils::random_range(0.0, 1.0)) / ((params.image_width - 1) as Float);
    let v = ((image_y as Float) + utils::random_range(0.0, 1.0)) / ((params.image_height - 1) as Float);
    let r = camera.get_ray(u, v);
    
    shoot_ray(&r, &camera.get_background(), &world, params.max_depth)
}

// --------------------------------------------------------------------------------------------------------------------

pub fn multi_sample(enable_average_sum: bool, image_x: u32, image_y: u32, params: &RaytracerParams, camera: &camera::Camera, world: &Arc<dyn Hittable>) -> Vec3<Float> {
    let mut sample_sum = Vec3::default();
    for _s in 0..params.samples_per_pixel {
        sample_sum = sample_sum + one_sample(image_x, image_y, &params, &camera, &world);
    }

    // Average out with num samples
    if enable_average_sum {
        return sample_sum * (1.0 / params.samples_per_pixel as Float);
    } else {
        return sample_sum;
    }
}

// --------------------------------------------------------------------------------------------------------------------

fn get_grid(x0: u32, y0: u32, width: u32, height: u32) -> Vec::<(u32, u32)> {
    // How many pixels?
    let num_pixels = (width * height) as usize;

    // Create a grid so we can divide the work
    let x1 = x0 + width;
    let y1 = y0 + height;
    let mut grid = Vec::<(u32, u32)>::with_capacity(num_pixels);
    for y in y0..y1 {
        for x in x0..x1 {
            grid.push((x,y));
        }
    }

    grid
}

// --------------------------------------------------------------------------------------------------------------------

pub fn multisample_image_region(x0: u32, y0: u32, width: u32, height: u32, params: &RaytracerParams, camera: &camera::Camera, world: &Arc<dyn Hittable>) -> Vec<u8> {
    let grid = get_grid(x0, y0, width, height);

    return grid.iter().flat_map(|&point| -> Color {
        color::vec3_to_color(&multi_sample(true, point.0, point.1, &params, &camera, &world), 1.0)
    }).collect();
}

// --------------------------------------------------------------------------------------------------------------------

fn multisample_image(enable_progress_bar: bool, params: &RaytracerParams, camera: &camera::Camera, world: &Arc<dyn Hittable>) -> Vec<u8> {
    let grid = get_grid(0, 0, params.image_width, params.image_height);

    if enable_progress_bar {
        #[cfg(feature = "progress-ui")]
        {
            let pb = ProgressBar::new(grid.len() as u64);
            pb.set_draw_delta(64);

            return grid.iter().progress_with(pb).flat_map(|&point| -> Color {
                color::vec3_to_color(&multi_sample(true, point.0, point.1, &params, &camera, &world), 1.0)
            }).collect()
        }
    }

    return grid.iter().flat_map(|&point| -> Color {
        color::vec3_to_color(&multi_sample(true, point.0, point.1, &params, &camera, &world), 1.0)
    }).collect();
}

// --------------------------------------------------------------------------------------------------------------------

fn multisample_image_parallel(enable_progress_bar: bool, params: &RaytracerParams, camera: &camera::Camera, world: &Arc<dyn Hittable>) -> Vec<u8> {
    let grid = get_grid(0, 0, params.image_width, params.image_height);

    if enable_progress_bar {
        #[cfg(feature = "progress-ui")]
        {
            let pb = ProgressBar::new(grid.len() as u64);
            pb.set_draw_delta(64);

            return grid.par_iter().progress_with(pb).flat_map(|&point| -> Color {
                color::vec3_to_color(&multi_sample(true, point.0, point.1, &params, &camera, &world), 1.0)
            }).collect()
        }
    }

    return grid.par_iter().flat_map(|&point| -> Color {
        color::vec3_to_color(&multi_sample(true, point.0, point.1, &params, &camera, &world), 1.0)
    }).collect();
}

// --------------------------------------------------------------------------------------------------------------------

pub fn multi_sample_buffer(enable_average_sum: bool, enable_parallel: bool, params: &RaytracerParams, camera: &camera::Camera, world: &Arc<dyn Hittable>) -> Vec::<Float> {
    let grid = get_grid(0, 0, params.image_width, params.image_height);

    // Iterate and collect results
    if enable_parallel {
        return grid.par_iter()
            .flat_map(|&point| -> [Float; 4] {
                let &result = multi_sample(enable_average_sum, point.0, point.1, &params, &camera, &world).array();
                [result[0], result[1], result[2], 1.0]
            }).collect();
    } else {
        return grid.iter()
            .flat_map(|&point| -> [Float; 4] {
                let &result = multi_sample(enable_average_sum, point.0, point.1, &params, &camera, &world).array();
                [result[0], result[1], result[2], 1.0]
            }).collect();
    }
}

// --------------------------------------------------------------------------------------------------------------------

pub fn render_image(enable_parallel: bool, enable_progress_bar: bool, params: &RaytracerParams, camera: &camera::Camera, world: &Arc<dyn Hittable>) -> Option<image::RgbaImage> {
    // Iterate and collect results
    let results;
    if enable_parallel {
        results = multisample_image_parallel(enable_progress_bar, &params, &camera, &world);
    } else {
        results = multisample_image(enable_progress_bar, &params, &camera, &world);
    }

    image::RgbaImage::from_raw(params.image_width, params.image_height, results)
}
