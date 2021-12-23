use crate::types::*;
use crate::log_print;
use crate::utils;

// --------------------------------------------------------------------------------------------------------------------

pub fn new_color(r: Float, g: Float, b: Float, a: Float) -> Color {
    [r, g, b, a]
}

pub fn normalize_color(src: &Color, samples_per_pixel: u32) -> Color{
    let scale = 1.0 / samples_per_pixel as Float;
    [
        (src[0] * scale).sqrt().clamp(0.0, 0.999),
        (src[1] * scale).sqrt().clamp(0.0, 0.999),
        (src[2] * scale).sqrt().clamp(0.0, 0.999),
        (src[3]),
    ]
}

pub fn multiply_colors(c1: &Color, c2: &Color) -> Color {
    [
        c1[0] * c2[0],
        c1[1] * c2[1],
        c1[2] * c2[2],
        1.0,
    ]
}

pub fn add_colors(c1: &Color, c2: &Color) -> Color {
    [
        c1[0] + c2[0],
        c1[1] + c2[1],
        c1[2] + c2[2],
        1.0,
    ]
}

pub fn print_color(color: &Color) {
    log_print!("{0} {1} {2}\n", (color[0] * 256.0) as i32 , (color[1] * 256.0) as i32, (color[2] * 256.0) as i32);
}

// --------------------------------------------------------------------------------------------------------------------

pub fn random() -> Color {
    [utils::random_float(), utils::random_float(), utils::random_float(), 1.0]
}

pub fn random_range(min: Float, max: Float) -> Color {
    [utils::random_range(min, max), utils::random_range(min, max), utils::random_range(min, max), 1.0]
}