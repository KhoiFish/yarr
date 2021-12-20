pub mod utils;

use owr::examples::*;
use wasm_bindgen::{prelude::*, convert::{FromWasmAbi, IntoWasmAbi}};
use owr::types::*;
use std::rc::Rc;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Raytracer on the web!");
}

#[wasm_bindgen]
pub fn test_sowr() {
    let params = example_params(); 
    let camera = example_camera(params.aspect_ratio);
    let world = random_scene();

    run_and_print_ppm(&params, &camera, &world);
}
