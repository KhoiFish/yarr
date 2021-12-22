use owr::examples::*;
use owr::types::*;
use owr::camera;
use owr::hittable;
use owr::log_print;

use wasm_bindgen::prelude::*;

// Uncomment the following if you want to try building for shared memory and atomics
//pub use wasm_bindgen_rayon::init_thread_pool;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name);
    log_print!("{}", name);
}

#[wasm_bindgen]
pub struct WebRaytracer {
    params: RaytracerParams,
    camera: camera::Camera,
    world: hittable::HittableList
}

#[wasm_bindgen]
#[derive(Copy, Clone, Default)]
pub struct WebColor {
    r : u8,
    g : u8,
    b : u8
}

#[wasm_bindgen]
impl WebRaytracer {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebRaytracer, JsValue> {
        // Set panic hook
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        // Default stuff
        let params = example_params(); 
        let camera = example_camera(params.aspect_ratio);
        let world = random_scene();

        // Return 
        Ok(WebRaytracer {
            params,
            camera,
            world
        })
    }

    #[wasm_bindgen]
    pub fn sample(&self, x: u32, y: u32) -> WebColor {
        let color = owr::sampling::multi_sample(x, y, &self.params, &self.camera, &self.world);
        let mut ret_color = WebColor::default();
        owr::color::get_color_components(&color, &mut ret_color.r, &mut ret_color.g, &mut ret_color.b);

        ret_color
    }

    // #[wasm_bindgen]
    // pub fn multi_threaded_render(&self) {
    //     owr::sampling::render_multisample_image(&self.params, &self.camera, &self.world);
    // }
}

#[wasm_bindgen]
impl WebColor {
    #[wasm_bindgen]
    pub fn r(&self) -> u8 {
        self.r
    }

    #[wasm_bindgen]
    pub fn g(&self) -> u8 {
        self.g
    }

    #[wasm_bindgen]
    pub fn b(&self) -> u8 {
        self.b
    }
}
