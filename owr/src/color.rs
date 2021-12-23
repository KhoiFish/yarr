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
