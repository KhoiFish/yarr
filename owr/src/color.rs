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

pub fn convert_to_u32_color(c: &Vec3<Float>) -> u32 {
    let r = convert_to_u8_range(c.x()) as u32;
    let g = convert_to_u8_range(c.y()) as u32;
    let b = convert_to_u8_range(c.z()) as u32;
    let a = convert_to_u8_range(1.0)as u32;

    r.shl(24) | g.shl(16) | b.shl(8) | a
}

pub fn vec3_to_color(v: &Vec3<Float>, a: Float) -> Color {
    [
        convert_to_u8_range(gamma_correct(v.x())),
        convert_to_u8_range(gamma_correct(v.y())),
        convert_to_u8_range(gamma_correct(v.z())),
        convert_to_u8_range(a)
    ]
}

pub fn print_color(color: &Color) {
    log_print!("{} {} {}\n", color[0], color[1], color[2]);
}
