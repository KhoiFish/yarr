#![allow(dead_code)]
use vecmath;
use std::ops::{Add, Sub, Mul, Div};
use crate::types::*;

// --------------------------------------------------------------------------------------------------------------------
// Vec3 class definition & implementation

#[derive(Copy, Clone, Default)]
pub struct Vec3<T> {
    data : vecmath::Vector3<T>
}

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { data : [x, y, z] }
    }

    pub fn x(&self) -> T {
        self.data[0]
    }

    pub fn y(&self) -> T {
        self.data[1]
    }

    pub fn z(&self) -> T {
        self.data[2]
    }

    pub fn dot(&self, other: &Self) -> T where T: Copy + Add<T, Output = T> + Mul<T, Output = T> {
        vecmath::vec3_dot(self.data, other.data)
    }

    pub fn cross(&self, other: &Self) -> Self where T: Copy  + Add<T, Output = T>  + Sub<T, Output = T> + Mul<T, Output = T> {
        Self { data: vecmath::vec3_cross(self.data, other.data) }
    }

    pub fn length(&self) -> T where T: Copy + float::Sqrt + Add<T, Output = T> + Mul<T, Output = T> {
        vecmath::vec3_len(self.data)
    }

    pub fn length_squared(&self) -> T where T: Copy + Add<T, Output = T> + Mul<T, Output = T> {
        vecmath::vec3_square_len(self.data)
    }

    pub fn unit_vector(&self) -> Self where T: Copy + float::One + float::Sqrt + Copy + Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> {
        Self { data: vecmath::vec3_normalized(self.data) }
    }

    pub fn to_vec4(&self, w: T) -> [T; 4] {
        [self.x(), self.y(), self.z(), w]
    }
}

impl Vec3<Float> {
    pub fn reverse_dir(&self) -> Self {
        *self * -1.0
    }
}

// --------------------------------------------------------------------------------------------------------------------
// Operator overloads

impl<T> Add for Vec3<T> where T: Copy + Add<T, Output = T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: vecmath::vec3_add(self.data, other.data)
        }
    }
}

impl<T> Sub for Vec3<T> where T: Copy + Sub<T, Output = T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            data: vecmath::vec3_sub(self.data, other.data)
        }
    }
}
impl<T> Mul for Vec3<T> where T: Copy + Mul<T, Output = T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: vecmath::vec3_mul(self.data, other.data)
        }
    }
}

impl<T> Mul<T> for Vec3<T> where T: Copy + Mul<T, Output = T> {
    type Output = Self;

    fn mul(self, t: T) -> Self {
        Self {
            data: vecmath::vec3_scale(self.data, t)
        }
    }
}
