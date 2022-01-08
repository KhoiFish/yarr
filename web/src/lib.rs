#![cfg(target_family = "wasm")]

use owr::examples::scene_select;
use owr::log_print;
use owr::types::*;
use owr::camera;
use owr::hittable;
use owr::utils as owr_utils;
use owr::bvh;

use wasm_bindgen::{prelude::*, Clamped};
use std::sync::Arc;
use std::collections::HashMap;
use reqwest;
use std::path::{Path, PathBuf};

extern crate image;

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
pub struct ResourceCache {
    cache: HashMap::<PathBuf, Vec<u8>>,
}

impl ResourceCache {
    pub fn new() -> ResourceCache {
        ResourceCache {
            cache: HashMap::<PathBuf, Vec<u8>>::default()
        }
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Option<&[u8]> {
        if self.cache.contains_key(path.as_ref()) {
            Some(self.cache.get(path.as_ref()).unwrap())
        } else {
            None
        }
    }

    pub fn insert(&mut self, path: impl AsRef<Path>, bytes: Vec<u8>) {
        self.cache.insert(path.as_ref().to_path_buf(), bytes);
    }

    async fn create_and_load(ref_paths: &[impl AsRef<Path>]) -> ResourceCache {
        let paths : Vec<PathBuf> = ref_paths.iter().map(|p| p.as_ref().to_path_buf()).collect();
        let mut resource_cache = ResourceCache::new();
        for path in paths.iter() {
            let url = reqwest::Url::parse(path.to_str().unwrap()).unwrap_or_else(|_| {
                let u = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .url()
                    .unwrap();
                let p = if !u.ends_with("/") {
                    std::path::PathBuf::from(u).parent().unwrap().join(path)
                } else {
                    std::path::PathBuf::from(u.clone()).join(path)
                };
                reqwest::Url::parse(p.to_str().unwrap()).unwrap()
            });
            let data = reqwest::get(url).await.unwrap().bytes().await.unwrap();
            resource_cache.insert(path.clone(), (*data).to_vec());
        }
        
        resource_cache
    }
}

// --------------------------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct WebRaytracer {
    params: RaytracerParams,
    camera: camera::Camera,
    world: Arc<dyn hittable::Hittable>,
}

impl WebRaytracer {
    pub fn new(scene_num: u32, image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, enable_bvh: bool, image: image::RgbaImage) -> Self {
        let example_scene = scene_select(scene_num, image_width, image_height, samples_per_pixel, max_depth, image);
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
         owr::sampling::render_image(false, false, &self.params, &self.camera, &self.world).unwrap().into_raw()
    }

    #[cfg(not(feature = "parallel"))]
    pub fn multi_sample_buffer(&self, enable_average_sum: bool) -> Vec<Float> {
         owr::sampling::multi_sample_buffer(enable_average_sum, false, &self.params, &self.camera, &self.world)
    }

    // ------------------------------------------------------------------------
    // Multi-threaded

    #[cfg(feature = "parallel")]
    pub fn render_image(&self) -> Vec<u8> {
         owr::sampling::render_image(true, false, &self.params, &self.camera, &self.world).unwrap().into_raw()
    }

    #[cfg(feature = "parallel")]
    pub fn multi_sample_buffer(&self, enable_average_sum: bool) -> Vec<Float> {
        owr::sampling::multi_sample_buffer(enable_average_sum, true, &self.params, &self.camera, &self.world)
   }
}

// --------------------------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub async fn create_and_load_resource_cache() -> ResourceCache { 
    // TODO: accept array of resources to load as input parameters
    ResourceCache::create_and_load(&["earthmap.jpeg"]).await
}

#[wasm_bindgen]
pub fn create_empty_resource_cache() -> ResourceCache {
    ResourceCache::new()
}

#[wasm_bindgen]
pub fn get_resource(resource_cache: &ResourceCache, path: &str) -> Option::<Vec<u8>> {
    match resource_cache.get(path) {
        Some(array) => { Some(array.to_vec()) }
        _ => { None }
    }
}

#[wasm_bindgen]
pub fn insert_resource(resource_cache: &mut ResourceCache, path: &str, data: Vec<u8>) {
    resource_cache.insert(path, data);
}

// --------------------------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub fn create_webraytracer(resource_cache: &ResourceCache, scene_num: u32, image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, enable_bvh: bool) -> WebRaytracer {
    // TODO: Fix/remove hardcoded image paths
    let image = match resource_cache.get("earthmap.jpeg") {
        Some(data) => { 
            match image::load_from_memory(data) {
                Ok(image) => {
                    image.to_rgba8()
                }
                Err(e) => {
                    log_print!("{}", e);
                    image::RgbaImage::new(512, 512)
                }
            }
        }
        _ => { 
            image::RgbaImage::new(512, 512) 
        }
    };
    WebRaytracer::new(scene_num, image_width, image_height, samples_per_pixel, max_depth, enable_bvh, image)
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
