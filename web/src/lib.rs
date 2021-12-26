use owr::examples::*;
use owr::types::*;
use owr::camera;
use owr::hittable;
use owr::log_print;

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

struct WebRaytracer {
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
        let world = random_scene();

        Self {
            params,
            camera,
            world
        }
    }

    // ------------------------------------------------------------------------
    // Single-threaded

    #[cfg(not(feature = "parallel"))]
    pub fn render_image(&self) -> Vec<u8> {
         owr::sampling::render_image(false, &self.params, &self.camera, &self.world)
    }

    #[cfg(feature = "parallel")]
    pub fn render_image(&self) -> Vec<u8> {
         owr::sampling::render_image(true, &self.params, &self.camera, &self.world)
    }
}

// --------------------------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub fn render_image(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> Clamped<Vec<u8>> {
    let raytracer = WebRaytracer::new(image_width, image_height, samples_per_pixel, max_depth);
    Clamped(
        raytracer.render_image()
    )
}
