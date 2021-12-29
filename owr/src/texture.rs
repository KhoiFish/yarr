use crate::types::*;
use crate::vec3::Vec3;

use std::sync::Arc;

// --------------------------------------------------------------------------------------------------------------------

pub trait Texture {
    fn value(&self, u: Float, v: Float, p: &Vec3<Float>) -> &Vec3<Float>;
}

// --------------------------------------------------------------------------------------------------------------------

pub struct SolidColor {
    color: Vec3<Float>
}

impl SolidColor {
    pub fn new(color: &Vec3<Float>) -> Self {
        Self {
            color: *color
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: Float, _v: Float, _p: &Vec3<Float>) -> &Vec3<Float> {
        &self.color
    }
}

// --------------------------------------------------------------------------------------------------------------------

pub struct Checker {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>
}

impl Checker {
    pub fn new(odd: &Arc<dyn Texture>, even: &Arc<dyn Texture>) -> Self {
        Self {
            odd: odd.clone(),
            even: even.clone()
        }
    }

    pub fn new_from_colors(odd: &Vec3<Float>, even: &Vec3<Float>) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(&odd)),
            even: Arc::new(SolidColor::new(&even))
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: Float, v: Float, p: &Vec3<Float>) -> &Vec3<Float> {
        let sines = Float::sin(10.0*p.x()) * Float::sin(10.0*p.y()) * Float::sin(10.0*p.z());
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}
