use crate::types::*;
use crate::log_print;
use crate::utils;

// --------------------------------------------------------------------------------------------------------------------

pub fn write_color(color: &Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as Float;
    let r = (color.x() * scale).sqrt().clamp(0.0, 0.999);
    let g = (color.y() * scale).sqrt().clamp(0.0, 0.999);
    let b = (color.z() * scale).sqrt().clamp(0.0, 0.999);

    log_print!("{0} {1} {2}\n", (r * 256.0) as i32 , (g * 256.0) as i32, (b * 256.0) as i32);
}

pub fn multiply_colors(c1: &Color, c2: &Color) -> Color {
    Color::new(
        c1.x() * c2.x(),
        c1.y() * c2.y(),
        c1.z() * c2.z()
    )
}

pub fn random() -> Color {
    Color::new(utils::random_float(), utils::random_float(), utils::random_float())
}

pub fn random_range(min: Float, max: Float) -> Color {
    Color::new(utils::random_range(min, max), utils::random_range(min, max), utils::random_range(min, max))
}