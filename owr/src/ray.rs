use crate::vec3::{Vec3};
use std::ops::{Add, Mul};

// --------------------------------------------------------------------------------------------------------------------

pub struct Ray<T> {
    pub orig: Vec3<T>,
    pub dir: Vec3<T>,
    pub time: T
}

impl<T> Ray<T> {
    pub fn at(&self, t : T) -> Vec3<T> where T: Copy + Mul<T, Output = T> + Add<T, Output = T> {
        self.orig + self.dir * t
    }
}
