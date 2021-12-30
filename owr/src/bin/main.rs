use owr::examples::*;
use owr::log_print;
use owr::sampling::*;
use owr::bvh;
use std::env;
use std::sync::Arc;

// --------------------------------------------------------------------------------------------------------------------

pub fn main() {
    let args: Vec<String> = env::args().collect();

    // Select scene from commandline args
    let scene_num = if args.len() > 1 { args[1].parse().unwrap() } else { 0 };
    let example_scene = match scene_num {
        0 => { first_weekend_example(576, 1024, 32, 50) }
        1 => { second_weekend_example_4dot4(576, 1024, 32, 50) }
        _ => { first_weekend_example(576, 1024, 32, 50) }
    };

    // Build bvh?
    let build_bvh = if args.len() > 2 { args[2].parse().unwrap() } else { false };
    log_print!("Building bvh? {}\n", build_bvh);

    // Build bvh if set
    let hittables = match build_bvh {
        true => { bvh::BvhNode::build_bvh(&example_scene.2, 0.0, 0.0) },
        _ => { Arc::new(example_scene.2) }
    };

    // Render and write out image
    log_print!("Rendering scene {}\n", scene_num);
    let final_image = render_image(true, &example_scene.0, &example_scene.1, &hittables);
    final_image.unwrap().save("output.png").unwrap();
    log_print!("Done.\n");
}
