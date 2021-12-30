use owr::examples::*;

// --------------------------------------------------------------------------------------------------------------------

pub fn main() {
    let example_scene = first_weekend_example(576, 1024, 32, 50);
    run_and_print_ppm(&example_scene.0, &example_scene.1, &example_scene.2);
}
