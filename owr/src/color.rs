use crate::types::*;
use crate::log_print;
use crate::utils;

// --------------------------------------------------------------------------------------------------------------------

pub fn normalize_color(src: &Color, samples_per_pixel: u32) -> Color{
    let scale = 1.0 / samples_per_pixel as Float;

    Color::new(
    (src.x() * scale).sqrt().clamp(0.0, 0.999),
    (src.y() * scale).sqrt().clamp(0.0, 0.999),
    (src.z() * scale).sqrt().clamp(0.0, 0.999))
}

pub fn multiply_colors(c1: &Color, c2: &Color) -> Color {
    Color::new(
        c1.x() * c2.x(),
        c1.y() * c2.y(),
        c1.z() * c2.z()
    )
}

pub fn get_color_u32(color: &Color) -> u32 {
    let r = (color.x() * 256.0) as u32;
    let g = (color.y() * 256.0) as u32;
    let b =  (color.z() * 256.0) as u32;
    let color_u32 = (r << 16) | (g << 8) | (b << 0);

    color_u32
}

pub fn get_color_components(color: &Color, r: &mut u8, g: &mut u8, b: &mut u8) {
    *r = (color.x() * 256.0) as u8;
    *g = (color.y() * 256.0) as u8;
    *b = (color.z() * 256.0) as u8;
}

pub fn print_color(color: &Color) {
    log_print!("{0} {1} {2}\n", (color.x() * 256.0) as i32 , (color.y() * 256.0) as i32, (color.z() * 256.0) as i32);
}

// --------------------------------------------------------------------------------------------------------------------

pub fn random() -> Color {
    Color::new(utils::random_float(), utils::random_float(), utils::random_float())
}

pub fn random_range(min: Float, max: Float) -> Color {
    Color::new(utils::random_range(min, max), utils::random_range(min, max), utils::random_range(min, max))
}