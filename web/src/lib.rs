#![cfg(target_family = "wasm")]

use owr::examples::scene_select;
use owr::types::*;
use owr::camera;
use owr::hittable;
use owr::utils as owr_utils;
use owr::log_print;
use owr::bvh;

use wasm_bindgen::{prelude::*, Clamped};
use std::sync::Arc;

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
    world: Arc<dyn hittable::Hittable>,
}

impl WebRaytracer {
    pub fn new(scene_num: u32, image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, enable_bvh: bool) -> Self {
        let example_scene = scene_select(scene_num, image_width, image_height, samples_per_pixel, max_depth);
        Self {
            params: example_scene.0,
            camera: example_scene.1,
            world: if enable_bvh { bvh::BvhNode::build_bvh(&example_scene.2, 0.0, 1.0) } else { Arc::new(example_scene.2) }
        }
    }

    pub fn multi_sample_point(&self, x: u32, y: u32) -> u32 {
        let sample = owr::color::vec3_to_u32(&owr::sampling::multi_sample(true, x, y, &self.params, &self.camera, &self.world), 1.0);
        sample
    }

    // ------------------------------------------------------------------------
    // Single-threaded

    #[cfg(not(feature = "parallel"))]
    pub fn render_image(&self) -> Vec<u8> {
         owr::sampling::render_image(false, &self.params, &self.camera, &self.world).unwrap().into_raw()
    }

    #[cfg(not(feature = "parallel"))]
    pub fn multi_sample_buffer(&self, enable_average_sum: bool) -> Vec<Float> {
         owr::sampling::multi_sample_buffer(enable_average_sum, false, &self.params, &self.camera, &self.world)
    }

    // ------------------------------------------------------------------------
    // Multi-threaded

    #[cfg(feature = "parallel")]
    pub fn render_image(&self) -> Vec<u8> {
         owr::sampling::render_image(true, &self.params, &self.camera, &self.world).unwrap().into_raw()
    }

    #[cfg(feature = "parallel")]
    pub fn multi_sample_buffer(&self, enable_average_sum: bool) -> Vec<Float> {
        owr::sampling::multi_sample_buffer(enable_average_sum, true, &self.params, &self.camera, &self.world)
   }
}

// --------------------------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub fn create_webraytracer(scene_num: u32, image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, enable_bvh: bool) -> WebRaytracer {
    WebRaytracer::new(scene_num, image_width, image_height, samples_per_pixel, max_depth, enable_bvh)
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
pub fn multi_sample_buffer(raytracer: &WebRaytracer, enable_average_sum: bool) -> Vec<Float> {
    raytracer.multi_sample_buffer(enable_average_sum)
}

#[wasm_bindgen]
pub fn multi_sample_point(raytracer: &WebRaytracer, x: u32, y: u32) -> u32 {
    raytracer.multi_sample_point(x, y)
}
