pub mod color;
pub mod ray;
pub mod vec3;
pub mod hittable;
pub mod sphere;
pub mod camera;
pub mod material;
pub mod utils;
pub mod examples;
pub mod types;

use hittable::{Hittable, HittableList};
use color::{Color64};
use ray::{Ray};

// --------------------------------------------------------------------------------------------------------------------

pub fn ray_color(r : &Ray<f64>, world: &HittableList, depth: u32) -> Color64 {
    if depth <= 0 {
        return Color64::default();
    }

    match world.hit(&r, 0.001, f64::MAX) {
        Some(hit) => {
            match hit.material.scatter(&r, &hit) {
                Some(scatter_result) => {
                    return color::multiply_colors(
                        &ray_color(&scatter_result.scattered, world, depth-1),
                        &scatter_result.attenuation
                    );
                }
                _ => {
                    return Color64::default();
                }
            }
        }
        _ => {}
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (Color64::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color64::new(0.5, 0.7, 1.0) * t)
}
