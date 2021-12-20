use wasm_bindgen::prelude::*;
use owr::examples::*;
use owr::types::*;
use owr::camera;
use owr::hittable;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Raytracer on the web!");
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
    pub fn sample_ray(&self, image_x: u32, image_y: u32) -> WebColor {
        let color = owr::sample_rays(image_x, image_y, &self.params, &self.camera, &self.world);
        let mut ret_color = WebColor::default();
        owr::color::get_color_components(&color, &mut ret_color.r, &mut ret_color.g, &mut ret_color.b);

        ret_color
    }
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
