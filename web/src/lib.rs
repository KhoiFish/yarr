use owr::examples::*;
use owr::types::*;
use owr::camera;
use owr::hittable;
use owr::log_print;

use wasm_bindgen::{prelude::*, Clamped};

// --------------------------------------------------------------------------------------------------------------------

#[cfg(feature = "parallel")]
use rayon::prelude::*;

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
    alert(name);
    log_print!("{}", name);
}

// --------------------------------------------------------------------------------------------------------------------

struct WebRaytracer {
    params: RaytracerParams,
    camera: camera::Camera,
    world: hittable::HittableList
}

impl WebRaytracer {
    pub fn new() -> Self {
        // Default stuff
        let params = example_params(); 
        let camera = example_camera(params.aspect_ratio);
        let world = random_scene();

        Self {
            params,
            camera,
            world
        }
    }

    // pub fn render_image(&self, x: u32, y: u32) -> Vec<u8> {
    //     let color = owr::sampling::multi_sample(x, y, &self.params, &self.camera, &self.world);
    //     let mut ret_color = WebColor::default();
    //     owr::color::get_color_components(&color, &mut ret_color.r, &mut ret_color.g, &mut ret_color.b);

    //     ret_color
    // }

    #[cfg(feature = "parallel")]
    pub fn render_image(&self) -> Vec<u8> {
         owr::sampling::render_image_parallel(&self.params, &self.camera, &self.world)
    }

    #[cfg(not(feature = "parallel"))]
    pub fn render_image(&self) -> Vec<u8> {
         owr::sampling::render_image(&self.params, &self.camera, &self.world)
    }
}

// --------------------------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub fn generate(_width: u32, _height: u32, _max_iterations: u32) -> Clamped<Vec<u8>> {
    let raytracer = WebRaytracer::new();
    Clamped(
        raytracer.render_image()
    )
}