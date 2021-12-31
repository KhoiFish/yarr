use crate::types::*;
use crate::vec3::Vec3;
use crate::perlin::Perlin;

extern crate image;
use std::sync::Arc;

// --------------------------------------------------------------------------------------------------------------------

pub trait Texture {
    fn value(&self, u: Float, v: Float, p: &Vec3<Float>) -> Vec3<Float>;
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
    fn value(&self, _u: Float, _v: Float, _p: &Vec3<Float>) -> Vec3<Float> {
        self.color
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
    fn value(&self, u: Float, v: Float, p: &Vec3<Float>) -> Vec3<Float> {
        let sines = Float::sin(10.0*p.x()) * Float::sin(10.0*p.y()) * Float::sin(10.0*p.z());
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------

pub struct Noise {
    perlin: Perlin,
    scale: Float
}

impl Noise {
    pub fn new(scale: Float) -> Self {
        Self {
            perlin: Perlin::new(),
            scale
        }
    }

    #[allow(dead_code)]
    fn netted_look(&self, _u: Float, _v: Float, p: &Vec3<Float>) -> Vec3<Float> {
        Vec3::new(1.0, 1.0, 1.0)  * self.perlin.turb( &(*p * self.scale), 7)
    }

    fn marble_look(&self, _u: Float, _v: Float, p: &Vec3<Float>) -> Vec3<Float> {
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + Float::sin(self.scale * p.z() + 10.0 * self.perlin.turb(&p, 7)))
    }
}

impl Texture for Noise {
    fn value(&self, u: Float, v: Float, p: &Vec3<Float>) -> Vec3<Float> {
        self.marble_look(u, v, p)
    }
}

// --------------------------------------------------------------------------------------------------------------------

pub struct Image {
    image: image::RgbaImage
}

impl Image {
    pub fn new(image: image::RgbaImage) -> Self {
        Self {
            image
        }
    }
}

impl Texture for Image {
    fn value(&self, u: Float, v: Float, _p: &Vec3<Float>) -> Vec3<Float> {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Float::clamp(u, 0.0, 1.0);
        let v = 1.0 - Float::clamp(v, 0.0, 1.0);

        let mut i = (u * self.image.width() as Float) as u32;
        let mut j = (v * self.image.height() as Float) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.image.width() { i = self.image.width() - 1; }
        if j >= self.image.height() { j = self.image.height() - 1; }

        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);

        return Vec3::new(color_scale * pixel[0] as Float, color_scale * pixel[1] as Float, color_scale * pixel[2] as Float);
    }
}