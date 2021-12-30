use std::ops::Shl;

use crate::types::*;
use crate::log_print;
use crate::vec3::Vec3;

// --------------------------------------------------------------------------------------------------------------------

pub fn gamma_correct(c: Float) -> Float {
    c.sqrt().clamp(0.0, 0.999)
}

pub fn convert_to_u8_range(c: Float) -> u8 {
    (c * 256.0) as u8
}

pub fn color_to_vec3(color: &Color) -> Vec3<Float> {
    let scale = 1.0 / 255.0;
    Vec3::new(
        (color[0] as Float) * scale,
        (color[1] as Float) * scale,
        (color[2] as Float) * scale
    )
}

pub fn vec3_to_color(color: &Vec3<Float>, alpha: Float) -> Color {
    [
        convert_to_u8_range(gamma_correct(color[0])),
        convert_to_u8_range(gamma_correct(color[1])),
        convert_to_u8_range(gamma_correct(color[2])),
        convert_to_u8_range(alpha)
    ]
}

pub fn vec3_to_u32(color: &Vec3<Float>, alpha: Float) -> u32 {
    let color_array = vec3_to_color(&color, alpha);
     
    (color_array[0] as u32).shl(24) | 
    (color_array[1] as u32).shl(16) | 
    (color_array[2] as u32).shl(8) | 
    (color_array[3] as u32) 
}

pub fn print_color(color: &Color) {
    log_print!("{} {} {}\n", color[0], color[1], color[2]);
}

