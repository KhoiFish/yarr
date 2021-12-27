use owr::examples::*;
use owr::types::*;
use owr::camera;
use owr::hittable;
use owr::utils as owr_utils;
use owr::log_print;
use owr::vec3::Vec3;

use wasm_bindgen::{prelude::*, Clamped};

// --------------------------------------------------------------------------------------------------------------------

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

// --------------------------------------------------------------------------------------------------------------------

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

// --------------------------------------------------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn wasm_alert(name: &str) {
    //alert(name);
    log_print!("{}", name);
}

// --------------------------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct WebRaytracer {
    params: RaytracerParams,
    camera: camera::Camera,
    world: hittable::HittableList
}

impl WebRaytracer {
    pub fn new(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> Self {
        let params = RaytracerParams {
            aspect_ratio: (image_width as Float) / (image_height as Float),
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        };

        let camera = example_camera(params.aspect_ratio);
        let world = first_weekend_scene();

        Self {
            params,
            camera,
            world
        }
    }

    pub fn multi_sample_point(&self, enable_average_sum: bool, x: u32, y: u32) -> Vec<Float> {
        owr::sampling::multi_sample(enable_average_sum, x, y, &self.params, &self.camera, &self.world).to_vec4(1.0).to_vec()
    }

    // ------------------------------------------------------------------------
    // Single-threaded

    #[cfg(not(feature = "parallel"))]
    pub fn render_image(&self) -> Vec<u8> {
         owr::sampling::render_image(false, &self.params, &self.camera, &self.world)
    }

    #[cfg(not(feature = "parallel"))]
    pub fn multi_sample_image(&self, enable_average_sum: bool) -> Vec<Float> {
         owr::sampling::multi_sample_image(enable_average_sum, false, &self.params, &self.camera, &self.world)
    }

    // ------------------------------------------------------------------------
    // Multi-threaded

    #[cfg(feature = "parallel")]
    pub fn render_image(&self) -> Vec<u8> {
         owr::sampling::render_image(true, &self.params, &self.camera, &self.world)
    }

    #[cfg(feature = "parallel")]
    pub fn multi_sample_image(&self, enable_average_sum: bool) -> Vec<Float> {
        owr::sampling::multi_sample_image(enable_average_sum, true, &self.params, &self.camera, &self.world)
   }
}

// --------------------------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub fn create_webraytracer(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> WebRaytracer {
    WebRaytracer::new(image_width, image_height, samples_per_pixel, max_depth)
}

#[wasm_bindgen]
pub fn seed_rand(seed: u32) {
    owr_utils::seed_rand(seed)
}

#[wasm_bindgen]
pub fn render_image(raytracer: &WebRaytracer) -> Clamped<Vec<u8>> {
    Clamped(raytracer.render_image())
}

#[wasm_bindgen]
pub fn multi_sample_image(raytracer: &WebRaytracer, enable_average_sum: bool) -> Vec<Float> {
    raytracer.multi_sample_image(enable_average_sum)
}

#[wasm_bindgen]
pub fn multi_sample_point(raytracer: &WebRaytracer, enable_average_sum: bool, x: u32, y: u32) -> Vec<Float> {
    raytracer.multi_sample_point(enable_average_sum, x, y)
}
