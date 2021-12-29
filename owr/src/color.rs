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

pub fn vec3_to_u32(color: &Vec3<Float>, alpha: Float) -> u32 {
    let r = convert_to_u8_range(gamma_correct(color.x())) as u32;
    let g = convert_to_u8_range(gamma_correct(color.y())) as u32;
    let b = convert_to_u8_range(gamma_correct(color.z())) as u32;
    let a = convert_to_u8_range(alpha) as u32;

    r.shl(24) | g.shl(16) | b.shl(8) | a
}

pub fn vec3_to_color(color: &Vec3<Float>, alpha: Float) -> Color {
    [
        convert_to_u8_range(gamma_correct(color.x())),
        convert_to_u8_range(gamma_correct(color.y())),
        convert_to_u8_range(gamma_correct(color.z())),
        convert_to_u8_range(alpha)
    ]
}

pub fn print_color(color: &Color) {
    log_print!("{} {} {}\n", color[0], color[1], color[2]);
}
