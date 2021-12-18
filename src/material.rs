use crate::ray::{Ray};
use crate::hittable::{HitRecord};
use crate::color::{Color64};
use crate::utils;

// --------------------------------------------------------------------------------------------------------------------
// Scatter result

pub struct ScatterResult {
    pub scattered: Ray<f64>,
    pub attenuation: Color64
}

// --------------------------------------------------------------------------------------------------------------------
// Material trait

pub trait Material {
    fn scatter(&self, r_in : &Ray<f64>, hit: &HitRecord) -> Option<ScatterResult>;
}

// --------------------------------------------------------------------------------------------------------------------
// Lambert

pub struct Lambertian {
    pub albedo: Color64
}

impl Material for Lambertian {
    fn scatter(&self, _r_in : &Ray<f64>, hit: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = hit.normal + utils::random_unit_vec3();
        if utils::near_zero(&scatter_direction) {
            scatter_direction = hit.normal;
        }

        Some(ScatterResult {
            scattered: Ray { orig: hit.point, dir: scatter_direction },
            attenuation: self.albedo
        })
    }
}

// --------------------------------------------------------------------------------------------------------------------
// Metal

pub struct Metal {
    pub albedo: Color64,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, r_in : &Ray<f64>, hit: &HitRecord) -> Option<ScatterResult> { 
        let reflected = utils::reflect(&r_in.dir.unit_vector(), &hit.normal);
        let mut return_option = Option::None;
        if reflected.dot(&hit.normal) > 0.0 {
            return_option = Some(ScatterResult {
                scattered: Ray { orig: hit.point, dir: reflected + (utils::random_in_unit_sphere() * self.fuzz)},
                attenuation: self.albedo
            })
        }
    
        return_option
    }
}

// --------------------------------------------------------------------------------------------------------------------
// Dielectric

pub struct Dielectric {
    pub index_of_refraction: f64
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0*r0;
        
        r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in : &Ray<f64>, hit: &HitRecord) -> Option<ScatterResult> { 
        let refraction_ratio = if hit.front_facing { 1.0/self.index_of_refraction } else { self.index_of_refraction };

        let unit_direction = r_in.dir.unit_vector();
        let cos_theta = f64::min(unit_direction.reverse_dir().dot(&hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0 || (Dielectric::reflectance(cos_theta, refraction_ratio) > utils::random_float());
        let direction;
        if cannot_refract {
            direction = utils::reflect(&unit_direction, &hit.normal);
        } else {
            direction = utils::refract(&unit_direction, &hit.normal, refraction_ratio);
        }

        Some(ScatterResult {
            scattered: Ray { orig: hit.point, dir: direction },
            attenuation: Color64::new(1.0, 1.0, 1.0)
        })
    }
}
