#![allow(dead_code)]
use crate::vec3::{Vec3};

// --------------------------------------------------------------------------------------------------------------------

#[cfg(not(target_family = "wasm"))]
#[macro_export]
macro_rules! log_print{
    ($($elem: expr),+ ) => {
        println!(concat!($(concat!(stringify!($elem), " - {:?}\n")),+), $($elem),+);
    };
}

#[cfg(not(target_family = "wasm"))]
use fastrand;

#[cfg(not(target_family = "wasm"))]
pub fn random_float() -> f64 {
    fastrand::f64()
}

// --------------------------------------------------------------------------------------------------------------------

#[cfg(target_family = "wasm")]
extern crate web_sys;

#[cfg(target_family = "wasm")]
#[macro_export]
macro_rules! log_print {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(target_family = "wasm")]
pub fn unsafe_pseudo_rand_u32() -> u32 {
    static mut NEXT : u32 = 1;
    let rand_value;
    unsafe {
        NEXT = NEXT * 1103515245 + 12345;
        rand_value = (NEXT/65536) % 32768
    }

    rand_value
}

#[cfg(target_family = "wasm")]
pub fn random_float() -> f64 {
    (unsafe_pseudo_rand_u32() as f64) / (u32::MAX as f64)
}

// --------------------------------------------------------------------------------------------------------------------

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (random_float() * (max - min))
}

pub fn random_range_vec3(min: f64, max: f64) -> Vec3<f64> {
    Vec3::<f64>::new(random_range(min, max), random_range(min, max), random_range(min, max))
}

pub fn random_unitdisk_vec3() -> Vec3<f64> {
    loop {
        let p = Vec3::<f64>::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3<f64> {
    loop {
        let p = random_range_vec3(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub fn random_in_hemisphere(normal: &Vec3<f64>) -> Vec3<f64> {
    let unit_sphere_vec = random_in_unit_sphere();
    if unit_sphere_vec.dot(&normal) > 0.0 {
        return unit_sphere_vec;
    } else {
        return unit_sphere_vec.reverse_dir();
    }
}

pub fn random_unit_vec3() -> Vec3<f64> {
    random_in_unit_sphere().unit_vector()
}

// --------------------------------------------------------------------------------------------------------------------

pub fn near_zero(v: &Vec3<f64>) -> bool {
    let s = 1e-18;

    (v.x().abs() < s) && (v.y().abs() < s) && (v.z().abs() < s)
}

pub fn reflect(v: &Vec3<f64>, n: &Vec3<f64>) -> Vec3<f64> {
    *v - (*n * (2.0*v.dot(n)))
}

pub fn refract(uv: &Vec3<f64>, n: &Vec3<f64>, etai_over_etat: f64) -> Vec3<f64> {
    let cos_theta = f64::min(uv.reverse_dir().dot(n), 1.0);
    let r_out_perp = (*uv + (*n * cos_theta)) * etai_over_etat;
    let r_out_parallel = *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();

    r_out_perp + r_out_parallel
}
