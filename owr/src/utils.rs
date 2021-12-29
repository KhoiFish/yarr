#![allow(dead_code)]
use crate::vec3::{Vec3};
use crate::types::*;
use std::cell::RefCell;

// --------------------------------------------------------------------------------------------------------------------

thread_local! {
    static NEXT_RAND: RefCell<u32> = RefCell::new(1);
    static DETERMINISTIC_NEXT_RAND: RefCell<u32> = RefCell::new(1);
}

fn next_rand_u32(tls_int: &RefCell<u32>) -> u32 {
    let mut next = tls_int.borrow_mut();
    *next = *next * 1103515245 + 12345;

    (*next/65536) % 32768
}

pub fn seed_rand(seed: u32) {
    NEXT_RAND.with(|next_rand| {
        let mut next = next_rand.borrow_mut();
        *next = seed;
    })
}

pub fn rand_u32() -> u32 {
    // rand_value
    NEXT_RAND.with(|next_rand| {
        return next_rand_u32(next_rand);
    })
}

pub fn det_rand_u32() -> u32 {
    DETERMINISTIC_NEXT_RAND.with(|next_rand| {
        return next_rand_u32(next_rand);
    })
}

pub fn det_random_float() -> Float {
    (det_rand_u32() as Float) / (32768.0)
}

// --------------------------------------------------------------------------------------------------------------------
// Non-wasm targets (Windows, Linux, MacOS, etc.)

#[cfg(not(target_family = "wasm"))]
#[macro_export]
macro_rules! log_print {
    ($($arg:tt)*) => { print!($($arg)*) };
}

#[cfg(not(target_family = "wasm"))]
use fastrand;

#[cfg(not(target_family = "wasm"))]
pub fn random_float() -> Float {
    fastrand::f64() as Float
}

// --------------------------------------------------------------------------------------------------------------------
// WASM target

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
pub fn random_float() -> Float {
    (rand_u32() as Float) / (32768.0)
}

// --------------------------------------------------------------------------------------------------------------------

pub fn random_range(min: Float, max: Float) -> Float {
    min + (random_float() * (max - min))
}

pub fn random_vec3() -> Vec3<Float> {
    Vec3::<Float>::new(random_float(), random_float(), random_float())
}

pub fn random_range_vec3(min: Float, max: Float) -> Vec3<Float> {
    Vec3::<Float>::new(random_range(min, max), random_range(min, max), random_range(min, max))
}

pub fn random_unitdisk_vec3() -> Vec3<Float> {
    loop {
        let p = Vec3::<Float>::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3<Float> {
    loop {
        let p = random_range_vec3(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub fn random_in_hemisphere(normal: &Vec3<Float>) -> Vec3<Float> {
    let unit_sphere_vec = random_in_unit_sphere();
    if unit_sphere_vec.dot(&normal) > 0.0 {
        return unit_sphere_vec;
    } else {
        return unit_sphere_vec.reverse_dir();
    }
}

pub fn random_unit_vec3() -> Vec3<Float> {
    random_in_unit_sphere().unit_vector()
}

// --------------------------------------------------------------------------------------------------------------------

pub fn det_random_range(min: Float, max: Float) -> Float {
    min + (det_random_float() * (max - min))
}

pub fn det_random_vec3() -> Vec3<Float> {
    Vec3::<Float>::new(det_random_float(), det_random_float(), det_random_float())
}

pub fn det_random_range_vec3(min: Float, max: Float) -> Vec3<Float> {
    Vec3::<Float>::new(det_random_range(min, max), det_random_range(min, max), det_random_range(min, max))
}

// --------------------------------------------------------------------------------------------------------------------

pub fn near_zero(v: &Vec3<Float>) -> bool {
    let s = 1e-18;

    (v.x().abs() < s) && (v.y().abs() < s) && (v.z().abs() < s)
}

pub fn reflect(v: &Vec3<Float>, n: &Vec3<Float>) -> Vec3<Float> {
    *v - (*n * (2.0*v.dot(n)))
}

pub fn refract(uv: &Vec3<Float>, n: &Vec3<Float>, etai_over_etat: Float) -> Vec3<Float> {
    let cos_theta = Float::min(uv.reverse_dir().dot(n), 1.0);
    let r_out_perp = (*uv + (*n * cos_theta)) * etai_over_etat;
    let r_out_parallel = *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();

    r_out_perp + r_out_parallel
}

pub fn get_sphere_uv(p: &Vec3<Float>) -> (Float, Float) {
    let pi = std::f32::consts::PI as Float;
    let theta = Float::acos(-p.y());
    let phi = Float::atan2(-p.z(), p.x()) + pi;
    let u = phi / (2.0*pi);
    let v = theta / pi;

    (u, v)
}