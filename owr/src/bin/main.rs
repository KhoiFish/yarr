use owr::examples::*;

// --------------------------------------------------------------------------------------------------------------------

pub fn main() {
    let params = example_params(); 
    let camera = example_camera(params.aspect_ratio);
    let world = first_weekend_scene();

    run_and_print_ppm(&params, &camera, &world);
}
