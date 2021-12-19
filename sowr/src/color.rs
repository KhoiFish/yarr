use crate::vec3;
use crate::utils;
pub type Color64 = vec3::Vec3<f64>;
pub type Color32 = vec3::Vec3<f32>;

// --------------------------------------------------------------------------------------------------------------------

pub fn write_color64(color: &Color64, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x() * scale).sqrt().clamp(0.0, 0.999);
    let g = (color.y() * scale).sqrt().clamp(0.0, 0.999);
    let b = (color.z() * scale).sqrt().clamp(0.0, 0.999);

    print!("{0} {1} {2}\n", (r * 256.0) as i32 , (g * 256.0) as i32, (b * 256.0) as i32);
}

#[allow(dead_code)]
pub fn write_color32(color: &Color32, samples_per_pixel: u32) {
    write_color64(&Color64::new(color.x() as f64, color.y() as f64, color.z() as f64), samples_per_pixel)
}

pub fn multiply_colors(c1: &Color64, c2: &Color64) -> Color64 {
    Color64::new(
        c1.x() * c2.x(),
        c1.y() * c2.y(),
        c1.z() * c2.z()
    )
}

pub fn random() -> Color64 {
    Color64::new(utils::random_float(), utils::random_float(), utils::random_float())
}

pub fn random_range(min: f64, max: f64) -> Color64 {
    Color64::new(utils::random_range(min, max), utils::random_range(min, max), utils::random_range(min, max))
}