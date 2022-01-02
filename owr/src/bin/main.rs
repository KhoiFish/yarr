use owr::examples::*;
use owr::log_print;
use owr::sampling::*;
use owr::bvh;
use std::env;
use std::sync::Arc;

extern crate image;
use std::time::{Instant};

// --------------------------------------------------------------------------------------------------------------------

pub fn main() {
    let args: Vec<String> = env::args().collect();

    // Some default parameters. TODO: make them configurable from the command-line
    let image_width = 512;
    let image_height = 512;
    let samples_per_pixel = 32;
    let max_depth = 50;
    let output_filename = "output.png";

    // Select scene from commandline args
    let scene_num = if args.len() > 1 { args[1].parse().unwrap() } else { 0 };
    let example_scene = scene_select(scene_num, image_width, image_height, samples_per_pixel, max_depth, image::open("./earthmap.jpeg").unwrap().to_rgba8());

    // Build bvh?
    let build_bvh = if args.len() > 2 { args[2].parse().unwrap() } else { true };
    log_print!("Build bvh: {}\n", build_bvh);

    // Build bvh if set
    let hittables = match build_bvh {
        true => { bvh::BvhNode::build_bvh(&example_scene.2, 0.0, 0.0) },
        _ => { Arc::new(example_scene.2) }
    };

    // Render and write out image
    log_print!("Rendering scene {}...\n", scene_num);
    let now = Instant::now();
    let final_image = render_image(true, &example_scene.0, &example_scene.1, &hittables);
    final_image.unwrap().save(output_filename).unwrap();
    log_print!("Completed in {}s. Results written to {}\n", now.elapsed().as_secs_f32(), output_filename);
}
