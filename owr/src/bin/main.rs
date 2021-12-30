use owr::examples::*;
use owr::sampling::*;
use std::env;

// --------------------------------------------------------------------------------------------------------------------

pub fn main() {
    let args: Vec<String> = env::args().collect();

    // Select scene from commandline args
    let example_scene = match args[1].parse().unwrap() {
        0 => { first_weekend_example(576, 1024, 32, 50) }
        1 => { second_weekend_example_4dot4(576, 1024, 32, 50) }
        _ => { first_weekend_example(576, 1024, 32, 50) }
    };

    // Render and write out image
    let final_image = render_image(true, &example_scene.0, &example_scene.1, &example_scene.2);
    final_image.unwrap().save("output.png").unwrap();
}
